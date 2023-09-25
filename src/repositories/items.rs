use diesel::mysql::Mysql;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl, SelectableExpression, SelectableHelper, TextExpressionMethods,
};

use crate::database::schema::{categories, containers, groups, items, kinds, locations};
use crate::dtos::*;
use crate::models::category::Category;
use crate::models::container::Container;
use crate::models::group::Group;
use crate::models::item::Item;
use crate::models::kind::Kind;
use crate::models::location::Location;
use crate::models::{category, container, group, kind, location};
use crate::web::filter::Filter;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;
use crate::{apply_pageable, gen_filters_fn};

macro_rules! get_select {
    () => {
        items::table
            .left_join(categories::table.left_join(kinds::table.left_join(groups::table)))
            .left_join(containers::table.left_join(locations::table))
            .select((
                Item::as_select(),
                Option::<Container>::as_select(),
                Option::<Location>::as_select(),
                Option::<Category>::as_select(),
                Option::<Kind>::as_select(),
                Option::<Group>::as_select(),
            ))
    };
}

gen_filters_fn!(
    get_filters,
    items::name,
    items::name,
    items::description,
    items::description
);

fn to_item_dto(
    item: Item,
    container: Option<Container>,
    location: Option<Location>,
    category: Option<Category>,
    kind: Option<Kind>,
    group: Option<Group>,
) -> ItemDTO {
    ItemDTO {
        item,
        container: ContainerDTO {
            container: container.unwrap_or_else(|| container::missing()),
            location: location.unwrap_or_else(|| location::missing()),
        },
        category: CategoryDTO {
            category: category.unwrap_or_else(|| category::missing()),
            kind: KindDTO {
                kind: kind.unwrap_or_else(|| kind::missing()),
                group: group.unwrap_or_else(|| group::missing()),
            },
        },
    }
}

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<ItemDTO>> {
    let conn = &mut pool.get().unwrap();

    let select = get_select!().into_boxed();

    let result = apply_pageable!(select, page)
        .order_by(items::id)
        .get_results(conn);

    match result {
        Ok(v) => Ok(v
            .into_iter()
            .map(|(item, container, location, category, kind, group)| {
                to_item_dto(item, container, location, category, kind, group)
            })
            .collect::<Vec<ItemDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn search(filter: Filter, page: Pageable, pool: &WDPool) -> QueryResult<Vec<ItemDTO>> {
    let conn = &mut pool.get().unwrap();

    let mut select = get_select!().into_boxed();

    if !filter.words.is_empty() {
        select = select.filter(get_filters(filter));
    }

    let result = apply_pageable!(select, page)
        .order_by(items::id)
        .get_results(conn);

    match result {
        Ok(v) => Ok(v
            .into_iter()
            .map(|(item, container, location, category, kind, group)| {
                to_item_dto(item, container, location, category, kind, group)
            })
            .collect::<Vec<ItemDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn find_by_id(item_id: u32, pool: &WDPool) -> QueryResult<ItemDTO> {
    //QueryResult<(Item, Container, Location)> {
    let conn = &mut pool.get().unwrap();

    get_select!()
        .filter(items::id.eq(item_id))
        .first::<(
            Item,
            Option<Container>,
            Option<Location>,
            Option<Category>,
            Option<Kind>,
            Option<Group>,
        )>(conn)
        .map(|(item, container, location, category, kind, group)| {
            to_item_dto(item, container, location, category, kind, group)
        })
}
