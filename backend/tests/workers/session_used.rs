use financrr::app::App;
use loco_rs::prelude::*;
use loco_rs::testing;

use financrr::workers::session_used::SessionUsedWorker;
use financrr::workers::session_used::SessionUsedWorkerArgs;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_run_session_used_worker() {
    let boot = testing::boot_test::<App>().await.unwrap();

    // Execute the worker ensuring that it operates in 'ForegroundBlocking' mode, which prevents the addition of your worker to the background
    assert!(
        SessionUsedWorker::perform_later(&boot.app_context, SessionUsedWorkerArgs { session_id: 0 })
            .await
            .is_ok()
    );
    // Include additional assert validations after the execution of the worker
}
