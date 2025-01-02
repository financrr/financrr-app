use actix_web::web;
use factory::configure_factory_app_data;

pub(crate) mod factory;

pub(crate) fn configure_service_app_data(cfg: &mut web::ServiceConfig) {
    configure_factory_app_data(cfg);
}
