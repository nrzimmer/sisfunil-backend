use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::database::schema::{categories, containers, groups, items, kinds, locations};
use crate::dtos::*;
use crate::models::{category, container, group, kind, location};
use crate::models::category::Category;
use crate::models::container::Container;
use crate::models::group::Group;
use crate::models::item::Item;
use crate::models::kind::Kind;
use crate::models::location::Location;
use crate::web::pageable::Pageable;
use crate::web::types::WDPool;

macro_rules! get_select {
    () => {
        items::table
        .left_join(categories::table
            .left_join(kinds::table
                .left_join(groups::table)
            )
        )
        .left_join(containers::table
            .left_join(locations::table)
        )
        .select(
            (
                Item::as_select(),
                Option::<Container>::as_select(),
                Option::<Location>::as_select(),
                Option::<Category>::as_select(),
                Option::<Kind>::as_select(),
                Option::<Group>::as_select()
            )
        )
    };
}

fn to_item_dto(item: Item, container: Option<Container>, location: Option<Location>, category: Option<Category>, kind: Option<Kind>, group: Option<Group>) -> ItemDTO {
    ItemDTO {
        item,
        container: ContainerDTO {
            container: container.unwrap_or_else(
                || container::missing()
            ),
            location: location.unwrap_or_else(
                || location::missing()
            ),
        },
        category: CategoryDTO {
            category: category.unwrap_or_else(
                || category::missing()
            ),
            kind: KindDTO {
                kind: kind.unwrap_or_else(
                    || kind::missing()
                ),
                group: group.unwrap_or_else(
                    || group::missing()
                ),
            },
        },
    }
}

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<ItemDTO>> {
    let conn = &mut pool.get().unwrap();
    let limit = page.size.unwrap_or(50);
    let offset = page.start.unwrap_or(0) * limit;

    let result = get_select!()
        .limit(limit.into())
        .offset(offset.into())
        .get_results(conn);

    match result {
        Ok(v) => Ok(v.into_iter()
            .map(|(item, container, location, category, kind, group)| to_item_dto(item, container, location, category, kind, group))
            .collect::<Vec<ItemDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn find_by_id(item_id: u32, pool: &WDPool) -> QueryResult<ItemDTO> {//QueryResult<(Item, Container, Location)> {
    let conn = &mut pool.get().unwrap();

    get_select!()
        .filter(items::id.eq(item_id))
        .first::<(
            Item,
            Option::<Container>,
            Option::<Location>,
            Option::<Category>,
            Option::<Kind>,
            Option::<Group>)>(conn)
        .map(|(item, container, location, category, kind, group)| to_item_dto(item, container, location, category, kind, group))
}