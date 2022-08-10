use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::fee_components::FeeComponents;
use crate::proto::services::FeeData as ProtoFeeData;
use crate::proto::ToProto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeeData {
    pub node_data: Option<FeeComponents>,
    pub network_data: Option<FeeComponents>,
    pub service_data: Option<FeeComponents>,
    pub fee_data_type: FeeDataType,
}

impl TryFrom<ProtoFeeData> for FeeData {
    type Error = HederaError;
    fn try_from(services: ProtoFeeData) -> Result<FeeData, Self::Error> {
        let node_data = match services.nodedata {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        let network_data = match services.networkdata {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        let service_data = match services.servicedata {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        Ok(FeeData {
            node_data,
            network_data,
            service_data,
            fee_data_type: FeeDataType::from_i32(services.sub_type)
                .ok_or(HederaError::UnexpectedProtoType)?,
        })
    }
}

impl ToProto<ProtoFeeData> for FeeData {
    fn to_proto(&self) -> Result<ProtoFeeData, HederaError> {
        let nodedata = match self.node_data {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        let networkdata = match self.network_data {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        let servicedata = match self.service_data {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        Ok(ProtoFeeData {
            nodedata,
            networkdata,
            servicedata,
            sub_type: self
                .fee_data_type
                .to_i32()
                .ok_or(HederaError::UnexpectedProtoType)?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum FeeDataType {
    Default = 0,
    TokenFungibleCommon = 1,
    TokenNonFungibleUnique = 2,
    TokenFungibleCommonWithCustomFees = 3,
    TokenNonFungibleUniqueWithCustomFees = 4,
}
