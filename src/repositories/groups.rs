use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::database::schema::groups::dsl::*;
use crate::models::group::Group;
use crate::web::router::Pageable;
use crate::web::types::WDPool;

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<Group>> {
    let conn = &mut pool.get().unwrap();
    let limit = page.size.unwrap_or(50);
    let offset = page.start.unwrap_or(0) * limit;

    groups
        .limit(limit.into())
        .offset(offset.into())
        .load::<Group>(conn)
}

pub fn find_by_id(group_id: u32, pool: &WDPool) -> QueryResult<Group> {
    let conn = &mut pool.get().unwrap();
    groups
        .filter(id.eq(group_id))
        .first::<Group>(conn)
}