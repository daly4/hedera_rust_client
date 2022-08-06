use chrono::{DateTime, Utc};
use std::convert::TryFrom;

use crate::key_list::KeyList;
use crate::proto::services;
use crate::utils;
use crate::FileId;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub file_id: FileId,
    pub size: i64,
    pub expiration_time: Option<DateTime<Utc>>,
    pub deleted: bool,
    pub keys: Option<KeyList>,
    pub file_memo: String,
}

impl TryFrom<services::file_get_info_response::FileInfo> for FileInfo {
    type Error = crate::error::HederaError;
    fn try_from(
        services: services::file_get_info_response::FileInfo,
    ) -> Result<FileInfo, Self::Error> {
        Ok(FileInfo {
            file_id: utils::non_optional_file_id(services.file_id)?,
            size: services.size,
            expiration_time: utils::optional_timestamp(services.expiration_time)?,
            deleted: services.deleted,
            keys: utils::optional_key_list(services.keys)?,
            file_memo: services.memo,
        })
    }
}
