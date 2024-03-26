use std::collections::HashMap;

use opensearch::http::request::JsonBody;
use opensearch::indices::IndicesCreateParts;
use opensearch::BulkParts;
use serde_json::{json, Value};

use crate::databases::connections::search::get_open_search_client;

#[derive(Debug)]
pub(crate) struct IndexBuilder {
    name: String,
    fields: HashMap<String, String>,
}

impl IndexBuilder {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: Default::default(),
        }
    }

    pub(crate) fn add_field(mut self, name: &str, field_type: &str) -> Self {
        self.fields.insert(name.to_string(), field_type.to_string());
        self
    }

    pub(crate) async fn send(&self) {
        let client = get_open_search_client();
        client
            .indices()
            .create(IndicesCreateParts::Index(&self.name))
            .body(json!({
                "mappings" : {
                    "properties" : self.fields
                        .iter()
                        .map(|(name, field_type)| {
                            (name, json!({ "type": field_type }))
                        })
                        .collect::<HashMap<_, _>>()
                }
            }))
            .send()
            .await
            .unwrap_or_else(|_| panic!("INDEX {} not created", self.name));
    }
}

pub(crate) struct IndexType;

impl IndexType {
    pub(crate) const TEXT: &'static str = "text";
    pub(crate) const INTEGER: &'static str = "integer";
    pub(crate) const DATE: &'static str = "date";
}

pub(crate) struct Indexer;

impl Indexer {
    pub(crate) async fn index_documents(index: &str, documents: Vec<Value>) {
        let body: Vec<JsonBody<_>> = documents.into_iter().map(JsonBody::new).collect();

        let client = get_open_search_client();
        client
            .bulk(BulkParts::Index(index))
            .body(body)
            .send()
            .await
            .unwrap_or_else(|_| panic!("Documents not indexed in {}", index));
    }
}
