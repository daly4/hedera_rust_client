use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services::Fraction as ProtoFraction;
use crate::proto::ToProto;

#[derive(Debug, Clone, PartialEq)]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
}

impl TryFrom<ProtoFraction> for Fraction {
    type Error = HederaError;
    fn try_from(services: ProtoFraction) -> Result<Fraction, Self::Error> {
        Ok(Fraction {
            numerator: services.numerator as u32,
            denominator: services.denominator as u32,
        })
    }
}

impl ToProto<ProtoFraction> for Fraction {
    fn to_proto(&self) -> std::result::Result<ProtoFraction, HederaError> {
        Ok(ProtoFraction {
            numerator: self.numerator as i64,
            denominator: self.denominator as i64,
        })
    }
}
