// use crate::helpers::external_institutions::generate_institutions;
// use crate::helpers::init::init_test;
// use crate::helpers::session::generate_session;
// use crate::helpers::users::{DEFAULT_PASSWORD, generate_test_user};
// use axum::http::StatusCode;
// use financrr::app::App;
// use loco_rs::prelude::request;
// use serde_json::Value;
// use serial_test::serial;

// TODO fix this test
// macro_rules! configure_insta {
//     ($($expr:expr),*) => {
//         let mut settings = insta::Settings::clone_current();
//         settings.set_prepend_module_to_snapshot(false);
//         settings.set_snapshot_suffix("external_bank_institutions");
//         let _guard = settings.bind_to_scope();
//     };
// }
//
// #[tokio::test(flavor = "multi_thread")]
// #[serial]
// async fn is_not_public_available() {
//     init_test!();
//
//     request::<App, _, _>(|request, _ctx| async move {
//         let response = request.get("/api/v1/external_bank_institutions").await;
//         assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
//     })
//     .await;
// }
//
// #[tokio::test(flavor = "multi_thread")]
// #[serial]
// async fn can_accessed_by_user() {
//     init_test!();
//
//     request::<App, _, _>(|request, ctx| async move {
//         generate_institutions(&ctx, "test_provider", 5).await;
//
//         let user = generate_test_user(&ctx).await;
//         let session = generate_session(&ctx, &user, DEFAULT_PASSWORD).await;
//
//         let response = request
//             .get("/api/v1/external_bank_institutions")
//             .authorization_bearer(session.api_key)
//             .await;
//
//         assert_eq!(response.status_code(), StatusCode::OK);
//
//         let json: Value = response.json();
//         let results = json.get("results").unwrap();
//         let array = results.as_array().unwrap();
//         let len = array.len();
//         assert_eq!(len, 5);
//     })
//     .await;
// }
//TODO test search query
