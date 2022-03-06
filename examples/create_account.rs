use std::env;
use dotenv::dotenv;
use hedera_rust_client::{
    AccountId, ClientBuilder, NetworkName, Operator, PrivateKey,
    Hbar, HederaError, AccountCreateTransaction,
};

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

    // Create simple Account
    let (new_key, _) = PrivateKey::generate("key-password");
    let initial_balance = Hbar::new(2.0);
    let resp = AccountCreateTransaction::new()
        .set_key(new_key.clone().into())?
        .set_initial_balance(initial_balance)?
        .execute(&client)
        .await?;

    let receipt = resp.get_receipt(&client).await?;

    println!("Returned:\n{:#?}", receipt);

    Ok(())
}
