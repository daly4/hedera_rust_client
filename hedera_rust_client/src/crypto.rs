use bip39::{Language, Mnemonic, MnemonicType, Seed};
use once_cell::sync::Lazy;
use rand_chacha::{ChaCha20Core, ChaCha20Rng};
use rand_core::{OsRng, SeedableRng};
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeTupleStruct, Serializer};
#[allow(unused_imports)]
use simple_asn1::{
    der_decode, der_encode, oid, to_der, ASN1Block, ASN1Class, ASN1DecodeErr, ASN1EncodeErr,
    BigUint, FromASN1, ToASN1, OID,
};
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use thiserror::Error;

use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::transaction::Transaction;

// Types used for (de-)serializing public and secret keys from ASN.1 byte
// streams.

#[derive(Debug, Error)]
pub enum Asn1Error {
    #[error(transparent)]
    Decode(#[from] ASN1DecodeErr),

    #[error(transparent)]
    Encode(#[from] ASN1EncodeErr),

    #[error("expected `{}`; found: `{}`", expected, found)]
    UnexpectedType {
        expected: &'static str,
        found: String,
    },
}

// [https://tools.ietf.org/id/draft-ietf-curdle-pkix-01.html#rfc.section.3]
static OID_ED25519: Lazy<OID> = Lazy::new(|| oid!(1, 3, 101, 112));

// [https://www.ietf.org/rfc/rfc3280.txt]
// AlgorithmIdentifier ::= SEQUENCE {
//      algorithm               OBJECT IDENTIFIER,
//      parameters              ANY DEFINED BY algorithm OPTIONAL }

#[derive(Debug)]
struct AlgorithmIdentifier {
    algorithm: OID,
}

impl FromASN1 for AlgorithmIdentifier {
    type Error = Asn1Error;

    fn from_asn1(v: &[ASN1Block]) -> Result<(Self, &[ASN1Block]), Asn1Error> {
        let algorithm = if let Some(ASN1Block::Sequence(_, blocks)) = v.get(0) {
            if let Some(ASN1Block::ObjectIdentifier(_, id)) = blocks.get(0) {
                id
            } else {
                return Err(Asn1Error::UnexpectedType {
                    expected: "OBJECT IDENTIFIER",
                    found: format!("{:?}", blocks.get(0)),
                });
            }
        } else {
            return Err(Asn1Error::UnexpectedType {
                expected: "SEQUENCE",
                found: format!("{:?}", v.get(0)),
            });
        };

        Ok((
            Self {
                // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                algorithm: algorithm.clone(),
            },
            &v[1..],
        ))
    }
}

// [https://www.ietf.org/rfc/rfc3280.txt]
// SubjectPublicKeyInfo ::= SEQUENCE {
//      algorithm            AlgorithmIdentifier,
//      subjectPublicKey     BIT STRING }

#[derive(Debug)]
struct SubjectPublicKeyInfo {
    algorithm: AlgorithmIdentifier,
    subject_public_key: Vec<u8>,
}

impl ToASN1 for SubjectPublicKeyInfo {
    type Error = Asn1Error;

    fn to_asn1_class(&self, _c: ASN1Class) -> Result<Vec<ASN1Block>, Asn1Error> {
        Ok(vec![ASN1Block::Sequence(
            0,
            vec![
                // AlgorithmIdentifier
                ASN1Block::Sequence(
                    0,
                    vec![
                        // Algorithm
                        // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                        ASN1Block::ObjectIdentifier(0, self.algorithm.algorithm.clone()),
                    ],
                ),
                // subjectPublicKey
                ASN1Block::BitString(
                    0,
                    self.subject_public_key.len() * 8,
                    // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                    self.subject_public_key.clone(),
                ),
            ],
        )])
    }
}

impl FromASN1 for SubjectPublicKeyInfo {
    type Error = Asn1Error;
    fn from_asn1(v: &[ASN1Block]) -> Result<(Self, &[ASN1Block]), Asn1Error> {
        let (algorithm, subject_public_key) = if let Some(ASN1Block::Sequence(_, blocks)) = v.get(0)
        {
            // Parse: algorithm
            let (algorithm, blocks): (AlgorithmIdentifier, _) = FromASN1::from_asn1(blocks)?;

            // Parse: subject_public_key
            if let Some(ASN1Block::BitString(_, _, bytes)) = blocks.get(0) {
                (algorithm, bytes)
            } else {
                return Err(Asn1Error::UnexpectedType {
                    expected: "BIT STRING",
                    found: format!("{:?}", blocks.get(0)),
                });
            }
        } else {
            return Err(Asn1Error::UnexpectedType {
                expected: "SEQUENCE",
                found: format!("{:?}", v.get(0)),
            });
        };

        Ok((
            Self {
                algorithm,
                // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                subject_public_key: subject_public_key.clone(),
            },
            &v[1..],
        ))
    }
}

// [https://www.ietf.org/rfc/rfc5208.txt]
// PrivateKeyInfo ::= SEQUENCE {
//      version                   INTEGER,
//      privateKeyAlgorithm       AlgorithmIdentifier,
//      privateKey                OCTET STRING,
//      attributes           [0]  IMPLICIT Attributes OPTIONAL }

struct PrivateKeyInfo {
    algorithm: AlgorithmIdentifier,
    private_key: Vec<u8>,
}

impl ToASN1 for PrivateKeyInfo {
    type Error = Asn1Error;

    fn to_asn1_class(&self, _c: ASN1Class) -> Result<Vec<ASN1Block>, Asn1Error> {
        Ok(vec![ASN1Block::Sequence(
            0,
            vec![
                // Version
                ASN1Block::Integer(0, 0.into()),
                // AlgorithmIdentifier
                ASN1Block::Sequence(
                    0,
                    vec![
                        // Algorithm
                        // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                        ASN1Block::ObjectIdentifier(0, self.algorithm.algorithm.clone()),
                    ],
                ),
                // PrivateKey
                ASN1Block::OctetString(
                    0,
                    // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                    to_der(&ASN1Block::OctetString(0, self.private_key.clone()))?,
                ),
            ],
        )])
    }
}

impl FromASN1 for PrivateKeyInfo {
    type Error = Asn1Error;

    fn from_asn1(v: &[ASN1Block]) -> Result<(Self, &[ASN1Block]), Asn1Error> {
        let (algorithm, key) = if let Some(ASN1Block::Sequence(_, blocks)) = v.get(0) {
            // Parse: algorithm
            let (algorithm, blocks): (AlgorithmIdentifier, _) = FromASN1::from_asn1(&blocks[1..])?;

            // Parse: subject_public_key
            if let Some(ASN1Block::OctetString(_, bytes)) = blocks.get(0) {
                (algorithm, bytes)
            } else {
                return Err(Asn1Error::UnexpectedType {
                    expected: "OCTET STRING",
                    found: format!("{:?}", blocks.get(0)),
                });
            }
        } else {
            return Err(Asn1Error::UnexpectedType {
                expected: "SEQUENCE",
                found: format!("{:?}", v.get(0)),
            });
        };

        Ok((
            Self {
                // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                algorithm,
                // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                private_key: key.clone(),
            },
            &v[1..],
        ))
    }
}

/// An ed25519 public key.
#[derive(Clone, Copy)]
pub struct PublicKey(ed25519_dalek::PublicKey);

impl PublicKey {
    /// Construct a `PublicKey` from a slice of bytes.
    /// Bytes are expected to be either a raw key or encoded in ASN.1.
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, HederaError> {
        let bytes = bytes.as_ref();

        if bytes.len() == ed25519_dalek::PUBLIC_KEY_LENGTH {
            // If the buffer is exactly the length of a public key; assume that this is
            // a raw key and return it directly
            return Ok(PublicKey(ed25519_dalek::PublicKey::from_bytes(bytes)?));
        }

        let info: SubjectPublicKeyInfo = der_decode(&bytes)?;

        if info.algorithm.algorithm != *OID_ED25519 {
            return Err(HederaError::UnknownPublicKeyAlgorithm);
        }

        if info.subject_public_key.len() != ed25519_dalek::PUBLIC_KEY_LENGTH {
            return Err(HederaError::InvalidPublicKeyLength);
        }

        Ok(PublicKey(ed25519_dalek::PublicKey::from_bytes(
            &info.subject_public_key,
        )?))
    }

    pub fn from_hex_bytes(bytes: Vec<u8>) -> Result<Self, HederaError> {
        if bytes.len() == 64 {
            // This is hex-encoded
            // CryptoGetInfo returns the public key like this
            Self::from_bytes(hex::decode(&bytes)?)
        } else {
            Self::from_bytes(bytes)
        }
    }

    /// Return the `PublicKey` as raw bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; ed25519_dalek::PUBLIC_KEY_LENGTH] {
        self.0.as_bytes()
    }

    #[inline]
    pub fn as_bytes_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    /// Format a `PublicKey` as a vec of bytes in ASN.1 format.
    pub fn to_encoded_bytes(&self) -> Vec<u8> {
        der_encode(&SubjectPublicKeyInfo {
            algorithm: AlgorithmIdentifier {
                algorithm: OID_ED25519.clone(),
            },
            subject_public_key: self.as_bytes_vec(),
        })
        // NOTE: Not possible to fail. Only fail case the library has is if OIDs are
        //       given incorrectly.
        .unwrap()
    }

    /// Verify a signature on a message with this `PublicKey`.
    pub fn verify(
        &self,
        message: impl AsRef<[u8]>,
        signature: &Signature,
    ) -> Result<bool, HederaError> {
        match self.0.verify_strict(message.as_ref(), &signature.0) {
            Ok(_) => Ok(true),
            Err(error) => {
                if error.to_string() == "Verification equation was not satisfied" {
                    Ok(false)
                } else {
                    Err(error.into())
                }
            }
        }
    }

    pub fn to_signature_pair_protobuf(&self, signature: &Signature) -> services::SignaturePair {
        services::SignaturePair {
            pub_key_prefix: self.as_bytes_vec(),
            signature: Some(services::signature_pair::Signature::Ed25519(
                signature.as_bytes_vec(),
            )),
        }
    }
}

/// Construct a `PublicKey` from a hex representation of a raw or ASN.1 encoded
/// key.
impl FromStr for PublicKey {
    type Err = HederaError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, HederaError> {
        Self::from_bytes(&hex::decode(s.as_bytes())?)
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

/// Format a `PublicKey` as a hex representation of its bytes in ASN.1 format.
impl Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(&self.to_encoded_bytes()))
    }
}

impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for PublicKey {}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ts = serializer.serialize_tuple_struct("PublicKey", 1)?;
        ts.serialize_field(&self.to_string())?;
        ts.end()
    }
}
impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PublicKeyVisitor;

        impl<'de> Visitor<'de> for PublicKeyVisitor {
            type Value = PublicKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct PublicKey")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let s = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(PublicKey::from_str(s).map_err(de::Error::custom)?)
            }
        }

        deserializer.deserialize_seq(PublicKeyVisitor)
    }
}

impl TryFrom<services::Key> for PublicKey {
    type Error = HederaError;

    fn try_from(key: services::Key) -> Result<Self, HederaError> {
        let key = match key.key {
            Some(val) => val,
            None => return Err(HederaError::UnsupportedKeyType),
        };
        match key {
            services::key::Key::Ed25519(bytes) => Self::from_hex_bytes(bytes),
            _ => Err(HederaError::UnsupportedKeyType),
        }
    }
}

impl ToProto<services::key::Key> for PublicKey {
    fn to_proto(&self) -> Result<services::key::Key, HederaError> {
        Ok(services::key::Key::Ed25519(self.as_bytes_vec()))
    }
}

/// An EdDSA secret key.
pub struct PrivateKey(ed25519_dalek::SecretKey);

impl PrivateKey {
    pub fn new() -> Self {
        let mut rng = OsRng {};
        PrivateKey(ed25519_dalek::SecretKey::generate(&mut rng))
    }
    /// Generate a `PrivateKey` with a BIP-39 mnemonic using a cryptographically
    /// secure random number generator.
    ///
    /// The `password` is required with the mnemonic to reproduce the secret key.
    pub fn generate(password: &str) -> (Self, String) {
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);

        let secret = Self::generate_with_mnemonic(&mnemonic, password);

        (secret, mnemonic.into_phrase())
    }

    fn generate_with_mnemonic(mnemonic: &Mnemonic, password: &str) -> Self {
        let mut seed: [u8; 32] = Default::default();

        seed.copy_from_slice(&Seed::new(&mnemonic, password).as_bytes()[0..32]);

        let mut rng = ChaCha20Rng::from(ChaCha20Core::from_seed(seed));
        PrivateKey(ed25519_dalek::SecretKey::generate(&mut rng))
    }

    /// Construct a `PrivateKey` from a slice of bytes.
    /// Bytes are expected to be either a raw key or encoded in ASN.1.
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, HederaError> {
        let bytes = bytes.as_ref();

        if bytes.len() == ed25519_dalek::SECRET_KEY_LENGTH + ed25519_dalek::PUBLIC_KEY_LENGTH
            || bytes.len() == ed25519_dalek::SECRET_KEY_LENGTH
        {
            // If the buffer looks like a {secret}{public} byte string; just pull the secret
            // key bytes off of it
            return Ok(PrivateKey(ed25519_dalek::SecretKey::from_bytes(
                &bytes[..ed25519_dalek::SECRET_KEY_LENGTH],
            )?));
        }

        let info: PrivateKeyInfo = der_decode(&bytes)?;

        if info.algorithm.algorithm != *OID_ED25519 {
            return Err(HederaError::PkcsUnknownPublicKeyAlgorithm);
        }

        Ok(PrivateKey(ed25519_dalek::SecretKey::from_bytes(
            &info.private_key[2..],
        )?))
    }

    /// Re-construct a `PrivateKey` from the supplied mnemonic and password.
    pub fn from_mnemonic(mnemonic: &str, password: &str) -> Result<Self, HederaError> {
        let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English)
            .map_err(|_| HederaError::InvalidMnemonic)?;

        Ok(Self::generate_with_mnemonic(&mnemonic, password))
    }

    /// Return the `PrivateKey` as raw bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; ed25519_dalek::PUBLIC_KEY_LENGTH] {
        self.0.as_bytes()
    }

    #[inline]
    pub fn as_bytes_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    /// Format a `PrivateKey` as a vec of bytes in ASN.1 format.
    pub fn to_encoded_bytes(&self) -> Vec<u8> {
        der_encode(&PrivateKeyInfo {
            algorithm: AlgorithmIdentifier {
                algorithm: OID_ED25519.clone(),
            },
            private_key: self.as_bytes_vec(),
        })
        // NOTE: Not possible to fail. Only fail case the library has is if OIDs are
        //       given incorrectly.
        .unwrap()
    }

    /// Derive a `PublicKey` from this `PrivateKey`.
    #[inline]
    pub fn public(&self) -> PublicKey {
        PublicKey(ed25519_dalek::PublicKey::from(&self.0))
    }

    /// Sign a message with this `PrivateKey`.
    #[inline]
    pub fn sign(&self, message: impl AsRef<[u8]>) -> Signature {
        Signature(
            ed25519_dalek::ExpandedSecretKey::from(&self.0)
                .sign(message.as_ref(), &self.public().0),
        )
    }

    pub async fn sign_transaction(&self, transaction: &mut Transaction) -> Result<(), HederaError> {
        transaction.require_one_node_account_id()?;

        if !transaction.is_frozen() {
            transaction.freeze().await?;
        }

        let signature = self.sign(&transaction.signed_transactions[0].body_bytes);
        transaction.add_signature(self.public(), signature)?;

        Ok(())
    }
}

impl Clone for PrivateKey {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_bytes(self.0.as_bytes()).unwrap()
    }
}

/// Construct a `PrivateKey` from a hex representation of a raw or ASN.1 encoded
/// key.
impl FromStr for PrivateKey {
    type Err = crate::error::HederaError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, HederaError> {
        Self::from_bytes(&hex::decode(s.as_bytes())?)
    }
}

impl Debug for PrivateKey {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

/// Format a `PrivateKey` as a hex representation of its bytes in ASN.1 format.
impl Display for PrivateKey {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(&self.to_encoded_bytes()))
    }
}

/// An EdDSA signature.
#[derive(Debug)]
pub struct Signature(ed25519_dalek::Signature);

impl Signature {
    /// Construct a `Signature` from a slice of bytes.
    #[inline]
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, HederaError> {
        Ok(Signature(ed25519_dalek::Signature::try_from(
            bytes.as_ref(),
        )?))
    }

    /// Return the `Signature` as raw bytes.
    #[inline]
    pub fn to_bytes(&self) -> [u8; ed25519_dalek::SIGNATURE_LENGTH] {
        self.0.to_bytes()
    }

    #[inline]
    pub fn as_bytes_vec(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}

/// Construct a `Signature` from a hex representation of the signature.
impl FromStr for Signature {
    type Err = crate::error::HederaError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, HederaError> {
        Self::from_bytes(&hex::decode(s.as_bytes())?)
    }
}

/// Format a `Signature` as a hex representation of its bytes.
impl Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(&self.to_bytes()[..]))
    }
}

impl ToProto<services::Signature> for Signature {
    fn to_proto(&self) -> Result<services::Signature, HederaError> {
        Ok(services::Signature {
            signature: Some(services::signature::Signature::Ed25519(self.as_bytes_vec())),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{PrivateKey, PublicKey, Signature};

    const KEY_PUBLIC_ASN1_HEX: &str =
        "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7";

    const KEY_PUBLIC_HEX: &str = "e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7";

    const KEY_SECRET_ASN1_HEX: &str =
        "302e020100300506032b657004220420db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10";

    const KEY_SECRET_HEX: &str = "db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10\
                                  e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7";

    const MESSAGE: &str = "This is a message about the world.";
    const SIGNATURE: &str = "73bea53f31ca9c42a422ecb7516ec08d0bbd1a6bfd630ccf10ec1872454814d29f4a8011129cd007eab544af01a75f508285b591e5bed24b68f927751e49e30e";

    #[test]
    fn test_parse() {
        let public_key1: PublicKey = KEY_PUBLIC_ASN1_HEX.parse().unwrap();
        let public_key2: PublicKey = KEY_PUBLIC_HEX.parse().unwrap();

        let secret_key1: PrivateKey = KEY_SECRET_ASN1_HEX.parse().unwrap();
        let secret_key2: PrivateKey = KEY_SECRET_HEX.parse().unwrap();

        assert_eq!(public_key1, public_key2);
        assert_eq!(secret_key1.0.as_bytes(), secret_key2.0.as_bytes());
        assert_eq!(public_key1, secret_key1.public());
        assert_eq!(public_key2, secret_key2.public());
        assert_eq!(secret_key2.public(), secret_key1.public());
    }

    #[test]
    fn test_verify() {
        let key: PublicKey = KEY_PUBLIC_ASN1_HEX.parse().unwrap();
        let signature: Signature = SIGNATURE.parse().unwrap();
        let verified = key.verify(MESSAGE.as_bytes(), &signature).unwrap();

        assert!(verified);
    }

    #[test]
    fn test_sign() {
        let key: PrivateKey = KEY_SECRET_ASN1_HEX.parse().unwrap();
        let signature = key.sign(MESSAGE.as_bytes());

        assert_eq!(SIGNATURE, signature.to_string());
    }

    #[test]
    fn test_generate() {
        let (key, _mnemonic) = PrivateKey::generate("");
        let signature = key.sign(MESSAGE.as_bytes());
        let verified = key.public().verify(MESSAGE.as_bytes(), &signature).unwrap();

        assert!(verified);
    }

    #[test]
    fn test_display() {
        let public_key1: PublicKey = KEY_PUBLIC_ASN1_HEX.parse().unwrap();
        let public_key2: PublicKey = public_key1.to_string().parse().unwrap();

        let secret_key1: PrivateKey = KEY_SECRET_ASN1_HEX.parse().unwrap();
        let secret_key2: PrivateKey = secret_key1.to_string().parse().unwrap();

        assert_eq!(public_key1, public_key2);
        assert_eq!(secret_key1.as_bytes(), secret_key2.as_bytes());
    }

    #[test]
    fn test_reconstruct() {
        let (secret1, mnemonic) = PrivateKey::generate("this-is-not-a-password");
        let secret2 = PrivateKey::from_mnemonic(&mnemonic, "this-is-not-a-password").unwrap();

        assert_eq!(secret1.as_bytes(), secret2.as_bytes());
    }
}
