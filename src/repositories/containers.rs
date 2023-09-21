use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::database::schema::containers::dsl::*;
use crate::models::container::Container;
use crate::web::types::WDPool;

pub fn find_all(pool: &WDPool) -> QueryResult<Vec<Container>> {
    let conn = &mut pool.get().unwrap();
    containers
        .select(Container::as_select())
        .get_results(conn)
}

pub fn find_by_id(container_id: u32, pool: &WDPool) -> QueryResult<Container> {
    let conn = &mut pool.get().unwrap();
    containers
        .filter(id.eq(container_id))
        .select(Container::as_select())
        .first::<Container>(conn)
}