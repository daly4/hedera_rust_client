use serde::{Deserialize, Serialize};

use crate::error::HederaError;
use crate::file_id::FileId;
use crate::proto::services::contract_create_transaction_body::InitcodeSource as ProtoInitcodeSource;
use crate::proto::ToProto;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum InitcodeSource {
    FileId(FileId),
    Initcode(Vec<u8>),
}
impl ToProto<ProtoInitcodeSource> for InitcodeSource {
    fn to_proto(&self) -> Result<ProtoInitcodeSource, HederaError> {
        let pb = match &self {
            InitcodeSource::FileId(id) => ProtoInitcodeSource::FileId(id.to_proto()?),
            InitcodeSource::Initcode(code) => ProtoInitcodeSource::Initcode(code.clone()),
        };
        Ok(pb)
    }
}
