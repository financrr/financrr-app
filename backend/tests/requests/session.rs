use crate::helpers::init::init_test;
use crate::helpers::session::clean_up_session_response;
use crate::helpers::users::{create_unverified_user_with_email, create_user_with_email, DEFAULT_PASSWORD};
use axum::http::StatusCode;
use financrr::app::App;
use financrr::views::session::SessionResponse;
use insta::{assert_json_snapshot, with_settings};
use loco_rs::testing;
use rstest::rstest;
use sea_orm::IntoActiveModel;
use serde_json::json;
use serial_test::serial;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("sessions_request");
        let _guard = settings.bind_to_scope();
    };
}

#[rstest]
#[case("can_login_with_valid_password", DEFAULT_PASSWORD, StatusCode::CREATED)]
#[case("can_login_with_invalid_password", "invalid-password", StatusCode::BAD_REQUEST)]
#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn can_login(#[case] test_name: &str, #[case] password: &str, #[case] expected_status_code: StatusCode) {
    init_test!();

    testing::request::<App, _, _>(|request, ctx| async move {
        const EMAIL: &str = "can.login@financrr.test";
        let _ = create_user_with_email(&ctx, EMAIL).await;

        let payload = json!({
            "email": EMAIL,
            "password": password,
        });
        let response = request.post("/api/v1/sessions").json(&payload).await;
        assert_eq!(response.status_code(), expected_status_code);

        if StatusCode::CREATED == expected_status_code {
            let session_response: SessionResponse = response.json();

            with_settings!({
                filters => clean_up_session_response()
            }, {
                assert_json_snapshot!(test_name, session_response);
            })
        }
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn can_login_without_verify() {
    init_test!();

    testing::request::<App, _, _>(|request, ctx| async move {
        const EMAIL: &str = "can.login.without.verify@financrr.test";
        let user = create_unverified_user_with_email(&ctx, EMAIL).await;

        let payload = json!({
            "email": EMAIL,
            "password": DEFAULT_PASSWORD,
        });
        let response = request.post("/api/v1/sessions").json(&payload).await;
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        let _ = user.into_active_model().verified(&ctx.db).await.unwrap();
        let response = request.post("/api/v1/sessions").json(&payload).await;
        assert_eq!(response.status_code(), StatusCode::CREATED);

        let session_response: SessionResponse = response.json();
        with_settings!({
            filters => clean_up_session_response()
        }, {
            assert_json_snapshot!("can_login_without_verify", session_response);
        });
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn can_get_current_session() {
    init_test!();

    testing::request::<App, _, _>(|request, ctx| async move {
        const EMAIL: &str = "can.get.current.session@financrr.test";
        let user = create_user_with_email(&ctx, EMAIL).await;

        let payload = json!({
            "email": user.email,
            "password": DEFAULT_PASSWORD,
        });
        let response = request.post("/api/v1/sessions").json(&payload).await;
        assert_eq!(response.status_code(), StatusCode::CREATED);

        let session_response: SessionResponse = response.json();
        let response = request
            .get("/api/v1/sessions/current")
            .add_header("Authorization", format!("Bearer {}", session_response.api_key))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let session_response: SessionResponse = response.json();
        with_settings!({
            filters => clean_up_session_response()
        }, {
            assert_json_snapshot!("can_get_current_user", session_response);
        });
    })
    .await;
}
