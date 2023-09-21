use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::database::schema::categories::dsl::*;
use crate::models::category::Category;
use crate::web::types::WDPool;

pub fn find_all(pool: &WDPool) -> QueryResult<Vec<Category>> {
    let conn = &mut pool.get().unwrap();
    categories
        .select(Category::as_select())
        .get_results(conn)
}

pub fn find_by_id(category_id: u32, pool: &WDPool) -> QueryResult<Category> {
    let conn = &mut pool.get().unwrap();
    categories
        .filter(id.eq(category_id))
        .select(Category::as_select())
        .first::<Category>(conn)
}