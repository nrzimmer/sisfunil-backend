use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::database::schema::groups::dsl::*;
use crate::models::group::Group;
use crate::web::types::WDPool;

pub fn find_all(pool: &WDPool) -> QueryResult<Vec<Group>> {
    let conn = &mut pool.get().unwrap();
    groups.load::<Group>(conn)
}

pub fn find_by_id(group_id: u32, pool: &WDPool) -> QueryResult<Group> {
    let conn = &mut pool.get().unwrap();
    groups
        .filter(id.eq(group_id))
        .first::<Group>(conn)
}