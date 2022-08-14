mod utils;
use hedera_rust_client::{
    AccountBalanceQuery, Hbar, Key, NftId, TokenAssociateTransaction, TokenCreateTransaction,
    TokenMintTransaction, TokenSupplyType, TokenType, TransferTransaction, TokenWipeTransaction,
    TokenBurnTransaction,
};

#[test_log::test(tokio::test)]
#[ignore]
async fn test_transfer_fungible() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    // create token
    let init_supply = 1000000u64;
    let key: Key = env.client.operator_public_key().into();
    let receipt = TokenCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone())
        .unwrap()
        .set_name("transfer_test".to_string())
        .unwrap()
        .set_symbol("TEST".to_string())
        .unwrap()
        .set_memo("transfer_test".to_string())
        .unwrap()
        .set_decimals(3)
        .unwrap()
        .set_initial_supply(init_supply)
        .unwrap()
        .set_treasury(env.operator_id)
        .unwrap()
        .set_admin_key(key.clone())
        .unwrap()
        .set_supply_key(key.clone())
        .unwrap()
        .set_wipe_key(key.clone())
        .unwrap()
        .set_auto_renew_account(env.operator_id)
        .unwrap()
        .set_freeze_default(false)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let token_id = receipt
        .token_id
        .unwrap_or_else(|| panic!("no token_id in receipt: {:?}", receipt));

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

    let from_account_id = env.operator_id;
    let amount = 1000i64;
    let _tx = TransferTransaction::new()
        .add_token_transfer(token_id, from_account_id, -amount, None)
        .unwrap()
        .add_token_transfer(token_id, to_account_id, amount, None)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let balance_query = AccountBalanceQuery::new()
        .set_account_id(to_account_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    let balance = balance_query
        .token
        .get(&token_id)
        .unwrap_or_else(|| panic!("no token_id in query: {:?}", balance_query));
    let exp_amount: u64 = amount.try_into().unwrap();
    
    assert_eq!(balance, &exp_amount);

    let _tx = TokenWipeTransaction::new()
        .set_token_id(token_id)
        .unwrap()
        .set_account_id(to_account_id)
        .unwrap()
        .set_amount(exp_amount)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let amount = init_supply - exp_amount;
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

#[tokio::test]
#[ignore]
async fn test_transfer_nonfungible() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    // create token
    let key: Key = env.client.operator_public_key().into();
    let tx = TokenCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone())
        .unwrap()
        .set_name("transfer_test".to_string())
        .unwrap()
        .set_symbol("TEST".to_string())
        .unwrap()
        .set_decimals(0)
        .unwrap()
        .set_initial_supply(0)
        .unwrap()
        .set_supply_type(TokenSupplyType::Finite)
        .unwrap()
        .set_max_supply(250)
        .unwrap()
        .set_token_type(TokenType::NonFungibleUnique)
        .unwrap()
        .set_memo("transfer_test".to_string())
        .unwrap()
        .set_treasury(env.operator_id)
        .unwrap()
        .set_auto_renew_account(env.operator_id)
        .unwrap()
        .set_admin_key(key.clone())
        .unwrap()
        .set_supply_key(key.clone())
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

    // mint nft
    let metadata = "hello_world".as_bytes().to_vec();
    let tx = TokenMintTransaction::new()
        .set_token_id(token_id)
        .unwrap()
        .set_metadata(metadata)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let serial_number = tx
        .serial_numbers
        .get(0)
        .unwrap_or_else(|| panic!("no serial_numbers in receipt: {:?}", tx));

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

    // transfer nft
    let from_account_id = env.operator_id;
    let nft_id = NftId {
        token_id,
        serial_number: *serial_number,
    };
    let _tx = TransferTransaction::new()
        .add_nft_transfer(nft_id, from_account_id, to_account_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    // check from account balance
    let balance_query = AccountBalanceQuery::new()
        .set_account_id(from_account_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    let balance = balance_query.token.get(&token_id).unwrap();
    assert_eq!(balance, &0u64);

    // check to account balance
    let balance_query = AccountBalanceQuery::new()
        .set_account_id(to_account_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    let balance = balance_query.token.get(&token_id).unwrap();
    assert_eq!(balance, &1u64);

    env.close_with_token(token_id).await.unwrap();
}
