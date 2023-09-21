use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::database::schema::kinds::dsl::*;
use crate::models::kind::Kind;
use crate::web::types::WDPool;

pub fn find_all(pool: &WDPool) -> QueryResult<Vec<Kind>> {
    let conn = &mut pool.get().unwrap();
    kinds
        .select(Kind::as_select())
        .get_results(conn)
}

pub fn find_by_id(kind_id: u32, pool: &WDPool) -> QueryResult<Kind> {
    let conn = &mut pool.get().unwrap();
    kinds
        .filter(id.eq(kind_id))
        .select(Kind::as_select())
        .first::<Kind>(conn)
}