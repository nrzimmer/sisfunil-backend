use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::web::types::WDPool;
use crate::models::group::Group;
use crate::database::schema::groups::dsl::*;

pub fn find_all(pool: &WDPool) -> QueryResult<Vec<Group>> {
    let conn = &mut pool.get().unwrap();
    groups.load::<Group>(conn)
}

pub fn find_by_id(item_id: u32, pool: &WDPool) -> QueryResult<Group> {
    let conn = &mut pool.get().unwrap();
    groups
        .filter(id.eq(item_id))
        .first::<Group>(conn)
}