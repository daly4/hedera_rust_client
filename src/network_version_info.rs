use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticVersion {
    /// Increases with incompatible API changes
    pub major: i32,
    /// Increases with backwards-compatible new functionality
    pub minor: i32,
    /// Increases with backwards-compatible bug fixes
    pub patch: i32,
}

impl From<services::SemanticVersion> for SemanticVersion {
    fn from(services: services::SemanticVersion) -> SemanticVersion {
        SemanticVersion {
            major: services.major,
            minor: services.minor,
            patch: services.patch,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkVersionInfo {
    pub hapi_proto_version: SemanticVersion,
    pub hedera_services_version: SemanticVersion,
}

impl TryFrom<services::NetworkGetVersionInfoResponse> for NetworkVersionInfo {
    type Error = HederaError;
    fn try_from(
        services: services::NetworkGetVersionInfoResponse,
    ) -> Result<NetworkVersionInfo, Self::Error> {
        Ok(NetworkVersionInfo {
            hapi_proto_version: match services.hapi_proto_version {
                Some(v) => SemanticVersion::from(v),
                None => {
                    return Err(HederaError::MissingInProto(
                        "hapi_proto_version".to_string(),
                    ))
                }
            },
            hedera_services_version: match services.hedera_services_version {
                Some(v) => SemanticVersion::from(v),
                None => {
                    return Err(HederaError::MissingInProto(
                        "hedera_services_version".to_string(),
                    ))
                }
            },
        })
    }
}
