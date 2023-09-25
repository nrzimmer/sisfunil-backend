use diesel::mysql::Mysql;
use diesel::result::Error;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl, SelectableExpression, SelectableHelper, TextExpressionMethods,
};

use crate::database::schema::{containers, locations};
use crate::dtos::{get_result_dto, get_results_dto, ContainerDTO, QueryResultDTO};
use crate::models::container::Container;
use crate::models::location;
use crate::models::location::Location;
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;
use crate::{apply_pageable, gen_filter_fn};

macro_rules! get_boxed {
    () => {
        containers::table.left_join(locations::table).into_boxed()
    };
}

macro_rules! apply_select {
    ($fun:expr) => {
        $fun.select((Container::as_select(), Option::<Location>::as_select()))
    };
}

gen_filter_fn!(
    get_filters,
    containers::description,
    containers::description
);

fn to_container_dto(container: Container, location: Option<Location>) -> ContainerDTO {
    ContainerDTO {
        container,
        location: location.unwrap_or_else(|| location::missing()),
    }
}

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResultDTO<ContainerDTO> {
    let conn = &mut pool.get().unwrap();

    let select = get_boxed!();

    let result = apply_select!(apply_pageable!(select, page))
        .order_by(containers::id)
        .get_results(conn);

    let data = match_result(result);

    let count = get_boxed!().count().get_result(conn);

    get_results_dto(data, count)
}

fn match_result(
    result: QueryResult<Vec<(Container, Option<Location>)>>,
) -> Result<Vec<ContainerDTO>, Error> {
    match result {
        Ok(v) => Ok(v
            .into_iter()
            .map(|(container, location)| to_container_dto(container, location))
            .collect::<Vec<ContainerDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResultDTO<ContainerDTO> {
    let conn = &mut pool.get().unwrap();

    let mut select = get_boxed!();

    if !filter.words.is_empty() {
        let name = get_filters(filter.clone());
        select = select.filter(name);
    }

    let result = apply_select!(apply_pageable!(select, page))
        .order_by(containers::id)
        .get_results(conn);

    let data = match_result(result);

    let mut select = get_boxed!();

    if !filter.words.is_empty() {
        let name = get_filters(filter);
        select = select.filter(name);
    }

    let count = select.count().get_result(conn);

    get_results_dto(data, count)
}

pub fn find_by_id(container_id: u32, pool: &WDPool) -> QueryResultDTO<ContainerDTO> {
    let conn = &mut pool.get().unwrap();

    let data = apply_select!(get_boxed!())
        .filter(containers::id.eq(container_id))
        .first::<(Container, Option<Location>)>(conn)
        .map(|(container, location)| to_container_dto(container, location));

    get_result_dto(data)
}
