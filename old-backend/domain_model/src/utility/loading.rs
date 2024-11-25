use sea_orm::ConnectionTrait;

pub async fn load_schema(db: &impl ConnectionTrait) {
    let schema = include_str!("./schema.sql");
    db.execute_unprepared(schema).await.expect("Failed to load schema!");
}
