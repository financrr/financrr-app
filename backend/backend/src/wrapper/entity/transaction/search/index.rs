use opensearch::SearchParts;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::OffsetDateTime;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::databases::connections::search::get_open_search_client;
use crate::search::index::{IndexBuilder, Indexer, IndexType};
use crate::wrapper::entity::TableName;
use crate::wrapper::entity::transaction::Transaction;
use crate::wrapper::permission::{Permissions, PermissionsEntity};
use crate::wrapper::search::{Searchable, SearchQuery};

pub const INDEX_NAME: &str = "transaction";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct TransactionIndex {
    id: i32,
    source: Option<i32>,
    destination: Option<i32>,
    description: Option<String>,
    currency: i32,
    budget: Option<i32>,
    #[serde(with = "time::serde::rfc3339")]
    executed_at: OffsetDateTime,
    user_ids: Vec<i32>,
}

impl TransactionIndex {
    pub(super) fn from_transaction(transaction: Transaction, user_ids: Vec<i32>) -> Self {
        Self {
            id: transaction.id,
            source: transaction.source.map(|s| s.get_id()),
            destination: transaction.destination.map(|d| d.get_id()),
            description: transaction.description,
            currency: transaction.currency.get_id(),
            budget: transaction.budget.map(|b| b.get_id()),
            executed_at: transaction.executed_at,
            user_ids,
        }
    }
}

impl Searchable for TransactionIndex {
    async fn create_index() {
        IndexBuilder::new(INDEX_NAME)
            .add_field("id", IndexType::INTEGER)
            .add_field("source", IndexType::INTEGER)
            .add_field("destination", IndexType::INTEGER)
            .add_field("description", IndexType::TEXT)
            .add_field("currency", IndexType::INTEGER)
            .add_field("budget", IndexType::INTEGER)
            .add_field("executed_at", IndexType::DATE)
            .add_field("user_ids", IndexType::INTEGER)
            .send()
            .await;
    }

    async fn index() -> Result<(), ApiError> {
        let page_size = 500;
        let total = Transaction::count_all().await?;
        let pages = (total as f64 / page_size as f64).ceil() as u64;

        for page in 1..pages {
            let page_size_param = PageSizeParam::new(page, page_size);
            let transactions = Transaction::find_all_paginated(page_size_param).await?;
            tokio::spawn(async move {
                let mut docs = Vec::with_capacity(transactions.len());
                for transaction in transactions {
                    let user_ids = PermissionsEntity::find_users_with_permissions(
                        Transaction::table_name(),
                        transaction.id,
                        Permissions::READ,
                    )
                        .await
                        .unwrap_or_default();
                    if user_ids.is_empty() {
                        continue;
                    }

                    let index = TransactionIndex::from_transaction(transaction, user_ids);
                    let doc = serde_json::to_value(index).unwrap();
                    docs.push(doc);
                }
                Indexer::index_documents(INDEX_NAME, docs).await;
            });
        }

        Ok(())
    }

    async fn search(_query: SearchQuery, user_id: i32, page_size: PageSizeParam) -> Result<Vec<Self>, ApiError> {
        let from = page_size.page * page_size.limit;

        let client = get_open_search_client();
        client
            .search(SearchParts::Index(&[INDEX_NAME]))
            .from(from as i64)
            .size(page_size.limit as i64)
            .body(json!({
                "query": {
                    "bool": {
                        "must": [
                            {
                                "terms": {
                                    "user_ids": [user_id]
                                }
                            }
                        ]
                    }
                }
            })
            .await
            .map_err(ApiError::from)
    }
}
