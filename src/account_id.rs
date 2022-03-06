use hedera_derive::IdPartialEq;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};

use crate::client::Client;
use crate::entity_id::*;
use crate::entity_id::{validate, ValidateChecksum};
use crate::error::HederaError;
use crate::id::{id_from_string, IdChecksum};
use crate::proto::services::account_id::Account as ProtoAccount;
use crate::proto::services::AccountId as ProtoAccountId;
use crate::proto::ToProto;
use crate::PublicKey;

#[derive(IdPartialEq, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[hedera_derive(field_name = "account")]
pub struct AccountId {
    shard_num: i64,
    realm_num: i64,
    account: Account,
    checksum: Option<IdChecksum>,
}

impl AccountId {
    pub fn new(
        shard_num: i64,
        realm_num: i64,
        account: Account,
        checksum: Option<IdChecksum>,
    ) -> AccountId {
        AccountId {
            shard_num,
            realm_num,
            account,
            checksum,
        }
    }

    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn simple(account_num: i64) -> AccountId {
        Self::new(0, 0, Account::AccountNum(account_num), None)
    }

    pub fn from_num(shard_num: i64, realm_num: i64, account_num: i64) -> AccountId {
        Self::new(shard_num, realm_num, Account::AccountNum(account_num), None)
    }

    pub fn from_solidity_address(s: &str) -> Result<Self, HederaError> {
        let (shard_num, realm_num, num) = crate::entity_id::id_from_solidity_address(s)?;
        Ok(Self::from_num(shard_num, realm_num, num))
    }

    pub fn to_string_with_checksum(&self, client: &Client) -> Result<String, HederaError> {
        match &self.account {
            Account::AccountNum(x) => {
                let cs = client.ledger_id().for_checksum();
                let check = checksum(&cs, &format_id(&self.shard_num, &self.realm_num, x))?;
                let s = format_id_with_checksum(&self.shard_num, &self.realm_num, x, &check);
                Ok(s)
            }
            _ => Err(HederaError::UnableToConvertAccountAlias),
        }
    }

    pub fn to_solidity_address(&self) -> Result<String, HederaError> {
        match &self.account {
            Account::AccountNum(x) => id_to_solidity_address(self.shard_num, self.realm_num, *x),
            _ => Err(HederaError::UnableToConvertAccountAlias),
        }
    }
}

impl TryFrom<ProtoAccountId> for AccountId {
    type Error = HederaError;
    fn try_from(services: ProtoAccountId) -> Result<AccountId, Self::Error> {
        Ok(Self::new(
            services.shard_num,
            services.realm_num,
            services.account.ok_or(HederaError::NoAccountId)?.try_into()?,
            None,
        ))
    }
}

impl ToProto<ProtoAccountId> for AccountId {
    fn to_proto(&self) -> std::result::Result<ProtoAccountId, HederaError> {
        Ok(ProtoAccountId {
            shard_num: self.shard_num,
            realm_num: self.realm_num,
            account: Some(self.account.to_proto()?),
        })
    }
}

impl std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self.account {
            Account::Alias(pk) => {
                format!("{}.{}.{}", &self.shard_num, &self.realm_num, pk.to_string())
            }
            Account::AccountNum(n) => match &self.checksum {
                Some(check) => format_id_with_checksum(&self.shard_num, &self.realm_num, n, check),
                None => format_id(&self.shard_num, &self.realm_num, n),
            },
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for AccountId {
    type Err = HederaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shard, realm, num, checksum, alias) = id_from_string(s)?;
        let account = if num == -1 {
            Account::Alias(alias.ok_or(HederaError::UnknownIdFormat)?)
        } else {
            Account::AccountNum(num)
        };
        Ok(Self::new(shard, realm, account, checksum))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
pub enum Account {
    AccountNum(i64),
    Alias(PublicKey),
}

impl Ord for Account {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Account::AccountNum(x), Account::AccountNum(y)) => x.cmp(y),
            _ => Ordering::Less,
        }
    }
}

impl PartialOrd for Account {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<ProtoAccount> for Account {
    type Error = HederaError;
    fn try_from(services: ProtoAccount) -> Result<Account, Self::Error> {
        let ac = match services {
            ProtoAccount::AccountNum(n) => Account::AccountNum(n),
            ProtoAccount::Alias(pk) => Account::Alias(PublicKey::from_bytes(&pk)?),
        };
        Ok(ac)
    }
}

impl ToProto<ProtoAccount> for Account {
    fn to_proto(&self) -> std::result::Result<ProtoAccount, HederaError> {
        match *self {
            Account::AccountNum(n) => Ok(ProtoAccount::AccountNum(n)),
            Account::Alias(pk) => Ok(ProtoAccount::Alias(pk.as_bytes().to_vec())),
        }
    }
}

impl ValidateChecksum for AccountId {
    fn validate_checksum(&self, client: &Client) -> Result<(), HederaError> {
        match &self.account {
            Account::AccountNum(n) => {
                validate(client, &self.shard_num, &self.realm_num, n, &self.checksum)
            }
            Account::Alias(_) => Err(HederaError::UnableToValidateAccountAlias),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::PrivateKey;

    #[test]
    fn test_from_string() {
        let from: &'static str = "0.0.123-rmkyk";
        let account_id: AccountId = from.parse().unwrap();
        assert_eq!(account_id.account(), &Account::AccountNum(123));
    }

    #[test]
    fn test_to_string() {
        let account_id = AccountId::new(50, 150, Account::AccountNum(520), None);
        assert_eq!(account_id.to_string(), "50.150.520");
    }

    #[test]
    fn test_from_string_alias() {
        let (key, _) = PrivateKey::generate("test-password");
        let pk = key.public();
        let from = format!("0.0.{}", pk.to_string());
        let account_id: AccountId = from.parse().unwrap();
        assert_eq!(account_id.account(), &Account::Alias(pk));
    }
}
