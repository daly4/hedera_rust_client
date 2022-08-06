mod utils;
use chrono::Duration;
use hedera_rust_client::{
    Key, PrivateKey, TokenCreateTransaction, TokenInfoQuery, TokenSupplyType, TokenType,
};

#[tokio::test]
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
        .expect(&format!("no token_id in receipt: {:?}", receipt));

    env.close_with_token(token_id).await.unwrap();
}

#[tokio::test]
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
        .expect(&format!("no token_id in receipt: {:?}", receipt));

    env.close_with_token(token_id).await.unwrap();
}

#[tokio::test]
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
        .expect(&format!("no token_id in receipt: {:?}", receipt));

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

    env.close().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn test_token_create_admin_sign() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let n_keys = 5usize;
    let mut priv_keys: Vec<PrivateKey> = Vec::with_capacity(n_keys);
    let mut keys: Vec<Key> = Vec::with_capacity(n_keys);

    for _i in 0..n_keys {
        let new_key = PrivateKey::new();
        priv_keys.push(new_key.clone());
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
        .set_freeze_key(keys[1].clone())
        .unwrap()
        .set_wipe_key(keys[2].clone())
        .unwrap()
        .set_kyc_key(keys[3].clone())
        .unwrap()
        .set_supply_key(keys[4].clone())
        .unwrap()
        .freeze_with(Some(&env.client))
        .await
        .unwrap()
        .sign(&priv_keys[0])
        .unwrap()
        .sign(&priv_keys[1])
        .unwrap()
        .execute(&env.client)
        .await
        .unwrap();

    let receipt = resp.get_receipt(&env.client).await.unwrap();

    let token_id = receipt
        .token_id
        .expect(&format!("no token_id in receipt: {:?}", receipt));

    env.close_with_token(token_id).await.unwrap();
}

#[tokio::test]
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
        .expect(&format!("no token_id in receipt: {:?}", receipt));

    env.close_with_token(token_id).await.unwrap();
}

// func TestIntegrationTokenCreateTransactionWithCustomFees(t *testing.T) {
// 	env := NewIntegrationTestEnv(t)

// 	resp, err := NewTokenCreateTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenName("ffff").
// 		SetTokenSymbol("F").
// 		SetTokenMemo("fnord").
// 		SetDecimals(3).
// 		SetInitialSupply(1000000).
// 		SetTreasuryAccountID(env.Client.GetOperatorAccountID()).
// 		SetAdminKey(env.Client.GetOperatorPublicKey()).
// 		SetFreezeKey(env.Client.GetOperatorPublicKey()).
// 		SetWipeKey(env.Client.GetOperatorPublicKey()).
// 		SetKycKey(env.Client.GetOperatorPublicKey()).
// 		SetSupplyKey(env.Client.GetOperatorPublicKey()).
// 		SetCustomFees([]Fee{
// 			NewCustomFixedFee().
// 				SetFeeCollectorAccountID(env.OperatorID).
// 				SetAmount(10),
// 			NewCustomFractionalFee().
// 				SetFeeCollectorAccountID(env.OperatorID).
// 				SetNumerator(1).
// 				SetDenominator(20).
// 				SetMin(1).
// 				SetMax(10),
// 		}).
// 		SetFreezeDefault(false).
// 		Execute(env.Client)
// 	require.NoError(t, err)

// 	receipt, err := resp.GetReceipt(env.Client)
// 	require.NoError(t, err)

// 	err = CloseIntegrationTestEnv(env, receipt.TokenID)
// 	require.NoError(t, err)
// }

// func TestIntegrationTokenCreateTransactionWithCustomFeesDenominatorZero(t *testing.T) {
// 	env := NewIntegrationTestEnv(t)

// 	resp, err := NewTokenCreateTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenName("ffff").
// 		SetTokenSymbol("F").
// 		SetTokenMemo("fnord").
// 		SetDecimals(3).
// 		SetInitialSupply(1000000).
// 		SetTreasuryAccountID(env.Client.GetOperatorAccountID()).
// 		SetAdminKey(env.Client.GetOperatorPublicKey()).
// 		SetFreezeKey(env.Client.GetOperatorPublicKey()).
// 		SetWipeKey(env.Client.GetOperatorPublicKey()).
// 		SetKycKey(env.Client.GetOperatorPublicKey()).
// 		SetSupplyKey(env.Client.GetOperatorPublicKey()).
// 		SetCustomFees([]Fee{
// 			CustomFixedFee{
// 				CustomFee: CustomFee{
// 					FeeCollectorAccountID: &env.OperatorID,
// 				},
// 				Amount: 10,
// 			},
// 			CustomFractionalFee{
// 				CustomFee: CustomFee{
// 					FeeCollectorAccountID: &env.OperatorID,
// 				},
// 				Numerator:     1,
// 				Denominator:   0,
// 				MinimumAmount: 1,
// 				MaximumAmount: 10,
// 			},
// 		}).
// 		SetFreezeDefault(false).
// 		Execute(env.Client)
// 	require.NoError(t, err)

// 	_, err = resp.GetReceipt(env.Client)
// 	assert.Error(t, err)
// 	if err != nil {
// 		assert.Equal(t, "exceptional receipt status: FRACTION_DIVIDES_BY_ZERO", err.Error())
// 	}
// }

// func TestIntegrationTokenCreateTransactionWithInvalidFeeCollectorAccountID(t *testing.T) {
// 	env := NewIntegrationTestEnv(t)

// 	resp, err := NewTokenCreateTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenName("ffff").
// 		SetTokenSymbol("F").
// 		SetTokenMemo("fnord").
// 		SetDecimals(3).
// 		SetInitialSupply(1000000).
// 		SetTreasuryAccountID(env.Client.GetOperatorAccountID()).
// 		SetAdminKey(env.Client.GetOperatorPublicKey()).
// 		SetFreezeKey(env.Client.GetOperatorPublicKey()).
// 		SetWipeKey(env.Client.GetOperatorPublicKey()).
// 		SetKycKey(env.Client.GetOperatorPublicKey()).
// 		SetSupplyKey(env.Client.GetOperatorPublicKey()).
// 		SetCustomFees([]Fee{
// 			NewCustomFractionalFee().
// 				SetFeeCollectorAccountID(AccountID{}).
// 				SetNumerator(1).
// 				SetDenominator(20).
// 				SetMin(1).
// 				SetMax(10),
// 		}).
// 		SetFreezeDefault(false).
// 		Execute(env.Client)
// 	require.NoError(t, err)

// 	receipt, err := resp.GetReceipt(env.Client)
// 	assert.Error(t, err)
// 	if err != nil {
// 		assert.Equal(t, "exceptional receipt status: INVALID_CUSTOM_FEE_COLLECTOR", err.Error())
// 	}

// 	err = CloseIntegrationTestEnv(env, receipt.TokenID)
// 	require.NoError(t, err)
// }

// func TestIntegrationTokenCreateTransactionWithMaxLessThanMin(t *testing.T) {
// 	env := NewIntegrationTestEnv(t)

// 	resp, err := NewTokenCreateTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenName("ffff").
// 		SetTokenSymbol("F").
// 		SetTokenMemo("fnord").
// 		SetDecimals(3).
// 		SetInitialSupply(1000000).
// 		SetTreasuryAccountID(env.Client.GetOperatorAccountID()).
// 		SetAdminKey(env.Client.GetOperatorPublicKey()).
// 		SetFreezeKey(env.Client.GetOperatorPublicKey()).
// 		SetWipeKey(env.Client.GetOperatorPublicKey()).
// 		SetKycKey(env.Client.GetOperatorPublicKey()).
// 		SetSupplyKey(env.Client.GetOperatorPublicKey()).
// 		SetCustomFees([]Fee{
// 			CustomFractionalFee{
// 				CustomFee: CustomFee{
// 					FeeCollectorAccountID: &env.OperatorID,
// 				},
// 				Numerator:     1,
// 				Denominator:   20,
// 				MinimumAmount: 100,
// 				MaximumAmount: 10,
// 			},
// 		}).
// 		SetFreezeDefault(false).
// 		Execute(env.Client)
// 	require.NoError(t, err)

// 	receipt, err := resp.GetReceipt(env.Client)
// 	assert.Error(t, err)
// 	if err != nil {
// 		assert.Equal(t, "exceptional receipt status: FRACTIONAL_FEE_MAX_AMOUNT_LESS_THAN_MIN_AMOUNT", err.Error())
// 	}

// 	err = CloseIntegrationTestEnv(env, receipt.TokenID)
// 	require.NoError(t, err)
// }

// func TestIntegrationTokenCreateTransactionWithRoyaltyCustomFee(t *testing.T) {
// 	env := NewIntegrationTestEnv(t)

// 	resp, err := NewTokenCreateTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenName("ffff").
// 		SetTokenSymbol("F").
// 		SetTokenMemo("fnord").
// 		SetTokenType(TokenTypeNonFungibleUnique).
// 		SetTreasuryAccountID(env.Client.GetOperatorAccountID()).
// 		SetAdminKey(env.Client.GetOperatorPublicKey()).
// 		SetFreezeKey(env.Client.GetOperatorPublicKey()).
// 		SetWipeKey(env.Client.GetOperatorPublicKey()).
// 		SetKycKey(env.Client.GetOperatorPublicKey()).
// 		SetSupplyKey(env.Client.GetOperatorPublicKey()).
// 		SetCustomFees([]Fee{
// 			NewCustomRoyaltyFee().
// 				SetFeeCollectorAccountID(env.OperatorID).
// 				SetNumerator(1).
// 				SetDenominator(20).
// 				SetFallbackFee(
// 					NewCustomFixedFee().
// 						SetFeeCollectorAccountID(env.OperatorID).
// 						SetAmount(10),
// 				),
// 		}).
// 		SetFreezeDefault(false).
// 		Execute(env.Client)
// 	require.NoError(t, err)

// 	receipt, err := resp.GetReceipt(env.Client)
// 	require.NoError(t, err)

// 	err = CloseIntegrationTestEnv(env, receipt.TokenID)
// 	require.NoError(t, err)
// }

// func TestIntegrationTokenCreateTransactionWithRoyaltyCannotExceedOne(t *testing.T) {
// 	env := NewIntegrationTestEnv(t)

// 	resp, err := NewTokenCreateTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenName("ffff").
// 		SetTokenSymbol("F").
// 		SetTokenMemo("fnord").
// 		SetTokenType(TokenTypeNonFungibleUnique).
// 		SetTreasuryAccountID(env.Client.GetOperatorAccountID()).
// 		SetAdminKey(env.Client.GetOperatorPublicKey()).
// 		SetFreezeKey(env.Client.GetOperatorPublicKey()).
// 		SetWipeKey(env.Client.GetOperatorPublicKey()).
// 		SetKycKey(env.Client.GetOperatorPublicKey()).
// 		SetSupplyKey(env.Client.GetOperatorPublicKey()).
// 		SetCustomFees([]Fee{
// 			NewCustomRoyaltyFee().
// 				SetFeeCollectorAccountID(env.OperatorID).
// 				SetNumerator(2).
// 				SetDenominator(1),
// 		}).
// 		SetFreezeDefault(false).
// 		Execute(env.Client)
// 	require.NoError(t, err)

// 	receipt, err := resp.GetReceipt(env.Client)
// 	assert.Error(t, err)
// 	if err != nil {
// 		assert.Equal(t, "exceptional receipt status: ROYALTY_FRACTION_CANNOT_EXCEED_ONE", err.Error())
// 	}

// 	err = CloseIntegrationTestEnv(env, receipt.TokenID)
// 	require.NoError(t, err)
// }

// func TestIntegrationTokenCreateTransactionFeeCollectorMissing(t *testing.T) {
// 	env := NewIntegrationTestEnv(t)

// 	resp, err := NewTokenCreateTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenName("ffff").
// 		SetTokenSymbol("F").
// 		SetTokenMemo("fnord").
// 		SetTokenType(TokenTypeNonFungibleUnique).
// 		SetTreasuryAccountID(env.Client.GetOperatorAccountID()).
// 		SetAdminKey(env.Client.GetOperatorPublicKey()).
// 		SetFreezeKey(env.Client.GetOperatorPublicKey()).
// 		SetWipeKey(env.Client.GetOperatorPublicKey()).
// 		SetKycKey(env.Client.GetOperatorPublicKey()).
// 		SetSupplyKey(env.Client.GetOperatorPublicKey()).
// 		SetCustomFees([]Fee{
// 			NewCustomRoyaltyFee().
// 				SetNumerator(1).
// 				SetDenominator(20),
// 		}).
// 		SetFreezeDefault(false).
// 		Execute(env.Client)
// 	require.NoError(t, err)

// 	receipt, err := resp.GetReceipt(env.Client)
// 	assert.Error(t, err)
// 	if err != nil {
// 		assert.Equal(t, "exceptional receipt status: INVALID_CUSTOM_FEE_COLLECTOR", err.Error())
// 	}

// 	err = CloseIntegrationTestEnv(env, receipt.TokenID)
// 	require.NoError(t, err)
// }

// func TestIntegrationTokenCreateTransactionRoyaltyFeeOnlyAllowedForNonFungibleUnique(t *testing.T) {
// 	env := NewIntegrationTestEnv(t)

// 	resp, err := NewTokenCreateTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenName("ffff").
// 		SetTokenSymbol("F").
// 		SetTokenMemo("fnord").
// 		SetTokenType(TokenTypeFungibleCommon).
// 		SetTreasuryAccountID(env.Client.GetOperatorAccountID()).
// 		SetAdminKey(env.Client.GetOperatorPublicKey()).
// 		SetFreezeKey(env.Client.GetOperatorPublicKey()).
// 		SetWipeKey(env.Client.GetOperatorPublicKey()).
// 		SetKycKey(env.Client.GetOperatorPublicKey()).
// 		SetSupplyKey(env.Client.GetOperatorPublicKey()).
// 		SetCustomFees([]Fee{
// 			NewCustomRoyaltyFee().
// 				SetFeeCollectorAccountID(env.OperatorID).
// 				SetNumerator(1).
// 				SetDenominator(20),
// 		}).
// 		SetFreezeDefault(false).
// 		Execute(env.Client)
// 	require.NoError(t, err)

// 	receipt, err := resp.GetReceipt(env.Client)
// 	assert.Error(t, err)
// 	if err != nil {
// 		assert.Equal(t, "exceptional receipt status: CUSTOM_ROYALTY_FEE_ONLY_ALLOWED_FOR_NON_FUNGIBLE_UNIQUE", err.Error())
// 	}

// 	err = CloseIntegrationTestEnv(env, receipt.TokenID)
// 	require.NoError(t, err)
// }

// func TestIntegrationTokenAccountStillOwnsNfts(t *testing.T) {
// 	env := NewIntegrationTestEnv(t)

// 	newKey, err := PrivateKeyGenerateEd25519()
// 	require.NoError(t, err)

// 	newBalance := NewHbar(2)

// 	assert.Equal(t, 2*HbarUnits.Hbar._NumberOfTinybar(), newBalance.tinybar)

// 	resp, err := NewAccountCreateTransaction().
// 		SetKey(newKey).
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetInitialBalance(newBalance).
// 		Execute(env.Client)
// 	require.NoError(t, err)

// 	receipt, err := resp.GetReceipt(env.Client)
// 	require.NoError(t, err)

// 	accountID := *receipt.AccountID

// 	tokTx, err := NewTokenCreateTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenName("ffff").
// 		SetTokenSymbol("F").
// 		SetTokenMemo("fnord").
// 		SetTokenType(TokenTypeNonFungibleUnique).
// 		SetSupplyType(TokenSupplyTypeFinite).
// 		SetMaxSupply(5).
// 		SetTreasuryAccountID(accountID).
// 		SetAdminKey(newKey.PublicKey()).
// 		SetFreezeKey(newKey.PublicKey()).
// 		SetWipeKey(newKey.PublicKey()).
// 		SetKycKey(newKey.PublicKey()).
// 		SetSupplyKey(newKey.PublicKey()).
// 		SetFreezeDefault(false).
// 		FreezeWith(env.Client)
// 	require.NoError(t, err)

// 	tokTx.Sign(newKey)

// 	resp, err = tokTx.Execute(env.Client)
// 	require.NoError(t, err)

// 	receipt, err = resp.GetReceipt(env.Client)
// 	require.NoError(t, err)

// 	tokenID := *receipt.TokenID
// 	metaData := make([]byte, 50, 101)

// 	mintTx, err := NewTokenMintTransaction().
// 		SetNodeAccountIDs([]AccountID{resp.NodeID}).
// 		SetTokenID(tokenID).
// 		SetMetadata(metaData).
// 		FreezeWith(env.Client)
// 	require.NoError(t, err)

// 	mintTx.Sign(newKey)

// 	mint, err := mintTx.Execute(env.Client)
// 	require.NoError(t, err)

// 	_, err = mint.GetReceipt(env.Client)
// 	require.NoError(t, err)

// 	deleteTx, err := NewTokenDeleteTransaction().
// 		SetNodeAccountIDs(env.NodeAccountIDs).
// 		SetTokenID(tokenID).
// 		FreezeWith(env.Client)
// 	require.NoError(t, err)

// 	deleteTx.Sign(newKey)

// 	resp, err = deleteTx.Execute(env.Client)
// 	require.NoError(t, err)

// 	_, err = resp.GetReceipt(env.Client)
// 	require.NoError(t, err)
// }
