mod utils;
use chrono::Duration;
use hedera_rust_client::{
    Key, PrivateKey, Status, TokenCreateTransaction, TokenFreezeStatus, TokenInfoQuery,
    TokenKycStatus, HederaError, TokenBurnTransaction, FreezeDefault,
};

#[test_log::test(tokio::test)]
#[ignore]
async fn test_token_create() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let amount = 1000000u64;
    let key: Key = env.client.operator_public_key().into();
    let tx = TokenCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone())
        .unwrap()
        .set_name("ffff".to_string())
        .unwrap()
        .set_symbol("F".to_string())
        .unwrap()
        .set_memo("fnord".to_string())
        .unwrap()
        .set_decimals(3)
        .unwrap()
        .set_initial_supply(amount)
        .unwrap()
        .set_treasury(env.operator_id)
        .unwrap()
        .set_auto_renew_account(env.operator_id)
        .unwrap()
        .set_admin_key(key.clone())
        .unwrap()
        .set_freeze_key(key.clone())
        .unwrap()
        .set_wipe_key(key.clone())
        .unwrap()
        .set_kyc_key(key.clone())
        .unwrap()
        .set_supply_key(key.clone())
        .unwrap()
        .set_freeze_default(FreezeDefault::Unfrozen)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let token_id = tx
        .token_id
        .unwrap_or_else(|| panic!("no token_id in receipt: {:?}", tx));

    let info = TokenInfoQuery::new()
        .set_token_id(token_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    assert_eq!(info.token_id, token_id);
    assert_eq!(info.name, "ffff");
    assert_eq!(info.symbol, "F");
    assert_eq!(info.decimals, 3);
    assert_eq!(info.total_supply, amount);
    assert_eq!(info.treasury, env.operator_id);
    assert_eq!(info.admin_key, Some(key.clone()));
    assert_eq!(info.freeze_key, Some(key.clone()));
    assert_eq!(info.kyc_key, Some(key.clone()));
    assert_eq!(info.wipe_key, Some(key.clone()));
    assert_eq!(info.supply_key, Some(key.clone()));
    assert_eq!(
        info.default_freeze_status,
        TokenFreezeStatus::Unfrozen
    );
    assert_eq!(info.default_kyc_status, TokenKycStatus::Revoked);
    assert_eq!(info.auto_renew_period, Some(Duration::seconds(7890000)));
    assert_eq!(info.auto_renew_account, Some(env.operator_id));
    assert!(info.expiry.is_some());

    let _tx = TokenBurnTransaction::new()
        .set_token_id(token_id)
        .unwrap()
        .set_amount(amount)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    env.close_with_token(token_id).await.unwrap();
}

#[test_log::test(tokio::test)]
#[ignore]
async fn test_token_create_multiple_keys() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let amount = 1000000u64;

    let op_key: Key = env.client.operator_public_key().into();
    let freeze_key: Key = PrivateKey::new().into();
    let wipe_key: Key = PrivateKey::new().into();
    let kyc_key: Key = PrivateKey::new().into();
    let supply_key = PrivateKey::new();

    let tx = TokenCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone())
        .unwrap()
        .set_name("ffff".to_string())
        .unwrap()
        .set_symbol("F".to_string())
        .unwrap()
        .set_memo("fnord".to_string())
        .unwrap()
        .set_decimals(3)
        .unwrap()
        .set_initial_supply(amount)
        .unwrap()
        .set_treasury(env.operator_id)
        .unwrap()
        .set_auto_renew_account(env.operator_id)
        .unwrap()
        .set_admin_key(op_key.clone())
        .unwrap()
        .set_freeze_key(freeze_key.clone())
        .unwrap()
        .set_wipe_key(wipe_key.clone())
        .unwrap()
        .set_kyc_key(kyc_key.clone())
        .unwrap()
        .set_supply_key(supply_key.clone().into())
        .unwrap()
        .set_freeze_default(FreezeDefault::Unfrozen)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let token_id = tx
        .token_id
        .unwrap_or_else(|| panic!("no token_id in receipt: {:?}", tx));

    let info = TokenInfoQuery::new()
        .set_token_id(token_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    assert_eq!(info.token_id, token_id);
    assert_eq!(info.name, "ffff");
    assert_eq!(info.symbol, "F");
    assert_eq!(info.decimals, 3);
    assert_eq!(info.total_supply, amount);
    assert_eq!(info.treasury, env.operator_id);
    assert_eq!(info.admin_key, Some(op_key.clone()));
    assert_eq!(info.freeze_key, Some(freeze_key));
    assert_eq!(info.kyc_key, Some(kyc_key.clone()));
    assert_eq!(info.wipe_key, Some(wipe_key.clone()));
    assert_eq!(info.supply_key, Some(supply_key.clone().into()));
    assert_eq!(
        info.default_freeze_status,
        TokenFreezeStatus::Unfrozen
    );
    assert_eq!(info.default_kyc_status, TokenKycStatus::Revoked);
    assert_eq!(info.auto_renew_period, Some(Duration::seconds(7890000)));
    assert_eq!(info.auto_renew_account, Some(env.operator_id));
    assert!(info.expiry.is_some());

    let _tx = TokenBurnTransaction::new()
        .set_token_id(token_id)
        .unwrap()
        .set_amount(amount)
        .unwrap()
        .freeze_with(Some(&env.client))
        .await
        .unwrap()
        .sign(&supply_key)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    env.close_with_token(token_id).await.unwrap();
}

#[test_log::test(tokio::test)]
#[ignore]
async fn test_token_no_admin_sign() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let resp = TokenCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone())
        .unwrap()
        .set_name("ffff".to_string())
        .unwrap()
        .set_symbol("F".to_string())
        .unwrap()
        .set_memo("fnord".to_string())
        .unwrap()
        .set_decimals(3)
        .unwrap()
        .set_initial_supply(1000000)
        .unwrap()
        .set_treasury(env.operator_id)
        .unwrap()
        .set_auto_renew_account(env.operator_id)
        .unwrap()
        .set_admin_key(PrivateKey::new().into())
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await;

    let err = resp.as_ref().err().unwrap_or_else(|| panic!("no err {:?}", resp));
    match err {
        HederaError::ReceiptStatusError{ status, .. } => {
            assert_eq!(*status, Status::InvalidSignature);
        },
        _ => panic!("unexpected err: {:?}", err),
    }

    env.close().await.unwrap();
}