use chrono::{DateTime, Duration, Utc};
use itertools::Itertools;
use std::convert::{TryFrom, TryInto};
use std::{fmt, str, str::FromStr};

use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::timestamp::HederaTimestamp;
use crate::AccountId;

#[derive(Debug, Clone, Hash, Eq)]
pub struct TransactionId {
    pub account_id: Option<AccountId>,
    pub transaction_valid_start: Option<DateTime<Utc>>,
    pub scheduled: bool,
    pub nonce: Option<i32>,
}

impl TransactionId {
    pub fn new(
        account_id: Option<AccountId>,
        transaction_valid_start: Option<DateTime<Utc>>,
        scheduled: bool,
        nonce: Option<i32>,
    ) -> TransactionId {
        TransactionId {
            account_id,
            transaction_valid_start,
            scheduled,
            nonce,
        }
    }

    pub fn with_valid_start(
        account_id: AccountId,
        transaction_valid_start: DateTime<Utc>,
        scheduled: bool,
    ) -> TransactionId {
        TransactionId::new(
            Some(account_id),
            Some(transaction_valid_start),
            scheduled,
            None,
        )
    }

    pub fn generate(account_id: AccountId) -> TransactionId {
        let transaction_valid_start = Utc::now() - Duration::seconds(10);
        TransactionId::new(Some(account_id), Some(transaction_valid_start), false, None)
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = if self.scheduled { "?scheduled" } else { "" }.to_string();

        if let Some(nonce) = self.nonce {
            s.push_str(&format!("/{}", nonce));
        }

        if self.account_id.is_some() && self.transaction_valid_start.is_some() {
            write!(
                f,
                "{}@{}.{}{}",
                self.account_id.as_ref().unwrap(),
                self.transaction_valid_start.unwrap().timestamp(),
                self.transaction_valid_start
                    .unwrap()
                    .timestamp_subsec_nanos(),
                s
            )
        } else {
            write!(f, "UNKNOWN")
        }
    }
}
// [{account}@{seconds}.{nanos}][?scheduled]
impl FromStr for TransactionId {
    type Err = HederaError;

    fn from_str(s: &str) -> Result<TransactionId, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        let l = parts.len();
        if parts.is_empty() || l > 2 {
            return Err(HederaError::InvalidTransactionIdFormat);
        }

        let mut nonce: Option<i32> = None;
        if l == 2 {
            nonce = Some(parts[1].parse::<i32>()?);
        }

        let parts: Vec<&str> = parts[0].split('?').collect();
        let mut scheduled = false;
        if parts.len() == 2 {
            scheduled = parts[1] == "scheduled";
        }

        // {account}@{seconds}.{nanos}
        match parts[0].split('@').next_tuple() {
            Some((account_id, timestamp)) => Ok(TransactionId::new(
                Some(AccountId::from_str(account_id)?),
                Some(DateTime::<Utc>::try_from(HederaTimestamp::from_str(
                    timestamp,
                )?)?),
                scheduled,
                nonce,
            )),
            None => Err(HederaError::InvalidTransactionIdFormat),
        }
    }
}

impl TryFrom<services::TransactionId> for TransactionId {
    type Error = HederaError;

    fn try_from(services: services::TransactionId) -> Result<TransactionId, Self::Error> {
        if services.account_id.is_some() && services.transaction_valid_start.is_some() {
            let mut nonce: Option<i32> = None;
            if services.nonce == 0 {
                nonce = Some(services.nonce);
            }
            Ok(TransactionId::new(
                Some(services.account_id.unwrap().try_into()?),
                Some(DateTime::<Utc>::try_from(
                    services.transaction_valid_start.unwrap(),
                )?),
                services.scheduled,
                nonce,
            ))
        } else {
            Err(HederaError::MissingInProto(
                "account_id and transaction_valid_start".to_string(),
            ))
        }
    }
}

impl ToProto<services::TransactionId> for TransactionId {
    fn to_proto(&self) -> Result<services::TransactionId, HederaError> {
        Ok(services::TransactionId {
            transaction_valid_start: match &self.transaction_valid_start {
                Some(v) => Some(v.to_proto()?),
                None => None,
            },
            account_id: match &self.account_id {
                Some(v) => Some(v.to_proto()?),
                None => None,
            },
            scheduled: self.scheduled,
            nonce: self.nonce.unwrap_or(0),
        })
    }
}

impl PartialEq for TransactionId {
    fn eq(&self, other: &Self) -> bool {
        if self.account_id.is_some()
            && other.account_id.is_some()
            && self.transaction_valid_start.is_some()
            && other.transaction_valid_start.is_some()
        {
            self.account_id.as_ref().unwrap() == other.account_id.as_ref().unwrap()
                && self.transaction_valid_start.unwrap() == other.transaction_valid_start.unwrap()
                && self.scheduled == other.scheduled
                && self.nonce == other.nonce
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn test_display() {
        let account_id = AccountId::from_num(7, 5, 1001);
        let transaction_valid_start =
            DateTime::<Utc>::try_from(HederaTimestamp(1234567, 10001)).unwrap();
        let transaction_id =
            TransactionId::with_valid_start(account_id, transaction_valid_start, false);

        assert_eq!(format!("{}", transaction_id), "7.5.1001@1234567.10001");
    }

    #[test]
    fn test_from_string() {
        let account_id = AccountId::from_num(0, 0, 3);
        let from: &'static str = "0.0.3@1614997926.774912965?scheduled";
        let tx_id: TransactionId = from.parse().unwrap();
        assert_eq!(tx_id.account_id, Some(account_id));
        assert_eq!(tx_id.nonce, None);
        assert!(tx_id.scheduled);
    }

    #[test]
    fn test_from_string_nonce() {
        let account_id = AccountId::from_num(0, 0, 3);
        let from: &'static str = "0.0.3@1614997926.774912965?scheduled/4";
        let tx_id: TransactionId = from.parse().unwrap();
        assert_eq!(tx_id.account_id, Some(account_id));
        assert_eq!(tx_id.nonce, Some(4i32));
        assert!(tx_id.scheduled);
    }

    #[test]
    fn test_parse() {
        let account_id = AccountId::from_num(7, 5, 1001);
        let transaction_valid_start =
            DateTime::<Utc>::try_from(HederaTimestamp(1234567, 10001)).unwrap();
        let transaction_id =
            TransactionId::with_valid_start(account_id, transaction_valid_start, false);

        assert_eq!(
            "7.5.1001@1234567.10001".parse::<TransactionId>().unwrap(),
            transaction_id,
            "\nexpected: {:?}\nactual: {:?}",
            transaction_id,
            "7:5:1001@1234567.10001".parse::<TransactionId>().unwrap()
        );
    }
}
