pub(crate) mod user_factory;

use actix_web::web;
use user_factory::UserFactory;
use web::Data;

pub(super) fn configure_factory_app_data(cfg: &mut web::ServiceConfig) {
    cfg.app_data(Data::new(UserFactory));
}
