use diesel::mysql::Mysql;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableExpression, SelectableHelper, TextExpressionMethods,
};

use crate::apply_pageable;
use crate::database::schema::locations;
use crate::dtos::{get_result_dto, get_results_dto, QueryResultDTO};
use crate::gen_filters_fn;
use crate::models::location::Location;
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;

gen_filters_fn!(
    get_filters,
    locations::name,
    locations::name,
    locations::description,
    locations::description
);

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResultDTO<Location> {
    let conn = &mut pool.get().unwrap();

    let data = apply_pageable!(locations::table.into_boxed(), page)
        .order_by(locations::id)
        .select(Location::as_select())
        .load::<Location>(conn);

    let count = apply_pageable!(locations::table.into_boxed(), page)
        .count()
        .get_result(conn);

    get_results_dto(data, count)
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResultDTO<Location> {
    let conn = &mut pool.get().unwrap();

    let mut select = locations::table.into_boxed();

    if !filter.words.is_empty() {
        select = select.filter(get_filters(filter.clone()));
    }

    let data = apply_pageable!(select, page)
        .order_by(locations::id)
        .select(Location::as_select())
        .load::<Location>(conn);

    let mut select = locations::table.into_boxed();

    if !filter.words.is_empty() {
        select = select.filter(get_filters(filter));
    }

    let count = apply_pageable!(select, page).count().get_result(conn);

    get_results_dto(data, count)
}

pub fn find_by_id(location_id: u32, pool: &WDPool) -> QueryResultDTO<Location> {
    let conn = &mut pool.get().unwrap();

    let data = locations::table
        .filter(locations::id.eq(location_id))
        .first::<Location>(conn);

    get_result_dto(data)
}
