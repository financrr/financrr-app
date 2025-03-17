use financrr::app::App;
use loco_rs::{task, testing::prelude::*};

use crate::helpers::init::load_envs;
use loco_rs::boot::run_task;
use serial_test::serial;

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_can_run_sync_institutions() {
    load_envs();

    let boot = boot_test::<App>().await.unwrap();

    assert!(
        run_task::<App>(
            &boot.app_context,
            Some(&"SyncInstitutions".to_string()),
            &task::Vars::default()
        )
        .await
        .is_ok()
    );
}
