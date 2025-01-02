use crate::helpers::init::init_test;
use crate::helpers::users::{clean_up_user_model, create_user_with_email, generate_unactivated_user};
use axum::http::StatusCode;
use financrr::app::App;
use financrr::controllers::user::{ForgotParams, ResetParams, VerifyParams};
use financrr::models::users;
use financrr::utils::context::AdditionalAppContextMethods;
use financrr::views::user::UserResponse;
use insta::{assert_debug_snapshot, assert_json_snapshot, with_settings};
use loco_rs::testing;
use serde_json::json;
use serial_test::serial;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("user_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn can_register() {
    init_test!();

    testing::request::<App, _, _>(|request, ctx| async move {
        assert!(ctx.is_mailer_enabled());
        const EMAIL: &str = "can.register@financrr.test";
        let payload = json!({
            "name": "TestUer",
            "email": EMAIL,
            "password": "Password123"
        });

        let response = request.post("/api/v1/users/register").json(&payload).await;
        assert_eq!(response.status_code(), StatusCode::CREATED);

        let user_response: UserResponse = response.json();

        let db_user = users::Model::find_by_email(&ctx.db, EMAIL).await.unwrap().unwrap();

        let mailer = ctx.mailer.unwrap();
        let deliveries = &mailer.deliveries();
        assert_eq!(deliveries.count, 1, "Exactly one email should be sent");

        with_settings!({
            filters => clean_up_user_model()
        }, {
            assert_debug_snapshot!(db_user);
        });

        with_settings!({
            filters => clean_up_user_model(),
        }, {
            assert_json_snapshot!(user_response);
        });
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn can_verify() {
    init_test!();

    testing::request::<App, _, _>(|request, ctx| async move {
        assert!(ctx.is_mailer_enabled());
        let user = generate_unactivated_user(&ctx).await;
        let token = user.email_verification_token.clone().unwrap();

        let payload = VerifyParams {
            email: user.email.clone(),
            token,
        };

        let response = request.post("/api/v1/users/verify").form(&payload).await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let db_user = users::Model::find_by_email(&ctx.db, &user.email)
            .await
            .unwrap()
            .unwrap();

        assert!(db_user.email_verified_at.is_some());
        assert!(db_user.email_verification_token.is_none());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn can_reset_password() {
    init_test!();

    testing::request::<App, _, _>(|request, ctx| async move {
        assert!(ctx.is_mailer_enabled());
        const EMAIL: &str = "can.reset.password@financrr.test";
        let user = create_user_with_email(&ctx, EMAIL).await;

        let payload = ForgotParams {
            email: user.email.clone(),
        };

        let response = request.post("/api/v1/users/forgot").form(&payload).await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(ctx.mailer.unwrap().deliveries().count, 1);

        let db_user = users::Model::find_by_email(&ctx.db, &user.email)
            .await
            .unwrap()
            .unwrap();

        assert!(db_user.reset_token.is_some());

        const NEW_PASSWORD: &str = "NewPassword123";
        let payload = ResetParams {
            email: user.email.clone(),
            token: db_user.reset_token.unwrap(),
            password: NEW_PASSWORD.to_string(),
        };
        let response = request.post("/api/v1/users/reset").form(&payload).await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let user_response: UserResponse = response.json();

        with_settings!({
            filters => clean_up_user_model(),
        }, {
            assert_json_snapshot!(user_response);
        });

        let db_user = users::Model::find_by_email(&ctx.db, &user.email)
            .await
            .unwrap()
            .unwrap();

        assert!(db_user.reset_token.is_none());

        assert!(db_user.verify_password(NEW_PASSWORD));
    })
    .await;
}
