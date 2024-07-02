use sea_orm::{DatabaseBackend, QueryTrait};

use entity::{account, transaction};

#[test]
fn test_find_all_accounts_by_user_id() {
    let user_id = 1;
    let query = account::Entity::find_all_by_user_id(user_id.into());
    let actual_sql = query.build(DatabaseBackend::Postgres).to_string();

    let expected_sql = "SELECT \"account\".\"id\", \"account\".\"name\", \"account\".\"description\", \"account\".\"iban\", \"account\".\"balance\", \"account\".\"original_balance\", \"account\".\"currency\", \"account\".\"created_at\" FROM \"account\" INNER JOIN \"permissions\" ON \"permissions\".\"entity_id\" = \"account\".\"id\" WHERE \"permissions\".\"user_id\" = 1 AND \"permissions\".\"entity_type\" = 'account'";

    assert_eq!(actual_sql, expected_sql);
}

#[test]
fn test_find_by_id_and_user_id() {
    let account_id = 13;
    let user_id = 1;
    let query = account::Entity::find_by_id_and_user_id(account_id.into(), user_id.into());
    let actual_sql = query.build(DatabaseBackend::Postgres).to_string();

    let expected_sql = "SELECT \"account\".\"id\", \"account\".\"name\", \"account\".\"description\", \"account\".\"iban\", \"account\".\"balance\", \"account\".\"original_balance\", \"account\".\"currency\", \"account\".\"created_at\" FROM \"account\" INNER JOIN \"permissions\" ON \"permissions\".\"entity_id\" = \"account\".\"id\" AND \"entity_type\" = 'account' WHERE \"permissions\".\"user_id\" = 1 AND \"account\".\"id\" = 13";

    assert_eq!(actual_sql, expected_sql);
}

#[test]
fn test_find_all_transactions_by_user_id() {
    let user_id = 1;
    let query = transaction::Entity::find_all_by_user_id(user_id.into());
    let actual_sql = query.build(DatabaseBackend::Postgres).to_string();

    let expected = "SELECT \"transaction\".\"id\", \"transaction\".\"source\", \"transaction\".\"destination\", \"transaction\".\"amount\", \"transaction\".\"currency\", \"transaction\".\"name\", \"transaction\".\"description\", \"transaction\".\"budget\", \"transaction\".\"executed_at\", \"transaction\".\"created_at\" FROM \"transaction\" INNER JOIN \"permissions\" ON \"permissions\".\"entity_id\" = \"transaction\".\"id\" WHERE \"permissions\".\"user_id\" = 1 AND \"permissions\".\"entity_type\" = 'transaction'";

    assert_eq!(actual_sql, expected);
}
