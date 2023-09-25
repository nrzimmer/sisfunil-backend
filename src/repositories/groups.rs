use diesel::mysql::Mysql;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableExpression, SelectableHelper, TextExpressionMethods,
};

use crate::database::schema::groups;
use crate::dtos::{get_result_dto, get_results_dto, QueryResultDTO};
use crate::models::group::Group;
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;
use crate::{apply_pageable, gen_filter_fn};

gen_filter_fn!(get_filter, groups::name, groups::name);

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResultDTO<Group> {
    let conn = &mut pool.get().unwrap();

    let data = apply_pageable!(groups::table, page)
        .order_by(groups::id)
        .select(Group::as_select())
        .load::<Group>(conn);

    let count = groups::table.count().get_result(conn);

    get_results_dto(data, count)
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResultDTO<Group> {
    let conn = &mut pool.get().unwrap();

    let mut select = groups::table.into_boxed();

    if !filter.words.is_empty() {
        let name = get_filter(filter.clone());
        select = select.filter(name);
    }

    let data = apply_pageable!(select, page)
        .order_by(groups::id)
        .select(Group::as_select())
        .load::<Group>(conn);

    let mut select = groups::table.into_boxed();

    if !filter.words.is_empty() {
        let name = get_filter(filter);
        select = select.filter(name);
    }

    let count = select.count().get_result(conn);

    get_results_dto(data, count)
}

pub fn find_by_id(group_id: u32, pool: &WDPool) -> QueryResultDTO<Group> {
    let conn = &mut pool.get().unwrap();
    let data = groups::table
        .filter(groups::id.eq(group_id))
        .first::<Group>(conn);

    get_result_dto(data)
}
