use diesel::mysql::Mysql;
use diesel::result::Error;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl, SelectableExpression, SelectableHelper, TextExpressionMethods,
};

use crate::database::schema::{categories, groups, kinds};
use crate::dtos::{get_result_dto, get_results_dto, CategoryDTO, KindDTO, QueryResultDTO};
use crate::models::category::Category;
use crate::models::group::Group;
use crate::models::kind::Kind;
use crate::models::{group, kind};
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;
use crate::{apply_pageable, gen_filter_fn};

gen_filter_fn!(get_filters, categories::name, categories::name);

macro_rules! get_boxed {
    () => {
        categories::table
            .left_join(kinds::table.left_join(groups::table))
            .into_boxed()
    };
}

macro_rules! apply_select {
    ($fun:expr) => {
        $fun.select((
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

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResultDTO<CategoryDTO> {
    let conn = &mut pool.get().unwrap();

    let result = apply_select!(apply_pageable!(get_boxed!(), page).order_by(categories::id))
        .get_results(conn);

    let data = match_result(result);

    let count = get_boxed!().count().get_result(conn);

    get_results_dto(data, count)
}

fn match_result(
    result: QueryResult<Vec<(Category, Option<Kind>, Option<Group>)>>,
) -> Result<Vec<CategoryDTO>, Error> {
    match result {
        Ok(v) => Ok(v
            .into_iter()
            .map(|(category, kind, group)| to_category_dto(category, kind, group))
            .collect::<Vec<CategoryDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResultDTO<CategoryDTO> {
    let conn = &mut pool.get().unwrap();

    let mut select = apply_select!(get_boxed!());

    if !filter.words.is_empty() {
        let name = get_filters(filter);
        select = select.filter(name);
    }

    let result = apply_pageable!(select, page)
        .order_by(categories::id)
        .get_results(conn);

    let data = match_result(result);

    let count = get_boxed!().count().get_result(conn);

    get_results_dto(data, count)
}

pub fn find_by_id(category_id: u32, pool: &WDPool) -> QueryResultDTO<CategoryDTO> {
    let conn = &mut pool.get().unwrap();

    let data = apply_select!(get_boxed!())
        .filter(categories::id.eq(category_id))
        .first::<(Category, Option<Kind>, Option<Group>)>(conn)
        .map(|(category, kind, group)| to_category_dto(category, kind, group));

    get_result_dto(data)
}
