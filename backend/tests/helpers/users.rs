use crate::helpers::faker::generate_random_email;
use financrr::controllers::user::RegisterParams;
use financrr::models::users;
use financrr::models::users::Model;
use financrr::services::snowflake_generator::SnowflakeGeneratorInner;
use financrr::services::user_verification::UserVerificationServiceInner;
use financrr::services::Service;
use loco_rs::app::AppContext;
use sea_orm::IntoActiveModel;

pub const DEFAULT_PASSWORD: &str = "Password1234";
pub const DEFAULT_NAME: &str = "Test Account";

pub async fn create_user(ctx: &AppContext, email: &str, password: &str, activate: bool) -> Model {
    let snowflake_generator = SnowflakeGeneratorInner::get_arc(ctx).await.unwrap();
    let register_params = RegisterParams {
        email: email.to_string(),
        password: password.to_string(),
        name: DEFAULT_NAME.to_string(),
    };

    let user = users::Model::create_with_password(&ctx.db, &snowflake_generator, &register_params)
        .await
        .unwrap();

    let user = match activate {
        true => user.into_active_model().verified(&ctx.db).await.unwrap(),
        false => {
            let verification_service = UserVerificationServiceInner::get_arc(ctx).await.unwrap();

            verification_service
                .send_verification_email_or_verify_user(user.into_active_model())
                .await
                .unwrap()
        }
    };

    user
}

pub async fn create_user_with_email(ctx: &AppContext, email: &str) -> Model {
    create_user(ctx, email, DEFAULT_PASSWORD, true).await
}

pub async fn create_unverified_user_with_email(ctx: &AppContext, email: &str) -> Model {
    create_user(ctx, email, DEFAULT_PASSWORD, false).await
}

pub async fn create_user_with_password(ctx: &AppContext, password: &str) -> Model {
    create_user(ctx, &generate_random_email(), password, true).await
}

pub async fn generate_test_user(ctx: &AppContext) -> Model {
    create_user_with_email(ctx, &generate_random_email()).await
}

pub async fn generate_activated_user(ctx: &AppContext) -> Model {
    create_user(ctx, &generate_random_email(), DEFAULT_PASSWORD, true).await
}

pub async fn generate_unactivated_user(ctx: &AppContext) -> Model {
    create_user(ctx, &generate_random_email(), DEFAULT_PASSWORD, false).await
}

pub fn clean_up_user_model() -> Vec<(&'static str, &'static str)> {
    let mut combined_filters = Vec::new();
    combined_filters.extend(replace_dates().iter().copied());
    combined_filters.extend(replace_id().iter().copied());
    combined_filters.extend(replace_password().iter().copied());
    combined_filters.extend(replace_email_verification_token().iter().copied());

    combined_filters
}

pub fn replace_id() -> Vec<(&'static str, &'static str)> {
    vec![(r"id: \d+", "id: ID"), (r#""id": "\d+""#, "\"id\": ID")]
}

pub fn replace_dates() -> Vec<(&'static str, &'static str)> {
    vec![
        (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?\+\d{2}:\d{2}", "DATE"), // with tz
        (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+", "DATE"),
        (r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})", "DATE"),
    ]
}

pub fn replace_password() -> Vec<(&'static str, &'static str)> {
    vec![(r"password: (.*{60}),", "password: \"PASSWORD\",")]
}

pub fn replace_email_verification_token() -> Vec<(&'static str, &'static str)> {
    let regex = r#"email_verification_token: Some\(\n\s*\".{8}\",\n\s*\),"#;
    vec![(regex, "email_verification_token: Some(\n        \"TOKEN\",\n    ),")]
}
