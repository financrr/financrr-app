use opensearch::auth::Credentials;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
// I am not sure why we have to reexport this but when we do not do this the compiler fails?
pub(crate) use opensearch::OpenSearch;
use url::Url;

use crate::config::Config;
use crate::databases::connections::SEARCH_CONN;

pub(crate) async fn create_open_search_client() -> OpenSearch {
    let url = Url::parse(Config::get_config().search.get_url().as_str()).expect("Could not parse search URL!");
    let pool = SingleNodeConnectionPool::new(url);

    let credentials = Credentials::Basic(
        Config::get_config().search.username.to_string(),
        Config::get_config().search.password.to_string(),
    );

    let transport = TransportBuilder::new(pool)
        .auth(credentials)
        .disable_proxy()
        .build()
        .expect("Could not build search transport!");

    OpenSearch::new(transport)
}

pub(crate) fn get_open_search_client<'a>() -> &'a OpenSearch {
    SEARCH_CONN.get().expect("Could not get search connection!")
}
