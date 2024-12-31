use crate::helpers::init::init_test;
use crate::helpers::users::{clean_up_user_model, create_user_with_email, generate_test_user};
use financrr::controllers::user::RegisterParams;
use financrr::error::app_error::AppResult;
use financrr::services::snowflake_generator::SnowflakeGeneratorInner;
use financrr::services::Service;
use financrr::{app::App, models::users::Model};
use insta::assert_debug_snapshot;
use loco_rs::testing;
use sea_orm::IntoActiveModel;
use serial_test::serial;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("users");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn can_create_with_password() {
    init_test!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let params = RegisterParams {
        email: "test@financrr.dev".to_string(),
        password: "Password1234".to_string(),
        name: "Test Account".to_string(),
    };
    let snowflake_generator = SnowflakeGeneratorInner::new_arc(&boot.app_context).await.unwrap();

    let res = Model::create_with_password(&boot.app_context.db, &snowflake_generator, &params).await;

    insta::with_settings!({
        filters => clean_up_user_model()
    }, {
        assert_debug_snapshot!(res);
    });
}

#[tokio::test]
#[serial]
async fn handle_create_with_password_with_duplicate() {
    init_test!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let snowflake_generator = SnowflakeGeneratorInner::get_arc(&boot.app_context).await.unwrap();
    let _ = create_user_with_email(&boot.app_context, "duplicate_user@financrr.test").await;

    let new_user: AppResult<Model> = Model::create_with_password(
        &boot.app_context.db,
        &snowflake_generator,
        &RegisterParams {
            email: "duplicate_user@financrr.test".to_string(),
            password: "Password1234".to_string(),
            name: "Test Account".to_string(),
        },
    )
    .await;

    insta::with_settings!({
        filters => clean_up_user_model()
    }, {
        assert_debug_snapshot!(new_user);
    });
}

#[tokio::test]
#[serial]
async fn can_find_by_email() {
    init_test!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let user = create_user_with_email(&boot.app_context, "user1@financrr.test").await;

    let existing_user = Model::find_by_email(&boot.app_context.db, user.email.as_str()).await;
    let non_existing_user_results = Model::find_by_email(&boot.app_context.db, "non-existing@financrr.test").await;

    assert_debug_snapshot!(existing_user);
    assert_debug_snapshot!(non_existing_user_results);
}

#[tokio::test]
#[serial]
async fn can_verification_token() {
    init_test!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let user = Model::find_by_id(&boot.app_context.db, 1).await.unwrap();

    assert!(user.email_verification_sent_at.is_none());
    assert!(user.email_verification_token.is_none());

    assert!(user
        .into_active_model()
        .set_email_verification_sent(&boot.app_context.db)
        .await
        .is_ok());

    let user = Model::find_by_id(&boot.app_context.db, 1).await.unwrap();

    assert!(user.email_verification_sent_at.is_some());
    assert!(user.email_verification_token.is_some());
}

#[tokio::test]
#[serial]
async fn can_set_forgot_password_sent() {
    init_test!();

    let boot = testing::boot_test::<App>().await.unwrap();
    let user = generate_test_user(&boot.app_context).await;
    let user_id = user.id;

    assert!(user.reset_sent_at.is_none());
    assert!(user.reset_token.is_none());

    assert!(user
        .into_active_model()
        .set_forgot_password_sent(&boot.app_context.db)
        .await
        .is_ok());

    let user = Model::find_by_id(&boot.app_context.db, user_id).await.unwrap();

    assert!(user.reset_sent_at.is_some());
    assert!(user.reset_token.is_some());
}

#[tokio::test]
#[serial]
async fn can_verified() {
    init_test!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let user = Model::find_by_id(&boot.app_context.db, 1).await.unwrap();

    assert!(user.email_verified_at.is_none());

    assert!(user.into_active_model().verified(&boot.app_context.db).await.is_ok());

    let user = Model::find_by_id(&boot.app_context.db, 1).await.unwrap();

    assert!(user.email_verified_at.is_some());
}

#[tokio::test]
#[serial]
async fn can_reset_password() {
    init_test!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let user = Model::find_by_id(&boot.app_context.db, 1).await.unwrap();

    assert!(user.verify_password("12341234"));

    assert!(user
        .clone()
        .into_active_model()
        .reset_password(&boot.app_context.db, "new-password")
        .await
        .is_ok());

    assert!(Model::find_by_id(&boot.app_context.db, 1)
        .await
        .unwrap()
        .verify_password("new-password"));
}
