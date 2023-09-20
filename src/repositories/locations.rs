use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::web::types::WDPool;
use crate::models::location::Location;
use crate::database::schema::locations::dsl::*;

pub fn find_all(pool: &WDPool) -> QueryResult<Vec<Location>> {
    let conn = &mut pool.get().unwrap();
    locations.load::<Location>(conn)
}

pub fn find_by_id(item_id: u32, pool: &WDPool) -> QueryResult<Location> {
    let conn = &mut pool.get().unwrap();
    locations
        .filter(id.eq(item_id))
        .first::<Location>(conn)
}