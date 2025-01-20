use crate::helpers::init::load_envs;
use axum::http::StatusCode;
use financrr::app::App;
use loco_rs::prelude::request;
use serial_test::serial;

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_ignore_trailing_slash() {
    load_envs();

    request::<App, _, _>(|request, _| async move {
        let response = request.get("/api/status/version").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let response = request.get("/api/status/version/").await;

        assert_eq!(response.status_code(), StatusCode::OK);
    })
    .await;
}
