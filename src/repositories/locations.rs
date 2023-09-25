use diesel::{BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use diesel::sql_types::Bool;

use crate::database::schema::locations;
use crate::get_filters;
use crate::apply_pageable;
use crate::models::location::Location;
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<Location>> {
    let conn = &mut pool.get().unwrap();

    let mut select = locations::table
        .select(Location::as_select()).into_boxed();

    select = apply_pageable!(select, page);

    select.load::<Location>(conn)
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResult<Vec<Location>> {
    let conn = &mut pool.get().unwrap();

    let mut select = locations::table
        .select(Location::as_select()).into_boxed();

    if !filter.words.is_empty() {
        let name = get_filters!(filter, locations::table, locations::name);
        let description = get_filters!(filter, locations::table, locations::description);
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
