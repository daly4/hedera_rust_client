use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services::SemanticVersion as ProtoSemanticVersion;
use crate::proto::ToProto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: String,
    pub build: String,
}

impl TryFrom<ProtoSemanticVersion> for SemanticVersion {
    type Error = HederaError;
    fn try_from(services: ProtoSemanticVersion) -> Result<SemanticVersion, Self::Error> {
        Ok(SemanticVersion {
            major: services.major as u32,
            minor: services.minor as u32,
            patch: services.patch as u32,
            pre: services.pre,
            build: services.build,
        })
    }
}

impl ToProto<ProtoSemanticVersion> for SemanticVersion {
    fn to_proto(&self) -> std::result::Result<ProtoSemanticVersion, HederaError> {
        Ok(ProtoSemanticVersion {
            major: self.major as i32,
            minor: self.minor as i32,
            patch: self.patch as i32,
            pre: self.pre.clone(),
            build: self.build.clone(),
        })
    }
}
