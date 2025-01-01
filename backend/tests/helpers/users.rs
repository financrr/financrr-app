use crate::helpers::faker::generate_random_email;
use financrr::controllers::user::RegisterParams;
use financrr::models::users;
use financrr::models::users::Model;
use financrr::services::snowflake_generator::SnowflakeGeneratorInner;
use financrr::services::Service;
use loco_rs::app::AppContext;

pub async fn create_user_with_email_and_password(ctx: &AppContext, email: &str, password: &str) -> Model {
    let snowflake_generator = SnowflakeGeneratorInner::get_arc(ctx).await.unwrap();
    let register_params = RegisterParams {
        email: email.to_string(),
        password: password.to_string(),
        name: "Test Account".to_string(),
    };

    users::Model::create_with_password(&ctx.db, &snowflake_generator, &register_params)
        .await
        .unwrap()
}

pub async fn create_user_with_email(ctx: &AppContext, email: &str) -> Model {
    create_user_with_email_and_password(ctx, email, "Password1234").await
}

pub async fn create_user_with_password(ctx: &AppContext, password: &str) -> Model {
    create_user_with_email_and_password(ctx, &generate_random_email(), password).await
}

pub async fn generate_test_user(ctx: &AppContext) -> Model {
    create_user_with_email(ctx, &generate_random_email()).await
}

pub fn clean_up_user_model() -> Vec<(&'static str, &'static str)> {
    let mut combined_filters = Vec::new();
    combined_filters.extend(replace_dates().iter().copied());
    combined_filters.extend(replace_id().iter().copied());
    combined_filters.extend(replace_password().iter().copied());

    combined_filters
}

pub fn replace_id() -> Vec<(&'static str, &'static str)> {
    vec![(r"id: \d+,", "id: ID")]
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
