use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::database::schema::locations::dsl::*;
use crate::models::location::Location;
use crate::web::router::Pageable;
use crate::web::types::WDPool;

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<Location>> {
    let conn = &mut pool.get().unwrap();
    let limit = page.size.unwrap_or(50);
    let offset = page.start.unwrap_or(0) * limit;

    locations
        .limit(limit.into())
        .offset(offset.into())
        .load::<Location>(conn)
}

pub fn find_by_id(location_id: u32, pool: &WDPool) -> QueryResult<Location> {
    let conn = &mut pool.get().unwrap();
    locations
        .filter(id.eq(location_id))
        .first::<Location>(conn)
}