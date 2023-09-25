use diesel::{BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableExpression, SelectableHelper, TextExpressionMethods};
use diesel::mysql::Mysql;
use diesel::sql_types::Bool;
use crate::{apply_pageable, gen_filter_fn};

use crate::database::schema::groups;
use crate::models::group::Group;
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;

gen_filter_fn!(get_filter, groups::name, groups::name);

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<Group>> {
    let conn = &mut pool.get().unwrap();

    let mut select = groups::table
        .select(Group::as_select()).into_boxed();

    select = apply_pageable!(select, page);
    select.load::<Group>(conn)
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResult<Vec<Group>> {
    let conn = &mut pool.get().unwrap();

    let mut select = groups::table
        .select(Group::as_select()).into_boxed();

    if !filter.words.is_empty() {
        let name = get_filter(filter);
        select = select.filter(name);
    }

    select = apply_pageable!(select, page);
    select.load::<Group>(conn)
}

pub fn find_by_id(group_id: u32, pool: &WDPool) -> QueryResult<Group> {
    let conn = &mut pool.get().unwrap();
    groups::table
        .filter(groups::id.eq(group_id))
        .first::<Group>(conn)
}