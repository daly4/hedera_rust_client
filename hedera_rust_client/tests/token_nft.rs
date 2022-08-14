mod utils;
use hedera_rust_client::{
    Hbar, Key, NftId, TokenAssociateTransaction, TokenBurnTransaction, TokenCreateTransaction,
    TokenGrantKycTransaction, TokenMintTransaction, TokenNftInfoQuery, TokenType,
    TokenWipeTransaction, TransferTransaction,
};

#[test_log::test(tokio::test)]
#[ignore]
async fn test_token_nft_create() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

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
        .set_token_type(TokenType::NonFungibleUnique)
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
        .unwrap_or_else(|| panic!("no token_id: {:?}", tx));

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

    let _tx = TokenGrantKycTransaction::new()
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

    //TokenMintTransaction
    let serials = TokenMintTransaction::new()
        .set_token_id(token_id)
        .unwrap()
        .add_metadata(vec![1u8])
        .unwrap()
        .add_metadata(vec![2u8])
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap()
        .serial_numbers;

    let serial = serials.get(0).unwrap();

    let nft_id = NftId::new(token_id, *serial);

    let info = TokenNftInfoQuery::new()
        .set_nft_id(nft_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    assert_eq!(info.account_id, env.operator_id);

    let _tx = TransferTransaction::new()
        .add_nft_transfer(nft_id, env.operator_id, to_account_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let info = TokenNftInfoQuery::new()
        .set_nft_id(nft_id)
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    assert_eq!(info.account_id, to_account_id);

    let _tx = TokenWipeTransaction::new()
        .set_token_id(token_id)
        .unwrap()
        .set_account_id(to_account_id)
        .unwrap()
        .set_serial_numbers(vec![*serial])
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    let serial = serials.get(1).unwrap();

    let _tx = TokenBurnTransaction::new()
        .set_token_id(token_id)
        .unwrap()
        .set_serial_numbers(vec![*serial])
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap()
        .get_receipt(&env.client)
        .await
        .unwrap();

    env.close_with_token(token_id).await.unwrap();
}
