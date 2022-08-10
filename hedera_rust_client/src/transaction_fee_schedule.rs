use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::{TryFrom, TryInto};

use crate::error::HederaError;
use crate::fee_data::FeeData;
use crate::proto::services::{
    FeeData as ProtoFeeData, TransactionFeeSchedule as ProtoTransactionFeeSchedule,
};
use crate::proto::ToProto;

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionFeeSchedule {
    pub hedera_functionality: HederaFunctionality,
    pub fee_data: Option<FeeData>,
    pub fees: Vec<FeeData>,
}

impl TryFrom<ProtoTransactionFeeSchedule> for TransactionFeeSchedule {
    type Error = HederaError;
    fn try_from(
        services: ProtoTransactionFeeSchedule,
    ) -> Result<TransactionFeeSchedule, Self::Error> {
        #[allow(deprecated)]
        let fee_data = match services.fee_data {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        let fees = services
            .fees
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<FeeData>, HederaError>>()?;
        Ok(TransactionFeeSchedule {
            hedera_functionality: HederaFunctionality::from_i32(services.hedera_functionality)
                .ok_or(HederaError::UnexpectedProtoType)?,
            fee_data,
            fees,
        })
    }
}

impl ToProto<ProtoTransactionFeeSchedule> for TransactionFeeSchedule {
    fn to_proto(&self) -> Result<ProtoTransactionFeeSchedule, HederaError> {
        let fee_data = match self.fee_data {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        let fees = self
            .fees
            .iter()
            .map(|x| x.to_proto())
            .collect::<Result<Vec<ProtoFeeData>, HederaError>>()?;
        #[allow(deprecated)]
        Ok(ProtoTransactionFeeSchedule {
            hedera_functionality: self
                .hedera_functionality
                .to_i32()
                .ok_or(HederaError::UnexpectedProtoType)?,
            fee_data,
            fees,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum HederaFunctionality {
    ///*
    /// UNSPECIFIED - Need to keep first value as unspecified because first element is ignored and
    /// not parsed (0 is ignored by parser)
    None = 0,
    ///*
    /// crypto transfer
    CryptoTransfer = 1,
    ///*
    /// crypto update account
    CryptoUpdate = 2,
    ///*
    /// crypto delete account
    CryptoDelete = 3,
    ///*
    /// Add a livehash to a crypto account
    CryptoAddLiveHash = 4,
    ///*
    /// Delete a livehash from a crypto account
    CryptoDeleteLiveHash = 5,
    ///*
    /// Smart Contract Call
    ContractCall = 6,
    ///*
    /// Smart Contract Create Contract
    ContractCreate = 7,
    ///*
    /// Smart Contract update contract
    ContractUpdate = 8,
    ///*
    /// File Operation create file
    FileCreate = 9,
    ///*
    /// File Operation append file
    FileAppend = 10,
    ///*
    /// File Operation update file
    FileUpdate = 11,
    ///*
    /// File Operation delete file
    FileDelete = 12,
    ///*
    /// crypto get account balance
    CryptoGetAccountBalance = 13,
    ///*
    /// crypto get account record
    CryptoGetAccountRecords = 14,
    ///*
    /// Crypto get info
    CryptoGetInfo = 15,
    ///*
    /// Smart Contract Call
    ContractCallLocal = 16,
    ///*
    /// Smart Contract get info
    ContractGetInfo = 17,
    ///*
    /// Smart Contract, get the byte code
    ContractGetBytecode = 18,
    ///*
    /// Smart Contract, get by solidity ID
    GetBySolidityId = 19,
    ///*
    /// Smart Contract, get by key
    GetByKey = 20,
    ///*
    /// Get a live hash from a crypto account
    CryptoGetLiveHash = 21,
    ///*
    /// Crypto, get the stakers for the node
    CryptoGetStakers = 22,
    ///*
    /// File Operations get file contents
    FileGetContents = 23,
    ///*
    /// File Operations get the info of the file
    FileGetInfo = 24,
    ///*
    /// Crypto get the transaction records
    TransactionGetRecord = 25,
    ///*
    /// Contract get the transaction records
    ContractGetRecords = 26,
    ///*
    /// crypto create account
    CryptoCreate = 27,
    ///*
    /// system delete file
    SystemDelete = 28,
    ///*
    /// system undelete file
    SystemUndelete = 29,
    ///*
    /// delete contract
    ContractDelete = 30,
    ///*
    /// freeze
    Freeze = 31,
    ///*
    /// Create Tx Record
    CreateTransactionRecord = 32,
    ///*
    /// Crypto Auto Renew
    CryptoAccountAutoRenew = 33,
    ///*
    /// Contract Auto Renew
    ContractAutoRenew = 34,
    ///*
    /// Get Version
    GetVersionInfo = 35,
    ///*
    /// Transaction Get Receipt
    TransactionGetReceipt = 36,
    ///*
    /// Create Topic
    ConsensusCreateTopic = 50,
    ///*
    /// Update Topic
    ConsensusUpdateTopic = 51,
    ///*
    /// Delete Topic
    ConsensusDeleteTopic = 52,
    ///*
    /// Get Topic information
    ConsensusGetTopicInfo = 53,
    ///*
    /// Submit message to topic
    ConsensusSubmitMessage = 54,
    UncheckedSubmit = 55,
    ///*
    /// Create Token
    TokenCreate = 56,
    ///*
    /// Get Token information
    TokenGetInfo = 58,
    ///*
    /// Freeze Account
    TokenFreezeAccount = 59,
    ///*
    /// Unfreeze Account
    TokenUnfreezeAccount = 60,
    ///*
    /// Grant KYC to Account
    TokenGrantKycToAccount = 61,
    ///*
    /// Revoke KYC from Account
    TokenRevokeKycFromAccount = 62,
    ///*
    /// Delete Token
    TokenDelete = 63,
    ///*
    /// Update Token
    TokenUpdate = 64,
    ///*
    /// Mint tokens to treasury
    TokenMint = 65,
    ///*
    /// Burn tokens from treasury
    TokenBurn = 66,
    ///*
    /// Wipe token amount from Account holder
    TokenAccountWipe = 67,
    ///*
    /// Associate tokens to an account
    TokenAssociateToAccount = 68,
    ///*
    /// Dissociate tokens from an account
    TokenDissociateFromAccount = 69,
    ///*
    /// Create Scheduled Transaction
    ScheduleCreate = 70,
    ///*
    /// Delete Scheduled Transaction
    ScheduleDelete = 71,
    ///*
    /// Sign Scheduled Transaction
    ScheduleSign = 72,
    ///*
    /// Get Scheduled Transaction Information
    ScheduleGetInfo = 73,
    ///*
    /// Get Token Account Nft Information
    TokenGetAccountNftInfos = 74,
    ///*
    /// Get Token Nft Information
    TokenGetNftInfo = 75,
    ///*
    /// Get Token Nft List Information
    TokenGetNftInfos = 76,
    ///*
    /// Update a token's custom fee schedule, if permissible
    TokenFeeScheduleUpdate = 77,
    ///*
    /// Get execution time(s) by TransactionID, if available
    NetworkGetExecutionTime = 78,
    ///*
    /// Pause the Token
    TokenPause = 79,
    ///*
    /// Unpause the Token
    TokenUnpause = 80,
}
