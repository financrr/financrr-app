use financrr::app::App;
use loco_rs::prelude::*;
use loco_rs::testing;

use crate::helpers::init::load_envs;
use crate::helpers::session::generate_session;
use crate::helpers::users::{create_user_with_email, DEFAULT_PASSWORD};
use financrr::models::_entities::sessions;
use financrr::workers::session_used::SessionUsedWorker;
use financrr::workers::session_used::SessionUsedWorkerArgs;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_run_session_used_worker() {
    load_envs();

    let boot = testing::boot_test::<App>().await.unwrap();

    const EMAIL: &str = "run.session.used.worker@financrr.test";
    let user = create_user_with_email(&boot.app_context, EMAIL).await;
    let session = generate_session(&boot.app_context, &user, DEFAULT_PASSWORD).await;

    assert!(session.last_accessed_at.is_none());

    assert!(
        SessionUsedWorker::perform_later(&boot.app_context, SessionUsedWorkerArgs { session_id: session.id })
            .await
            .is_ok()
    );

    let session = sessions::Model::find_by_id(&boot.app_context.db, session.id)
        .await
        .unwrap()
        .unwrap();

    assert!(session.last_accessed_at.is_some());
}
