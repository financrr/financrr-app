use crate::helpers::users::clean_up_user_model;
use financrr::controllers::session::CreateSessionParams;
use financrr::models::_entities::sessions;
use financrr::models::users;
use financrr::services::secret_generator::SecretGeneratorInner;
use financrr::services::snowflake_generator::SnowflakeGeneratorInner;
use financrr::services::Service;
use loco_rs::app::AppContext;

pub async fn generate_session(ctx: &AppContext, user: &users::Model, password: &str) -> sessions::Model {
    let snowflake_generator = SnowflakeGeneratorInner::get_arc(ctx).await.unwrap();
    let secret_generator = SecretGeneratorInner::get_arc(ctx).await.unwrap();
    let params = CreateSessionParams {
        email: user.email.clone(),
        password: password.to_string(),
        name: None,
        user_agent: None,
    };

    sessions::Model::create_session(&ctx.db, &snowflake_generator, &secret_generator, user, &params)
        .await
        .unwrap()
}

pub fn clean_up_session_response() -> Vec<(&'static str, &'static str)> {
    let mut combined_filters = Vec::new();
    combined_filters.extend(clean_up_user_model().iter().copied());
    combined_filters.extend(replace_api_key().iter().copied());

    combined_filters
}

pub fn replace_api_key() -> Vec<(&'static str, &'static str)> {
    vec![
        (r#""api_key": ".*?""#, "\"api_key\": KEY"),
        (r#"api_key: ".*?""#, "api_key: KEY"),
    ]
}
