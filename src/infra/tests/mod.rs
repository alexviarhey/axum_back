#[allow(dead_code)]

pub async fn run_test_with_db_drop<F: std::future::Future>(test: F) -> F::Output {
    std::env::set_var("APP_MODE", "test");

    let state = crate::run_infrastructure().await;

    state
        .db
        .drop(None)
        .await
        .expect("Cannot drop db after test execution!");

    test.await
}
