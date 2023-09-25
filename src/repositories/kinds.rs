use diesel::mysql::Mysql;
use diesel::result::Error;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl, SelectableExpression, SelectableHelper, TextExpressionMethods,
};

use crate::database::schema::{groups, kinds};
use crate::dtos::{get_result_dto, get_results_dto, KindDTO, QueryResultDTO};
use crate::models::group;
use crate::models::group::Group;
use crate::models::kind::Kind;
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;
use crate::{apply_pageable, gen_filter_fn};

macro_rules! get_boxed {
    () => {
        kinds::table.left_join(groups::table).into_boxed()
    };
}

macro_rules! apply_select {
    ($fun:expr) => {
        $fun.select((Kind::as_select(), Option::<Group>::as_select()))
    };
}

fn to_kind_dto(kind: Kind, group: Option<Group>) -> KindDTO {
    KindDTO {
        kind,
        group: group.unwrap_or_else(|| group::missing()),
    }
}

gen_filter_fn!(get_filters, kinds::name, kinds::name);

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResultDTO<KindDTO> {
    let conn = &mut pool.get().unwrap();

    let select = get_boxed!();

    let result = apply_select!(apply_pageable!(select, page))
        .order_by(kinds::id)
        .get_results(conn);

    let data = match_result(result);

    let count = get_boxed!().count().get_result(conn);

    get_results_dto(data, count)
}

fn match_result(result: QueryResult<Vec<(Kind, Option<Group>)>>) -> Result<Vec<KindDTO>, Error> {
    match result {
        Ok(v) => Ok(v
            .into_iter()
            .map(|(kind, group)| to_kind_dto(kind, group))
            .collect::<Vec<KindDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResultDTO<KindDTO> {
    let conn = &mut pool.get().unwrap();

    let mut select = get_boxed!();

    if !filter.words.is_empty() {
        let name = get_filters(filter.clone());
        select = select.filter(name);
    }

    let result = apply_select!(apply_pageable!(select, page))
        .order_by(kinds::id)
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

pub fn find_by_id(kind_id: u32, pool: &WDPool) -> QueryResultDTO<KindDTO> {
    let conn = &mut pool.get().unwrap();
    let data = kinds::table
        .left_join(groups::table)
        .filter(kinds::id.eq(kind_id))
        .select((Kind::as_select(), Option::<Group>::as_select()))
        .first::<(Kind, Option<Group>)>(conn)
        .map(|(kind, group)| to_kind_dto(kind, group));
    get_result_dto(data)
}
