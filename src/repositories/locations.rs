use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::database::schema::locations::dsl::*;
use crate::models::location::Location;
use crate::web::types::WDPool;

pub fn find_all(pool: &WDPool) -> QueryResult<Vec<Location>> {
    let conn = &mut pool.get().unwrap();
    locations.load::<Location>(conn)
}

pub fn find_by_id(location_id: u32, pool: &WDPool) -> QueryResult<Location> {
    let conn = &mut pool.get().unwrap();
    locations
        .filter(id.eq(location_id))
        .first::<Location>(conn)
}