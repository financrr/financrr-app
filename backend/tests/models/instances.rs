use crate::helpers::init::init_test;
use financrr::app::App;
use financrr::models::_entities::instances;
use financrr::services::Service;
use financrr::services::instance_handler::InstanceHandlerInner;
use financrr::services::snowflake_generator::MAX_NODE_ID;
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
async fn can_create_instance() {
    init_test!();

    let boot = boot_test::<App>().await.unwrap();

    let instance = instances::Model::get_node_id_and_create_new_instance(&boot.app_context.db).await;
    assert!(instance.is_ok());

    let instance = instance.unwrap();
    assert_eq!(instance.node_id, 1);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn can_create_multiple_instances() {
    init_test!();

    let boot = boot_test::<App>().await.unwrap();

    let mut instances = Vec::new();

    for _ in 0..5 {
        let instance = instances::Model::get_node_id_and_create_new_instance(&boot.app_context.db).await;
        assert!(instance.is_ok());

        instances.push(instance.unwrap());
    }

    for (i, instance) in instances.into_iter().enumerate() {
        assert_eq!(instance.node_id, i as i16 + 1);
    }
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn can_create_instance_handler() {
    init_test!();

    let boot = boot_test::<App>().await.unwrap();

    let instance = InstanceHandlerInner::new(&boot.app_context).await;
    assert!(instance.is_ok());

    let node_id = instance.unwrap().get_instance_id();
    assert_eq!(node_id, 1);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn can_create_maximum_instances() {
    init_test!();

    let boot = boot_test::<App>().await.unwrap();

    let mut instances = Vec::with_capacity(MAX_NODE_ID as usize);

    for _ in 0..MAX_NODE_ID {
        let instance = InstanceHandlerInner::new(&boot.app_context).await;
        assert!(instance.is_ok());

        instances.push(instance.unwrap());
    }

    for (i, instance) in instances.into_iter().enumerate() {
        assert_eq!(instance.get_instance_id(), i as u16 + 1);
    }

    let number_of_instances = instances::Model::count_instances(&boot.app_context.db).await.unwrap();
    assert_eq!(number_of_instances, MAX_NODE_ID + 1);

    // Assert that no new instance can be created
    let instance = InstanceHandlerInner::new(&boot.app_context).await;
    assert!(instance.is_err());

    let error = instance.err().unwrap();
    assert_debug_snapshot!(error)
}
