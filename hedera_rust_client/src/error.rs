use config::ConfigError;
use ed25519_dalek::ed25519;
use hex::FromHexError;
use prost::{DecodeError, EncodeError};
use std::convert::Infallible;
use std::num::{ParseFloatError, ParseIntError, TryFromIntError};
use thiserror::Error;

use crate::client::ClientBuilderError;
use crate::crypto::Asn1Error;
use crate::Hbar;
use crate::Status;
use crate::TransactionId;
use crate::TransactionReceipt;

#[derive(Error, Debug)]
pub enum HederaError {
    #[error("bytes array must be of size 32: `{0}`")]
    BytesArrayLength(usize),

    #[error("Address `{0}` is required to be 40 characters")]
    ContractAddressLength(String),

    #[error("Failed hedera pre check with status: {0:?}")]
    FailedPreCheck(Status),

    #[error("invalid node account id set")]
    InvalidNodeAccountId,

    #[error("invalid node address: {0}")]
    InvalidNodeAddress(String),

    #[error("missing node cert hash")]
    MissingNodeCertHash,

    #[error("invalid node type returned")]
    InvalidNodeType,

    #[error("invalid response type returned")]
    InvalidResponseType,

    #[error("unable to deserialize node address book")]
    NodeAddressBookDeserialize,

    #[error("no node returned")]
    NoNode,

    #[error("no response header")]
    NoResponseHeader,

    #[error("return unexpected proto type")]
    UnexpectedProtoType,

    #[error("return unexpected proto response type: {0}")]
    UnexpectedProtoResponseType(String),

    #[error("receipt for transaction {transaction_id:?} contained error status {status:?} in receipt {transaction_receipt:?}")]
    ReceiptStatusError {
        transaction_receipt: TransactionReceipt,
        status: Status,
        transaction_id: TransactionId,
    },

    #[error("no result transactions")]
    NoResultTransactions,

    #[error("set value is of wrong type")]
    InvalidSetType,

    #[error("set transfer id not of type TransferContractId")]
    InvalidSetTransferId,

    #[error("unreacahble: function name must be non-nil at this point")]
    Unreacahble,

    #[error("ed25519: unknown public key algorithm")]
    UnknownPublicKeyAlgorithm,

    #[error("ed25519: public key length mismatch")]
    InvalidPublicKeyLength,

    #[error("ed25519: PKCS#8 wrapping contained private key with unknown algorithm")]
    PkcsUnknownPublicKeyAlgorithm,

    #[error("unable to serialize key")]
    UnableToSerializeKey,

    #[error("no threshold value set")]
    NoThresholdValue,

    #[error("no inner key set")]
    NoInnerKey,

    #[error("account id contains alias key, unable to validate")]
    UnableToValidateAccountAlias,

    #[error("account id contains alias key, unable to convert")]
    UnableToConvertAccountAlias,

    #[error("unknown status from code: {0}")]
    UnknownHederaStatusCode(i32),

    #[error("unable to seriaiize signed transaction to bytes: {0}")]
    UnableToSerializeTransaction(prost::EncodeError),

    #[error("unable to deseriaiize signed transaction from bytes: {0}")]
    UnableToDeserializeTransaction(prost::DecodeError),

    #[error("unsupported conversion for transaction body type")]
    UnsupportedTransactionBodyType,

    #[error("unable to generate mnemonic")]
    InvalidMnemonic,

    #[error("no {0} in proto")]
    MissingInProto(String),

    #[error("no {0} set")]
    ValueNotSet(String),

    #[error("no account id returned")]
    NoAccountId,

    #[error("no file provided")]
    NoFileProvided,

    #[error("Solidity address must be 20 bytes")]
    InvalidSolidityAddress,

    #[error("shard out of 32-bit range")]
    InvalidShardNum,

    #[error("node checksums did not match")]
    InvalidChecksum,

    #[error("expected format [account@seconds.nanos[?scheduled]")]
    InvalidTransactionIdFormat,

    #[error("exceeded max attemps {0:?}")]
    MaxAttempsExceeded(u8),

    #[error("max chunks exceeded, required chunks: {0}, max chunks: {1}")]
    MaxChunksExceeded(usize, usize),

    #[error("max query payment exceeded for TransactionRecordQuery: query cost {0}, max query payment {1}")]
    MaxQueryPaymentExceeded(Hbar, Hbar),

    #[error("memo cannot exceed 100 bytes: `{0}`")]
    MemoLength(String),

    #[error("no response")]
    NoResponse,

    #[error("no Query response")]
    NoQueryResponse,

    #[error("no Transaction response")]
    NoTransactionResponse,

    #[error("proto failed with code: {0:?}")]
    ProtoClientFailed(tonic::Code),

    #[error("request not of type Query")]
    QueryRequestTypeError,

    #[error(
        "The underlying transaction for a scheduled transaction cannot have node account ids set"
    )]
    ScheduledTransactionNodeAccountIdsSet,

    #[error("request not of type Transaction")]
    TransactionRequestTypeError,

    #[error("transaction must be frozen before calculating the hash to be stable")]
    TransactionHashMustBeFrozen,

    #[error(
        "transaction is immutable; it has at least one signature or has been explicitly frozen"
    )]
    TransactionImmutable,

    #[error("unable to seriailize transaction body to bytes: {0}")]
    TransactionSerializationError(EncodeError),

    #[error("unable to deseriailize transaction from bytes: {0}")]
    TransactionDeserializationError(DecodeError),

    #[error("transaction did not have exactly one node ID set")]
    TransactionNodeAccountIdNotSet,

    #[error("`client` must be provided or both `node_id` and `transaction_id` must be set")]
    TransactionNodeAccountIdOrClientNotSet,

    #[error("sign with failed, transaction must have been frozen before signing")]
    TransactionSignWithFailed,

    #[error(
        "transaction must have been frozen before getting the transaction ID, try calling `freeze`"
    )]
    TransactionIdNotFrozen,

    #[error("transaction body data must be set")]
    TransactionBodyNotSet,

    #[error("expected format [shard]:[realm]:[num]")]
    UnknownIdFormat,

    #[error("expected format [shard]:[realm]:[num]-[checksum]")]
    UnknownIdChecksumFormat,

    #[error("Unknown network type `{0}`")]
    UnknownNetworkType(String),

    #[error("No network provided")]
    NoNetworkNodes,

    #[error("expected format [seconds].[nanos]")]
    UnknownTimestampFormat,

    #[error("Only ed25519 public keys are currently supported")]
    UnsupportedKeyType,

    #[error(transparent)]
    ClientBuilder(#[from] ClientBuilderError),

    #[error(transparent)]
    Asn1(#[from] Asn1Error),

    #[error(transparent)]
    Crypto(#[from] ed25519::Error),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Float(#[from] ParseFloatError),

    #[error(transparent)]
    Int(#[from] ParseIntError),

    #[error(transparent)]
    Hex(#[from] FromHexError),

    #[error(transparent)]
    Http(#[from] http::Error),

    #[error(transparent)]
    Tonic(#[from] tonic::transport::Error),

    #[error(transparent)]
    TryFromInt(#[from] TryFromIntError),

    #[error(transparent)]
    TryFrom(#[from] Infallible),

    #[error(transparent)]
    Uri(#[from] http::uri::InvalidUri),
}
