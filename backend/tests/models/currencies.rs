use crate::helpers::init::init_test;
use crate::helpers::users::clean_up_user_model;
use financrr::app::App;
use financrr::models::_entities::currencies;
use insta::assert_debug_snapshot;
use loco_rs::prelude::boot_test;
use serial_test::serial;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("instances");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_unknown_currency() {
    init_test!();

    let boot = boot_test::<App>().await.unwrap();

    let unknown_currency = currencies::Model::get_default_currency(&boot.app_context.db).await;
    assert!(unknown_currency.is_ok());

    insta::with_settings!({
        filters => clean_up_user_model()
    }, {
        assert_debug_snapshot!(unknown_currency.unwrap());
    });
}
