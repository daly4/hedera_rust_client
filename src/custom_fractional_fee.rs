use std::convert::TryFrom;

use crate::error::HederaError;
use crate::fraction::Fraction;
use crate::proto::services::FractionalFee as ProtoFractionalFee;
use crate::proto::ToProto;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFractionalFee {
    pub fractional_amount: Option<Fraction>,
    pub minimum_amount: i64,
    pub maximum_amount: i64,
    pub net_of_transfers: bool,
}

impl TryFrom<ProtoFractionalFee> for CustomFractionalFee {
    type Error = HederaError;
    fn try_from(services: ProtoFractionalFee) -> Result<CustomFractionalFee, Self::Error> {
        let fractional_amount = match services.fractional_amount {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        Ok(CustomFractionalFee {
            fractional_amount,
            minimum_amount: services.minimum_amount,
            maximum_amount: services.maximum_amount,
            net_of_transfers: services.net_of_transfers,
        })
    }
}

impl ToProto<ProtoFractionalFee> for CustomFractionalFee {
    fn to_proto(&self) -> std::result::Result<ProtoFractionalFee, HederaError> {
        let fractional_amount = match &self.fractional_amount {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        Ok(ProtoFractionalFee {
            fractional_amount,
            minimum_amount: self.minimum_amount,
            maximum_amount: self.maximum_amount,
            net_of_transfers: self.net_of_transfers,
        })
    }
}
