use diesel::mysql::Mysql;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl, SelectableExpression, SelectableHelper, TextExpressionMethods,
};

use crate::apply_pageable;
use crate::database::schema::locations;
use crate::gen_filter_fn;
use crate::models::location::Location;
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;

gen_filter_fn!(get_filters_name, locations::name, locations::name);
gen_filter_fn!(
    get_filters_description,
    locations::description,
    locations::description
);

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<Location>> {
    let conn = &mut pool.get().unwrap();

    let mut select = locations::table.select(Location::as_select()).into_boxed();

    select = apply_pageable!(select, page);
    select.load::<Location>(conn)
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResult<Vec<Location>> {
    let conn = &mut pool.get().unwrap();

    let mut select = locations::table.select(Location::as_select()).into_boxed();

    if !filter.words.is_empty() {
        let name = get_filters_name(filter.clone());
        let description = get_filters_description(filter);
        select = select.filter(name.or(description));
    }

    select = apply_pageable!(select, page);
    select.load::<Location>(conn)
}

pub fn find_by_id(location_id: u32, pool: &WDPool) -> QueryResult<Location> {
    let conn = &mut pool.get().unwrap();
    locations::table
        .filter(locations::id.eq(location_id))
        .first::<Location>(conn)
}
