use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::database::schema::{categories, groups, kinds};
use crate::dtos::{CategoryDTO, KindDTO};
use crate::models::{group, kind};
use crate::models::category::Category;
use crate::models::group::Group;
use crate::models::kind::Kind;
use crate::web::router::Pageable;
use crate::web::types::WDPool;

macro_rules! get_select {
    () => {
        categories::table
        .left_join(kinds::table
            .left_join(groups::table)
        )
        .select((Category::as_select(), Option::<Kind>::as_select(), Option::<Group>::as_select()))
    };
}

fn to_category_dto(category: Category, kind: Option<Kind>, group: Option<Group>) -> CategoryDTO {
    CategoryDTO {
        category,
        kind: KindDTO {
            kind: kind.unwrap_or_else(
                || kind::missing()
            ),
            group: group.unwrap_or_else(
                || group::missing()
            ),
        },
    }
}

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<CategoryDTO>> {
    let conn = &mut pool.get().unwrap();
    let limit = page.size.unwrap_or(50);
    let offset = page.start.unwrap_or(0) * limit;

    let result = get_select!()
        .limit(limit.into())
        .offset(offset.into())
        .get_results(conn);

    match result {
        Ok(v) => Ok(v.into_iter()
            .map(|(category, kind, group)| to_category_dto(category, kind, group))
            .collect::<Vec<CategoryDTO>>()),
        Err(e) => Err(e),
    }
}


pub fn find_by_id(category_id: u32, pool: &WDPool) -> QueryResult<CategoryDTO> {
    let conn = &mut pool.get().unwrap();

    get_select!()
        .filter(categories::id.eq(category_id))
        .first::<(Category, Option::<Kind>, Option::<Group>)>(conn)
        .map(|(category, kind, group)| to_category_dto(category, kind, group))
}