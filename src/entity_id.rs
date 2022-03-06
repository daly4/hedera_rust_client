use bytes::{BufMut, BytesMut};
use std::cmp::min;
use std::convert::AsMut;

use crate::client::Client;
use crate::error::HederaError;
use crate::id::IdChecksum;
use crate::proto::services::{RealmId as ProtoRealmId, ShardId as ProtoShardId};
use crate::proto::ToProto;

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

pub fn format_id(shard: &i64, realm: &i64, num: &i64) -> String {
    format!("{}.{}.{}", shard, realm, num)
}

pub fn format_id_with_checksum(
    shard: &i64,
    realm: &i64,
    num: &i64,
    checksum: &IdChecksum,
) -> String {
    format!("{}.{}.{}-{}", shard, realm, num, checksum)
}

pub fn checksum(ledger_id: &str, addr: &str) -> Result<IdChecksum, HederaError> {
    let mut answer = [0u32; 5];
    let mut digits = Vec::new();
    let mut s0 = 0i64;
    let mut s1 = 0i64;
    let mut s = 0i64;
    let mut sh = 0i64;
    let mut checksum;
    let n = addr.len() as i64;
    let p3: i64 = 26 * 26 * 26;
    let p5: i64 = 26 * 26 * 26 * 26 * 26;
    let m = 1000003i64;
    let ascii_a = 'a' as u32;
    let w = 31i64;

    let id = format!("{}{}", ledger_id, "000000000000");
    let mut h = Vec::new();

    let len = id.len();
    for i in (0..len).step_by(2) {
        h.push(
            id[i..min(i + 2, len)]
                .parse::<i64>()
                .map_err(|_| HederaError::InvalidChecksum)?,
        );
    }

    for ch in addr.chars() {
        let c = if ch == '.' {
            10i64
        } else {
            ch.to_digit(10).ok_or(HederaError::InvalidChecksum)? as i64
        };
        digits.push(c);
    }

    for i in 0..digits.len() {
        s = (w * s + digits[i]) % p3;
        if i % 2 == 0 {
            s0 = (s0 + digits[i]) % 11;
        } else {
            s1 = (s1 + digits[i]) % 11;
        }
    }

    for i in 0..h.len() {
        sh = (w * sh + h[i]) % p5;
    }

    checksum = ((((n % 5) * 11 + s0) * 11 + s1) * p3 + s + sh) % p5;
    checksum = (checksum * m) % p5;

    for i in (0..5).rev() {
        answer[i] = ascii_a + ((checksum % 26) as u32);
        checksum /= 26;
    }

    Ok(IdChecksum::try_from(answer)?)
}

pub fn validate(
    client: &Client,
    shard: &i64,
    realm: &i64,
    num: &i64,
    check: &Option<IdChecksum>,
) -> Result<(), HederaError> {
    if let Some(check) = check {
        let network = client.ledger_id();
        let expected = checksum(&network.for_checksum(), &format_id(shard, realm, num))?;
        if check != &expected {
            return Err(HederaError::InvalidChecksum);
        }
    }
    Ok(())
}

pub fn id_from_solidity_address(s: &str) -> Result<(i64, i64, i64), HederaError> {
    let bytes = hex::decode(s)?;
    if bytes.len() > 20 {
        return Err(HederaError::InvalidSolidityAddress);
    }
    let shard = i32::from_be_bytes(clone_into_array(&bytes[0..4]));
    let realm = i64::from_be_bytes(clone_into_array(&bytes[4..12]));
    let num = i64::from_be_bytes(clone_into_array(&bytes[12..20]));
    Ok((shard as i64, realm, num))
}

pub fn id_to_solidity_address(shard: i64, realm: i64, num: i64) -> Result<String, HederaError> {
    let s = i32::try_from(shard).map_err(|_| HederaError::InvalidShardNum)?;
    let mut buf = BytesMut::with_capacity(20);
    buf.put_i32(s);
    buf.put_i64(realm);
    buf.put_i64(num);
    Ok(hex::encode(&buf).to_string())
}

pub trait ValidateChecksum {
    fn validate_checksum(&self, client: &Client) -> Result<(), HederaError>;
}

pub fn validate_option_id_checksum<T: ValidateChecksum>(
    id: &Option<T>,
    client: &Client,
) -> Result<(), HederaError> {
    if let Some(id) = id {
        validate_id_checksum(id, client)?;
    }
    Ok(())
}

pub fn validate_id_checksum<T: ValidateChecksum>(
    id: &T,
    client: &Client,
) -> Result<(), HederaError> {
    id.validate_checksum(client)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShardId {
    pub shard_num: i64,
}

impl ToProto<ProtoShardId> for ShardId {
    fn to_proto(&self) -> Result<ProtoShardId, HederaError> {
        Ok(ProtoShardId {
            shard_num: self.shard_num,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RealmId {
    pub shard_num: i64,
    pub realm_num: i64,
}

impl ToProto<ProtoRealmId> for RealmId {
    fn to_proto(&self) -> Result<ProtoRealmId, HederaError> {
        Ok(ProtoRealmId {
            shard_num: self.shard_num,
            realm_num: self.realm_num,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::LedgerId;
    
    #[test]
    fn test_checksum() {
        let cs = LedgerId::for_mainnet().for_checksum();
        let shard = 32i64;
        let realm = 12i64;
        let num = 232i64;
        checksum(&cs, &format_id(&shard, &realm, &num)).unwrap();
    }
}

