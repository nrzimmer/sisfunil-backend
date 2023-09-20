use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods, SelectableHelper};
use crate::database::schema::containers::dsl::*;
use crate::web::types::WDPool;
use crate::models::container::Container;

pub fn find_all(pool: &WDPool) -> QueryResult<Vec<Container>> {
    let conn = &mut pool.get().unwrap();
    containers
        .select(Container::as_select())
        .get_results(conn)
}

pub fn find_by_id(item_id: u32, pool: &WDPool) -> QueryResult<Container> {
    let conn = &mut pool.get().unwrap();
    containers
        .filter(id.eq(item_id))
        .select(Container::as_select())
        .first::<Container>(conn)
}