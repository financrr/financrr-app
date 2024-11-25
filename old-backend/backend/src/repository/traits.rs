use sea_orm::{ConnectionTrait, EntityName, EntityTrait};

pub(crate) trait Repository<M: EntityTrait, E> {
    fn new(conn: &impl ConnectionTrait) -> Self;

    fn table_name() -> &'static str {
        M::default().table_name()
    }

    fn get_conn(&self) -> &dyn ConnectionTrait;
}
