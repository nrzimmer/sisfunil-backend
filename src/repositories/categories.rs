use diesel::mysql::Mysql;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl, SelectableExpression, SelectableHelper, TextExpressionMethods,
};

use crate::database::schema::{categories, groups, kinds};
use crate::dtos::{CategoryDTO, KindDTO};
use crate::models::category::Category;
use crate::models::group::Group;
use crate::models::kind::Kind;
use crate::models::{group, kind};
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;
use crate::{apply_pageable, gen_filter_fn};

gen_filter_fn!(get_filters, categories::name, categories::name);

macro_rules! get_select {
    () => {
        categories::table
            .left_join(kinds::table.left_join(groups::table))
            .select((
                Category::as_select(),
                Option::<Kind>::as_select(),
                Option::<Group>::as_select(),
            ))
    };
}

fn to_category_dto(category: Category, kind: Option<Kind>, group: Option<Group>) -> CategoryDTO {
    CategoryDTO {
        category,
        kind: KindDTO {
            kind: kind.unwrap_or_else(|| kind::missing()),
            group: group.unwrap_or_else(|| group::missing()),
        },
    }
}

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<CategoryDTO>> {
    let conn = &mut pool.get().unwrap();

    let select = get_select!().into_boxed();

    let result = apply_pageable!(select, page).get_results(conn);

    match result {
        Ok(v) => Ok(v
            .into_iter()
            .map(|(category, kind, group)| to_category_dto(category, kind, group))
            .collect::<Vec<CategoryDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResult<Vec<CategoryDTO>> {
    let conn = &mut pool.get().unwrap();

    let mut select = get_select!().into_boxed();

    if !filter.words.is_empty() {
        let name = get_filters(filter);
        select = select.filter(name);
    }

    let result = apply_pageable!(select, page).get_results(conn);

    match result {
        Ok(v) => Ok(v
            .into_iter()
            .map(|(category, kind, group)| to_category_dto(category, kind, group))
            .collect::<Vec<CategoryDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn find_by_id(category_id: u32, pool: &WDPool) -> QueryResult<CategoryDTO> {
    let conn = &mut pool.get().unwrap();

    get_select!()
        .filter(categories::id.eq(category_id))
        .first::<(Category, Option<Kind>, Option<Group>)>(conn)
        .map(|(category, kind, group)| to_category_dto(category, kind, group))
}
