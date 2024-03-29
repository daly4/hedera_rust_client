use hedera_rust_client_derive::{Id, IdPartialEq, IdValidateChecksum};
use serde::{Deserialize, Serialize};

use crate::error::HederaError;
use crate::id::IdChecksum;
use crate::proto::services::{contract_id::Contract, ContractId as ProtoContractId};
use crate::proto::ToProto;

#[derive(Id, IdPartialEq, IdValidateChecksum, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[hedera_rust_client_derive(field_name = "contract_num")]
pub struct ContractId {
    shard_num: i64,
    realm_num: i64,
    contract_num: i64,
    checksum: Option<IdChecksum>,
}

impl From<ProtoContractId> for ContractId {
    fn from(pb: ProtoContractId) -> ContractId {
        ContractId {
            shard_num: pb.shard_num,
            realm_num: pb.realm_num,
            contract_num: pb.contract.map_or(0, |v| match v {
                Contract::ContractNum(n) => n,
                _ => 0,
            }),
            checksum: None,
        }
    }
}

impl ToProto<ProtoContractId> for ContractId {
    fn to_proto(&self) -> Result<ProtoContractId, HederaError> {
        Ok(ProtoContractId {
            shard_num: self.shard_num,
            realm_num: self.realm_num,
            contract: Some(Contract::ContractNum(self.contract_num)),
        })
    }
}
