use crate::helpers::init::init_test;
use axum::http::StatusCode;
use financrr::app::App;
use loco_rs::testing;
use serial_test::serial;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("openapi_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn can_get_openapi_json() {
    init_test!();

    testing::request::<App, _, _>(|request, _| async move {
        let response = request.get("/api/v1/openapi.json").await;

        assert_eq!(response.status_code(), StatusCode::OK)
    })
    .await
}

#[tokio::test]
#[serial]
async fn can_get_openapi_yaml() {
    init_test!();

    testing::request::<App, _, _>(|request, _| async move {
        let response = request.get("/api/v1/openapi.yaml").await;

        assert_eq!(response.status_code(), StatusCode::OK)
    })
    .await
}
