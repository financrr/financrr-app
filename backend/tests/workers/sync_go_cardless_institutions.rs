use crate::helpers::init::init_test;
use axum::http::header::CONTENT_TYPE;
use financrr::{
    app::App,
    workers::sync_go_cardless_institutions::{SyncGoCardlessInstitutionsWorker, WorkerArgs},
};
use insta::assert_debug_snapshot;
use loco_rs::{bgworker::BackgroundWorker, testing::prelude::*};
use serde_json::json;
use serial_test::serial;
use std::env::set_var;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("sync_go_cardless_institutions.rs");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_cant_run_on_not_configured_client() {
    init_test!();
    set_var("GO_CARDLESS_ENABLED", "false");

    let boot = boot_test::<App>().await.unwrap();

    let rs = SyncGoCardlessInstitutionsWorker::perform_later(&boot.app_context, WorkerArgs).await;

    assert!(rs.is_err());
    assert_debug_snapshot!(rs.err().unwrap());
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_run_sync_go_cardless_institutions_worker() {
    init_test!();
    const INSTITUTIONS: &str = include_str!("go_cardless_institutions.json");

    let mut server = mockito::Server::new_async().await;
    let addr = server.url();
    set_var("GO_CARDLESS_ENABLED", "true");
    set_var("GO_CARDLESS_API_URL", addr);

    let token_mock = server
        .mock("POST", "/api/v2/token/new/")
        .expect(1)
        .with_header(CONTENT_TYPE, "application/json")
        .with_body(
            json!({
                "access": "access_token",
                "access_expires": 3600,
                "refresh": "refresh_token",
                "refresh_expires": 3600
            })
            .to_string(),
        )
        .create_async()
        .await;

    let institutions_mock = server
        .mock("GET", "/api/v2/institutions/")
        .expect(1)
        .with_header(CONTENT_TYPE, "application/json")
        .with_body(INSTITUTIONS)
        .create_async()
        .await;

    let boot = boot_test::<App>().await.unwrap();

    assert!(
        SyncGoCardlessInstitutionsWorker::perform_later(&boot.app_context, WorkerArgs)
            .await
            .is_ok()
    );

    token_mock.assert_async().await;
    institutions_mock.assert_async().await;
}
