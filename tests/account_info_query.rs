mod utils;
use tracing_subscriber;
use hedera_rust_client::AccountInfoQuery;

#[tokio::test]
#[ignore]
async fn test_account_get_info() {
    tracing_subscriber::fmt::init();
    let env = utils::IntegrationTestEnv::open().await.unwrap();
    let info = AccountInfoQuery::new()
        .set_account_id(env.operator_id).unwrap()
        .execute(&env.client)
        .await.unwrap();

    assert_eq!(info.account_id, env.operator_id);

    env.close().await.unwrap();
}
