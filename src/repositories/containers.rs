use diesel::{BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableExpression, SelectableHelper, TextExpressionMethods};
use diesel::mysql::Mysql;
use diesel::sql_types::Bool;

use crate::{apply_pageable, gen_filter_fn};
use crate::database::schema::{containers, locations};
use crate::dtos::ContainerDTO;
use crate::models::container::Container;
use crate::models::location;
use crate::models::location::Location;
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;

macro_rules! get_select {
    () => {
        containers::table
        .left_join(locations::table)
        .select((Container::as_select(), Option::< Location >::as_select()))
    };
}

gen_filter_fn!(get_filters, containers::description, containers::description);

fn to_container_dto(container: Container, location: Option<Location>) -> ContainerDTO {
    ContainerDTO {
        container,
        location: location.unwrap_or_else(
            || location::missing()
        ),
    }
}

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<ContainerDTO>> {
    let conn = &mut pool.get().unwrap();

    let select = get_select!()
        .into_boxed();

    let result = apply_pageable!(select, page)
        .get_results(conn);

    match result {
        Ok(v) => Ok(v.into_iter()
            .map(|(container, location)| to_container_dto(container, location))
            .collect::<Vec<ContainerDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResult<Vec<ContainerDTO>> {
    let conn = &mut pool.get().unwrap();

    let mut select = get_select!()
        .into_boxed();

    if !filter.words.is_empty() {
        let name = get_filters(filter);
        select = select.filter(name);
    }

    let result = apply_pageable!(select, page)
        .get_results(conn);

    match result {
        Ok(v) => Ok(v.into_iter()
            .map(|(container, location)| to_container_dto(container, location))
            .collect::<Vec<ContainerDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn find_by_id(container_id: u32, pool: &WDPool) -> QueryResult<ContainerDTO> {
    let conn = &mut pool.get().unwrap();

    get_select!()
        .filter(containers::id.eq(container_id))
        .first::<(Container, Option::<Location>)>(conn)
        .map(|(container, location)| to_container_dto(container, location))
}
