mod utils;
use chrono::Duration;
use hedera_rust_client::{
    Key, PrivateKey, TokenCreateTransaction, TokenInfoQuery, TokenSupplyType, TokenType, Status,
};

#[test_log::test(tokio::test)]
#[ignore]
async fn test_token_create() {
    
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let key: Key = env.client.operator_public_key().into();
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
        .set_admin_key(key.clone())
        .unwrap()
        .set_freeze_key(key.clone())
        .unwrap()
        .set_wipe_key(key.clone())
        .unwrap()
        .set_kyc_key(key.clone())
        .unwrap()
        .set_supply_key(key)
        .unwrap()
        .set_freeze_default(false)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    let receipt = resp.get_receipt(&env.client).await.unwrap();

    let token_id = receipt
        .token_id
        .unwrap_or_else(|| panic!("no token_id in receipt: {:?}", receipt));

    env.close_with_token(token_id).await.unwrap();
}

#[test_log::test(tokio::test)]
#[ignore]
async fn test_token_create_multiple_keys() {
    
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let n_keys = 4usize;
    let mut keys: Vec<Key> = Vec::with_capacity(n_keys);

    for _i in 0..n_keys {
        let new_key = PrivateKey::new();
        keys.push(new_key.into());
    }

    let op_key: Key = env.client.operator_public_key().into();
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
        .set_admin_key(op_key)
        .unwrap()
        .set_freeze_key(keys[0].clone())
        .unwrap()
        .set_wipe_key(keys[1].clone())
        .unwrap()
        .set_kyc_key(keys[2].clone())
        .unwrap()
        .set_supply_key(keys[3].clone())
        .unwrap()
        .set_freeze_default(false)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    let receipt = resp.get_receipt(&env.client).await.unwrap();

    let token_id = receipt
        .token_id
        .unwrap_or_else(|| panic!("no token_id in receipt: {:?}", receipt));

    env.close_with_token(token_id).await.unwrap();
}

#[test_log::test(tokio::test)]
#[ignore]
async fn test_token_create_no_keys() {
    
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let resp = TokenCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone())
        .unwrap()
        .set_name("ffff".to_string())
        .unwrap()
        .set_symbol("F".to_string())
        .unwrap()
        .set_treasury(env.operator_id)
        .unwrap()
        .set_auto_renew_account(env.operator_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    let receipt = resp.get_receipt(&env.client).await.unwrap();

    let token_id = receipt
        .token_id
        .unwrap_or_else(|| panic!("no token_id in receipt: {:?}", receipt));

    let info = TokenInfoQuery::new()
        .set_node_account_ids(vec![resp.node_id])
        .unwrap()
        .set_token_id(token_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    assert_eq!(info.name, "ffff");
    assert_eq!(info.symbol, "F");
    assert_eq!(info.decimals, 0);
    assert_eq!(info.total_supply, 0);
    assert_eq!(info.treasury, env.operator_id);
    assert!(info.admin_key.is_none());
    assert!(info.freeze_key.is_none());
    assert!(info.kyc_key.is_none());
    assert!(info.wipe_key.is_none());
    assert!(info.supply_key.is_none());
    assert!(!info.default_freeze_status);
    assert!(!info.default_kyc_status);
    assert_eq!(info.auto_renew_period, Some(Duration::seconds(7890000)));
    assert_eq!(info.auto_renew_account, Some(env.operator_id));
    assert!(info.expiry.is_some());

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
        .set_admin_key(PrivateKey::new().into())
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();
    
    let receipt = resp.get_receipt(&env.client).await.unwrap();
    
    assert_eq!(receipt.status, Status::InvalidSignature);

    env.close().await.unwrap();
}

#[test_log::test(tokio::test)]
#[ignore]
async fn test_token_nft_create() {
    
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let key: Key = env.client.operator_public_key().into();
    let resp = TokenCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone())
        .unwrap()
        .set_name("ffff".to_string())
        .unwrap()
        .set_symbol("F".to_string())
        .unwrap()
        .set_memo("fnord".to_string())
        .unwrap()
        .set_token_type(TokenType::NonFungibleUnique)
        .unwrap()
        .set_supply_type(TokenSupplyType::Finite)
        .unwrap()
        .set_max_supply(5)
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
        .set_supply_key(key)
        .unwrap()
        .set_freeze_default(false)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    let receipt = resp.get_receipt(&env.client).await.unwrap();

    let token_id = receipt
        .token_id
        .unwrap_or_else(|| panic!("no token_id in receipt: {:?}", receipt));

    env.close_with_token(token_id).await.unwrap();
}
