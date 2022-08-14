mod utils;
use hedera_rust_client::{
    AccountInfoQuery, Hbar, Key, TokenAssociateTransaction, TokenBurnTransaction,
    TokenCreateTransaction, TokenFreezeStatus, TokenFreezeTransaction, TokenInfoQuery,
    TokenKycStatus,
};

#[test_log::test(tokio::test)]
#[ignore]
async fn test_token_freeze() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    // create token
    let amount = 1000000u64;
    let key: Key = env.client.operator_public_key().into();
    let tx = TokenCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone())
        .unwrap()
        .set_name("ffff".to_string())
        .unwrap()
        .set_symbol("F".to_string())
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
        .set_supply_key(key)
        .unwrap()
        .set_freeze_default(false)
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

    // create account
    let (to_account_id, key) = env.new_test_account(Hbar::new(2.0)).await.unwrap();

    // associate new account w/ token
    let _tx = TokenAssociateTransaction::new()
        .set_account_id(to_account_id)
        .unwrap()
        .set_tokens(vec![token_id])
        .unwrap()
        .freeze_with(Some(&env.client))
        .await
        .unwrap()
        .sign(&key)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let _tx = TokenFreezeTransaction::new()
        .set_token_id(token_id)
        .unwrap()
        .set_account_id(to_account_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let info = AccountInfoQuery::new()
        .set_account_id(to_account_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    let relationship = info
        .token_relationships
        .get(&token_id)
        .unwrap_or_else(|| panic!("no token_id in relationships: {:?}", info));

    assert_eq!(relationship.token_id, token_id);
    assert_eq!(relationship.balance, 0);
    assert_eq!(relationship.kyc_status, TokenKycStatus::Revoked);
    assert_eq!(relationship.freeze_status, TokenFreezeStatus::Frozen);

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
