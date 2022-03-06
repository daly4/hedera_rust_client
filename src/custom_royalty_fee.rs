use std::convert::{TryFrom, TryInto};

use crate::custom_fixed_fee::CustomFixedFee;
use crate::error::HederaError;
use crate::fraction::Fraction;
use crate::proto::services::RoyaltyFee as ProtoRoyaltyFee;
use crate::proto::ToProto;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomRoyaltyFee {
    pub exchange_value_fraction: Option<Fraction>,
    pub fallback_fee: Option<CustomFixedFee>,
}

impl TryFrom<ProtoRoyaltyFee> for CustomRoyaltyFee {
    type Error = HederaError;
    fn try_from(services: ProtoRoyaltyFee) -> Result<CustomRoyaltyFee, Self::Error> {
        let exchange_value_fraction = match services.exchange_value_fraction {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        let fallback_fee = match services.fallback_fee {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        Ok(CustomRoyaltyFee {
            exchange_value_fraction,
            fallback_fee,
        })
    }
}

impl ToProto<ProtoRoyaltyFee> for CustomRoyaltyFee {
    fn to_proto(&self) -> std::result::Result<ProtoRoyaltyFee, HederaError> {
        let exchange_value_fraction = match &self.exchange_value_fraction {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        let fallback_fee = match &self.fallback_fee {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        Ok(ProtoRoyaltyFee {
            exchange_value_fraction,
            fallback_fee,
        })
    }
}
