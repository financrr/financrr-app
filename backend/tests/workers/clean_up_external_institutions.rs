use crate::helpers::external_institutions::generate_institutions;
use crate::helpers::init::load_envs;
use financrr::app::App;
use financrr::bank_account_linking::constants::GO_CARDLESS_PROVIDER;
use financrr::models::external_bank_institutions;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_run_clean_up_external_institutions_worker() {
    load_envs();

    let boot = boot_test::<App>().await.unwrap();

    const GENERATE_COUNT: usize = 20;
    const STRIP_COUNT: usize = 5;

    let institutions = generate_institutions(&boot.app_context, GO_CARDLESS_PROVIDER, GENERATE_COUNT).await;
    let _: Vec<String> = institutions
        .iter()
        .skip(STRIP_COUNT)
        .map(|i| i.external_id.clone())
        .collect();

    assert_eq!(
        external_bank_institutions::Entity::count_all(&boot.app_context.db)
            .await
            .unwrap(),
        GENERATE_COUNT as u64
    );

    assert_eq!(
        external_bank_institutions::Entity::count_all(&boot.app_context.db)
            .await
            .unwrap(),
        (GENERATE_COUNT - STRIP_COUNT) as u64
    );

    assert_eq!(
        external_bank_institutions::Entity::count_all(&boot.app_context.db)
            .await
            .unwrap(),
        0
    );
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_if_non_existing_ids_where_given() {
    load_envs();

    let boot = boot_test::<App>().await.unwrap();

    const GENERATE_COUNT: usize = 20;

    let _ = generate_institutions(&boot.app_context, GO_CARDLESS_PROVIDER, GENERATE_COUNT).await;
    let _ = vec!["non_existing_id".to_string()];

    assert_eq!(
        external_bank_institutions::Entity::count_all(&boot.app_context.db)
            .await
            .unwrap(),
        GENERATE_COUNT as u64
    );

    assert_eq!(
        external_bank_institutions::Entity::count_all(&boot.app_context.db)
            .await
            .unwrap(),
        0
    );
}
