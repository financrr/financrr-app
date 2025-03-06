use financrr::app::App;
use loco_rs::{task, testing::prelude::*};
use std::env::set_var;

use crate::helpers::init::load_envs;
use loco_rs::boot::run_task;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_can_run_check_health() {
    load_envs();

    let boot = boot_test::<App>().await.unwrap();

    assert!(
        run_task::<App>(&boot.app_context, Some(&"healthy".to_string()), &task::Vars::default())
            .await
            .is_ok()
    );
}

#[tokio::test]
#[serial]
async fn test_fail_check_health() {
    load_envs();
    unsafe {
        set_var("OPENSEARCH_PASSWORD", "jibberish");
    }

    let boot = boot_test::<App>().await;
    assert!(boot.is_err());
}
