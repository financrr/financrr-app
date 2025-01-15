use financrr::app::App;
use loco_rs::{task, testing::prelude::*};

use loco_rs::boot::run_task;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_can_run_sync_go_cardless_institutions() {
    let boot = boot_test::<App>().await.unwrap();

    assert!(run_task::<App>(
        &boot.app_context,
        Some(&"SyncGoCardlessInstitutions".to_string()),
        &task::Vars::default()
    )
    .await
    .is_ok());
}
