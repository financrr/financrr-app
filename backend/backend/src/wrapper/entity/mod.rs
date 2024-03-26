use crate::wrapper::entity::transaction::search::init_transactions_search;
use tracing::info;

pub(crate) mod account;
pub(crate) mod budget;
pub(crate) mod currency;
pub(crate) mod session;
pub(crate) mod transaction;
pub(crate) mod user;

pub(crate) async fn init_wrapper() {
    info!("\t[*] Init wrapper-search");
    init_search().await;
}

async fn init_search() {
    info!("\t[*] Init transaction search");
    init_transactions_search().await;
}

pub(crate) trait WrapperEntity: TableName {
    fn get_id(&self) -> i32;
}

pub(crate) trait TableName {
    fn table_name() -> &'static str;
}
