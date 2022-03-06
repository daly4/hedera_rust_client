use std::convert::{TryFrom, TryInto};

use crate::account_id::AccountId;
use crate::client::Client;
use crate::custom_fixed_fee::CustomFixedFee;
use crate::custom_fractional_fee::CustomFractionalFee;
use crate::custom_royalty_fee::CustomRoyaltyFee;
use crate::entity_id::{validate_option_id_checksum, ValidateChecksum};
use crate::error::HederaError;
use crate::proto::services::{custom_fee::Fee as ProtoFee, CustomFee as ProtoCustomFee};
use crate::proto::ToProto;
use crate::utils::optional_account_id;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFee {
    pub fee_collector_account_id: Option<AccountId>,
    pub fee: Option<Fee>,
}

impl ValidateChecksum for CustomFee {
    fn validate_checksum(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.fee_collector_account_id, client)
    }
}

impl TryFrom<ProtoCustomFee> for CustomFee {
    type Error = HederaError;
    fn try_from(services: ProtoCustomFee) -> Result<CustomFee, Self::Error> {
        let fee = match services.fee {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        Ok(CustomFee {
            fee_collector_account_id: optional_account_id(services.fee_collector_account_id)?,
            fee,
        })
    }
}

impl ToProto<ProtoCustomFee> for CustomFee {
    fn to_proto(&self) -> std::result::Result<ProtoCustomFee, HederaError> {
        let fee_collector_account_id = match &self.fee_collector_account_id {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        let fee = match &self.fee {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        Ok(ProtoCustomFee {
            fee_collector_account_id,
            fee,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Fee {
    CustomFixedFee(CustomFixedFee),
    CustomFractionalFee(CustomFractionalFee),
    CustomRoyaltyFee(CustomRoyaltyFee),
}

impl TryFrom<ProtoFee> for Fee {
    type Error = HederaError;
    fn try_from(services: ProtoFee) -> Result<Fee, Self::Error> {
        let fee = match services {
            ProtoFee::FixedFee(x) => Fee::CustomFixedFee(x.try_into()?),
            ProtoFee::FractionalFee(x) => Fee::CustomFractionalFee(x.try_into()?),
            ProtoFee::RoyaltyFee(x) => Fee::CustomRoyaltyFee(x.try_into()?),
        };
        Ok(fee)
    }
}

impl ToProto<ProtoFee> for Fee {
    fn to_proto(&self) -> Result<ProtoFee, HederaError> {
        let fee = match &self {
            Fee::CustomFixedFee(x) => ProtoFee::FixedFee(x.to_proto()?),
            Fee::CustomFractionalFee(x) => ProtoFee::FractionalFee(x.to_proto()?),
            Fee::CustomRoyaltyFee(x) => ProtoFee::RoyaltyFee(x.to_proto()?),
        };
        Ok(fee)
    }
}
