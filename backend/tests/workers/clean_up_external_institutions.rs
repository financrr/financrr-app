use crate::helpers::init::load_envs;
use financrr::bank_account_linking::constants::GO_CARDLESS_PROVIDER;
use financrr::{
    app::App,
    workers::clean_up_external_institutions::{CleanUpExternalInstitutions, WorkerArgs},
};
use loco_rs::{bgworker::BackgroundWorker, testing::prelude::*};
use serial_test::serial;

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_run_clean_up_external_institutions_worker() {
    load_envs();

    let boot = boot_test::<App>().await.unwrap();

    // Execute the worker ensuring that it operates in 'ForegroundBlocking' mode, which prevents the addition of your worker to the background
    assert!(CleanUpExternalInstitutions::perform_later(
        &boot.app_context,
        WorkerArgs {
            external_ids: vec![],
            provider: GO_CARDLESS_PROVIDER.to_string()
        }
    )
    .await
    .is_ok());
    // Include additional assert validations after the execution of the worker
}
