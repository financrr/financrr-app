macro_rules! init_test {
    ($($expr:expr),*) => {
        configure_insta!();
        financrr::utils::env::load_env_file();
        if std::path::Path::new(".env.test").exists() {
            dotenvy::from_path(".env.test").unwrap();
        }
    };
}

pub(crate) use init_test;
