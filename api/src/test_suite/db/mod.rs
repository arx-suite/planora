use super::tracing_init;

mod setup;

#[tokio::test]
async fn main() {
    tracing_init();
    let test_utils = setup::TestDb::new().await.unwrap();

    test_utils.teardown().await.unwrap();
}
