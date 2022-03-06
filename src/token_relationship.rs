use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services;
use crate::TokenId;

#[derive(Debug, Clone)]
pub struct TokenRelationship {
    pub token_id: TokenId,
    pub symbol: String,
    pub balance: u64,
    pub kyc_status: Option<bool>,
    pub freeze_status: Option<bool>,
}

impl TryFrom<services::TokenRelationship> for TokenRelationship {
    type Error = HederaError;
    fn try_from(services: services::TokenRelationship) -> Result<TokenRelationship, Self::Error> {
        let mut kyc_status: Option<bool> = None; // KycNotApplicable = 0,
        if services.kyc_status == 1 {
            // Granted = 1,
            kyc_status = Some(true);
        } else if services.kyc_status == 2 {
            // Revoked = 2,
            kyc_status = Some(false);
        }

        let mut freeze_status: Option<bool> = None; // FreezeNotApplicable = 0,
        if services.freeze_status == 1 {
            // Frozen = 1
            freeze_status = Some(true);
        } else if services.kyc_status == 2 {
            // Unfrozen = 2,
            freeze_status = Some(false);
        }

        Ok(TokenRelationship {
            token_id: match services.token_id {
                Some(v) => TokenId::from(v),
                None => return Err(HederaError::MissingInProto("token_id".to_string())),
            },
            symbol: services.symbol,
            balance: services.balance,
            kyc_status,
            freeze_status,
        })
    }
}
