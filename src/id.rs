use serde::{Deserialize, Serialize};
use std::fmt::{self};
use std::str::FromStr;

use crate::error::HederaError;
use crate::PublicKey;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct IdChecksum([u8; 5]);

impl IdChecksum {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }
}

impl TryFrom<[u32; 5]> for IdChecksum {
    type Error = HederaError;
    fn try_from(arr: [u32; 5]) -> Result<Self, Self::Error> {
        let mut b = [0; 5];
        for (i, c) in arr.into_iter().enumerate() {
            b[i] = u8::try_from(c).map_err(|_| HederaError::InvalidChecksum)?;
        }
        Ok(IdChecksum(b))
    }
}

impl FromStr for IdChecksum {
    type Err = HederaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = [0; 5];
        for c in s[0..5].chars() {
            if !c.is_ascii() {
                return Err(HederaError::InvalidChecksum);
            }
            c.encode_utf8(&mut b);
        }
        Ok(IdChecksum(b))
    }
}

impl fmt::Display for IdChecksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub fn id_from_string(
    s: &str,
) -> Result<(i64, i64, i64, Option<IdChecksum>, Option<PublicKey>), HederaError> {
    let id_s;
    let mut checksum: Option<IdChecksum> = None;
    if s.contains('-') {
        let sub = s.split('-').collect::<Vec<&str>>();
        if sub.len() > 2 {
            return Err(HederaError::UnknownIdChecksumFormat);
        }
        id_s = sub[0];
        checksum = Some(sub[1].parse()?);
    } else {
        id_s = s;
    }

    let split = id_s.split([':', '.'].as_ref()).collect::<Vec<&str>>();

    if split.len() > 3 {
        return Err(HederaError::UnknownIdFormat);
    }

    let shard = split[0]
        .parse::<i64>()
        .map_err(|_| HederaError::UnknownIdFormat)?;
    let realm = split[1]
        .parse::<i64>()
        .map_err(|_| HederaError::UnknownIdFormat)?;

    match split[2].parse::<PublicKey>() {
        Ok(public_key) => Ok((shard, realm, -1, checksum, Some(public_key))),
        Err(_) => {
            let n = split[2]
                .parse::<i64>()
                .map_err(|_| HederaError::UnknownIdFormat)?;
            Ok((shard, realm, n, checksum, None))
        }
    }
}
