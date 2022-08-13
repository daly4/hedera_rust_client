use dotenv::dotenv;
use hedera_rust_client::{
    AccountCreateTransaction, AccountId, ClientBuilder, FreezeDefault, Hbar, HederaError, Key,
    NetworkName, Operator, PrivateKey, TokenAssociateTransaction, TokenBurnTransaction,
    TokenCreateTransaction, TokenDeleteTransaction, TokenFreezeTransaction,
    TokenGrantKycTransaction, TokenWipeTransaction, TransferTransaction,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), HederaError> {
    // Setup Client from .env file for testnet
    dotenv().ok();

    let private_key: PrivateKey = match env::var("PRIVATE_KEY") {
        Ok(v) => v.parse().unwrap(),
        Err(err) => panic!("env PRIVATE_KEY: {}", err),
    };
    let account_id: AccountId = match env::var("ACCOUNT_ID") {
        Ok(v) => v.parse().unwrap(),
        Err(err) => panic!("env ACCOUNT_ID: {}", err),
    };

    // Build client
    let operator = Operator::new(account_id, private_key);
    let client = ClientBuilder::default()
        .operator(operator)
        .for_network_name(NetworkName::TestNet)?
        .build()?;

    // Create simple Account to transfer to
    let (new_pk, _) = PrivateKey::generate("key-password");
    let initial_balance = Hbar::new(2.0);
    let receipt = AccountCreateTransaction::new()
        .set_key(new_pk.clone().into())?
        .set_initial_balance(initial_balance)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    // get the account id in the receipt
    let to_account_id = receipt.account_id.unwrap();

    // grab the operator key and account id
    let operator_id = client.operator_account_id();
    let operator_key: Key = client.operator_public_key().into();

    let amount = 1000000u64;
    let transfer_amount = 1000u64;
    let post_transfer_amount = amount - transfer_amount;

    // create a token w/ all keys set
    let tx = TokenCreateTransaction::new()
        .set_name("ffff".to_string())?
        .set_symbol("F".to_string())?
        .set_memo("fnord".to_string())?
        .set_decimals(3)?
        .set_initial_supply(amount)?
        .set_treasury(operator_id)?
        .set_auto_renew_account(operator_id)?
        .set_admin_key(operator_key.clone())?
        .set_freeze_key(operator_key.clone())?
        .set_wipe_key(operator_key.clone())?
        .set_kyc_key(operator_key.clone())?
        .set_supply_key(operator_key.clone().into())?
        .set_freeze_default(FreezeDefault::Unfrozen)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let token_id = tx.token_id.unwrap();

    // associate new account w/ token
    let _tx = TokenAssociateTransaction::new()
        .set_account_id(to_account_id)?
        .set_tokens(vec![token_id])?
        .freeze_with(Some(&client))
        .await?
        .sign(&new_pk)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    // grant kyc to new account
    let _tx = TokenGrantKycTransaction::new()
        .set_token_id(token_id)?
        .set_account_id(to_account_id)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    // transfer token to new account
    let tx_amt = transfer_amount as i64;
    let _tx = TransferTransaction::new()
        .add_token_transfer(token_id, operator_id, -tx_amt, None)?
        .add_token_transfer(token_id, to_account_id, tx_amt, None)?
        .execute(&client)
        .await?;

    // freeze token for new account
    let _tx = TokenFreezeTransaction::new()
        .set_token_id(token_id)?
        .set_account_id(to_account_id)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    // wipe token from new account
    let _tx = TokenWipeTransaction::new()
        .set_token_id(token_id)?
        .set_account_id(to_account_id)?
        .set_amount(transfer_amount)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    // burn token from treasury
    let _tx = TokenBurnTransaction::new()
        .set_token_id(token_id)?
        .set_amount(post_transfer_amount)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    // delete token
    let _res = TokenDeleteTransaction::new()
        .set_token_id(token_id)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    Ok(())
}
