macro_rules! init_test {
    ($($expr:expr),*) => {
        configure_insta!();
        crate::helpers::init::load_envs();
    };
}

use financrr::utils::env::load_env_file;
pub(crate) use init_test;
use std::path::Path;

pub fn load_envs() {
    load_env_file();
    if Path::new(".env.test").exists() {
        dotenvy::from_path(".env.test").unwrap();
    }
}
