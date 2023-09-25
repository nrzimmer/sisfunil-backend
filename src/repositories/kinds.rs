use diesel::mysql::Mysql;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl, SelectableExpression, SelectableHelper, TextExpressionMethods,
};

use crate::{apply_pageable, gen_filter_fn};

use crate::database::schema::{groups, kinds};
use crate::dtos::KindDTO;
use crate::models::group;
use crate::models::group::Group;
use crate::models::kind::Kind;
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;

macro_rules! get_select {
    () => {
        kinds::table
            .left_join(groups::table)
            .select((Kind::as_select(), Option::<Group>::as_select()))
    };
}

fn to_kind_dto(kind: Kind, group: Option<Group>) -> KindDTO {
    KindDTO {
        kind,
        group: group.unwrap_or_else(|| group::missing()),
    }
}

gen_filter_fn!(get_filters, kinds::name, kinds::name);

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<KindDTO>> {
    let conn = &mut pool.get().unwrap();

    let select = get_select!().into_boxed();

    let result = apply_pageable!(select, page)
        .order_by(kinds::id)
        .get_results(conn);

    match result {
        Ok(v) => Ok(v
            .into_iter()
            .map(|(kind, group)| to_kind_dto(kind, group))
            .collect::<Vec<KindDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResult<Vec<KindDTO>> {
    let conn = &mut pool.get().unwrap();

    let mut select = get_select!().into_boxed();

    if !filter.words.is_empty() {
        let name = get_filters(filter);
        select = select.filter(name);
    }

    let result = apply_pageable!(select, page)
        .order_by(kinds::id)
        .get_results(conn);

    match result {
        Ok(v) => Ok(v
            .into_iter()
            .map(|(kind, group)| to_kind_dto(kind, group))
            .collect::<Vec<KindDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn find_by_id(kind_id: u32, pool: &WDPool) -> QueryResult<KindDTO> {
    let conn = &mut pool.get().unwrap();
    kinds::table
        .left_join(groups::table)
        .filter(kinds::id.eq(kind_id))
        .select((Kind::as_select(), Option::<Group>::as_select()))
        .first::<(Kind, Option<Group>)>(conn)
        .map(|(kind, group)| to_kind_dto(kind, group))
}
