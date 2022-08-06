use std::env;
use std::fs::{create_dir_all, read_dir};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // collect every proto file in services/_

    let services_protos: Vec<_> = read_dir("./proto/services")?
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();

    // services

    let config = tonic_build::configure().build_server(false);
    config.compile(&services_protos, &[Path::new("proto/services/").to_owned()])?;

    // sdk
    // NOTE: must be compiled in a separate folder otherwise it will overwrite the previous build

    let sdk_out_dir = Path::new(&env::var("OUT_DIR")?).join("sdk");
    create_dir_all(&sdk_out_dir)?;

    tonic_build::configure()
        .build_server(false)
        .extern_path(".proto.Transaction", "crate::proto::services::Transaction")
        .out_dir(&sdk_out_dir)
        .compile(
            &["proto/sdk/transaction_list.proto"],
            &["proto/sdk/", "proto/services/"],
        )?;

    // mirror
    // NOTE: must be compiled in a separate folder otherwise it will overwrite the previous build

    let mirror_out_dir = Path::new(&env::var("OUT_DIR")?).join("mirror");
    create_dir_all(&mirror_out_dir)?;

    tonic_build::configure()
        .build_server(false)
        .extern_path(".proto.Timestamp", "crate::proto::services::Timestamp")
        .extern_path(".proto.TopicID", "crate::proto::services::TopicId")
        .extern_path(
            ".proto.ConsensusMessageChunkInfo",
            "crate::proto::services::ConsensusMessageChunkInfo",
        )
        .out_dir(&mirror_out_dir)
        .compile(
            &["proto/mirror/consensus_service.proto"],
            &["proto/mirror/", "proto/services/"],
        )?;

    // streams
    // NOTE: must be compiled in a separate folder otherwise it will overwrite the previous build

    let streams_out_dir = Path::new(&env::var("OUT_DIR")?).join("streams");
    create_dir_all(&streams_out_dir)?;

    tonic_build::configure()
        .extern_path(".proto.Fraction", "crate::proto::services::Fraction")
        .extern_path(".proto.Timestamp", "crate::proto::services::Timestamp")
        .extern_path(".proto.AccountID", "crate::proto::services::AccountId")
        .extern_path(".proto.TokenID", "crate::proto::services::TokenId")
        .extern_path(
            ".proto.AccountAmount",
            "crate::proto::services::AccountAmount",
        )
        .extern_path(
            ".proto.CurrentAndNextFeeSchedule",
            "crate::proto::services::CurrentAndNextFeeSchedule",
        )
        .extern_path(
            ".proto.FeeComponents",
            "crate::proto::services::FeeComponents",
        )
        .extern_path(".proto.FeeData", "crate::proto::services::FeeData")
        .extern_path(".proto.FeeSchedule", "crate::proto::services::FeeSchedule")
        .extern_path(".proto.Key", "crate::proto::services::Key")
        .extern_path(".proto.FileID", "crate::proto::services::FileId")
        .extern_path(".proto.KeyList", "crate::proto::services::KeyList")
        .extern_path(".proto.NftTransfer", "crate::proto::services::NftTransfer")
        .extern_path(".proto.NodeAddress", "crate::proto::services::NodeAddress")
        .extern_path(
            ".proto.NodeAddressBook",
            "crate::proto::services::NodeAddressBook",
        )
        .extern_path(".proto.RealmID", "crate::proto::services::RealmId")
        .extern_path(".proto.ScheduleID", "crate::proto::services::ScheduleId")
        .extern_path(
            ".proto.SemanticVersion",
            "crate::proto::services::SemanticVersion",
        )
        .extern_path(
            ".proto.ServiceEndpoint",
            "crate::proto::services::ServiceEndpoint",
        )
        .extern_path(
            ".proto.ServicesConfigurationList",
            "crate::proto::services::ServicesConfigurationList",
        )
        .extern_path(".proto.Setting", "crate::proto::services::Setting")
        .extern_path(".proto.ShardID", "crate::proto::services::ShardId")
        .extern_path(".proto.Signature", "crate::proto::services::Signature")
        .extern_path(
            ".proto.SignatureList",
            "crate::proto::services::SignatureList",
        )
        .extern_path(
            ".proto.SignatureMap",
            "crate::proto::services::SignatureMap",
        )
        .extern_path(
            ".proto.SignaturePair",
            "crate::proto::services::SignaturePair",
        )
        .extern_path(
            ".proto.ThresholdKey",
            "crate::proto::services::ThresholdKey",
        )
        .extern_path(
            ".proto.ThresholdSignature",
            "crate::proto::services::ThresholdSignature",
        )
        .extern_path(
            ".proto.TimestampSeconds",
            "crate::proto::services::TimestampSeconds",
        )
        .extern_path(
            ".proto.TokenBalance",
            "crate::proto::services::TokenBalance",
        )
        .extern_path(
            ".proto.TokenBalances",
            "crate::proto::services::TokenBalances",
        )
        .extern_path(
            ".proto.TokenRelationship",
            "crate::proto::services::TokenRelationship",
        )
        .extern_path(
            ".proto.TokenTransferList",
            "crate::proto::services::TokenTransferList",
        )
        .extern_path(".proto.TopicID", "crate::proto::services::TopicId")
        .extern_path(
            ".proto.TransactionFeeSchedule",
            "crate::proto::services::TransactionFeeSchedule",
        )
        .extern_path(
            ".proto.TransactionID",
            "crate::proto::services::TransactionId",
        )
        .extern_path(
            ".proto.TransferList",
            "crate::proto::services::TransferList",
        )
        .extern_path(
            ".proto.HederaFunctionality",
            "crate::proto::services::HederaFunctionality",
        )
        .extern_path(".proto.SubType", "crate::proto::services::SubType")
        .extern_path(
            ".proto.TokenFreezeStatus",
            "crate::proto::services::TokenFreezeStatus",
        )
        .extern_path(
            ".proto.TokenKycStatus",
            "crate::proto::services::TokenKycStatus",
        )
        .extern_path(
            ".proto.TokenSupplyType",
            "crate::proto::services::TokenSupplyType",
        )
        .extern_path(".proto.TokenType", "crate::proto::services::TokenType")
        .extern_path(".proto.ContractID", "crate::proto::services::ContractId")
        .out_dir(&streams_out_dir)
        .compile(
            &["proto/streams/account_balance_file.proto"],
            &["proto/streams/", "proto/services/"],
        )?;

    Ok(())
}
