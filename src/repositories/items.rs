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
use crate::web::router::Pageable;
use crate::web::types::WDPool;

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<ItemDTO>> {
    let conn = &mut pool.get().unwrap();
    let limit = page.size.unwrap_or(50);
    let offset = page.start.unwrap_or(0) * limit;

    let result = items::table
        .left_join(categories::table
            .left_join(kinds::table
                .left_join(groups::table)
            )
        )
        .left_join(containers::table
            .left_join(locations::table)
        )
        .limit(limit.into())
        .offset(offset.into())
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
        .get_results(conn);

    match result {
        Ok(v) => Ok(v.into_iter()
            .map(|(a, b, c, d, e, f)|
                ItemDTO {
                    item: a,
                    container: ContainerDTO {
                        container: b.unwrap_or_else(
                            || container::missing()
                        ),
                        location: c.unwrap_or_else(
                            || location::missing()
                        ),
                    },
                    category: CategoryDTO {
                        category: d.unwrap_or_else(
                            || category::missing()
                        ),
                        kind: KindDTO {
                            kind: e.unwrap_or_else(
                                || kind::missing()
                            ),
                            group: f.unwrap_or_else(
                                || group::missing()
                            ),
                        },
                    },
                }
            )
            .collect::<Vec<ItemDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn find_by_id(item_id: u32, pool: &WDPool) -> QueryResult<ItemDTO> {//QueryResult<(Item, Container, Location)> {
    let conn = &mut pool.get().unwrap();

    items::table
        .left_join(categories::table
            .left_join(kinds::table
                .left_join(groups::table)
            )
        )
        .left_join(containers::table
            .left_join(locations::table)
        )
        .filter(items::id.eq(item_id))
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
        .first::<(
            Item,
            Option::<Container>,
            Option::<Location>,
            Option::<Category>,
            Option::<Kind>,
            Option::<Group>)>(conn)
        .map(|(a, b, c, d, e, f)|
            ItemDTO {
                item: a,
                container: ContainerDTO {
                    container: b.unwrap_or_else(
                        || container::missing()
                    ),
                    location: c.unwrap_or_else(
                        || location::missing()
                    ),
                },
                category: CategoryDTO {
                    category: d.unwrap_or_else(
                        || category::missing()
                    ),
                    kind: KindDTO {
                        kind: e.unwrap_or_else(
                            || kind::missing()
                        ),
                        group: f.unwrap_or_else(
                            || group::missing()
                        ),
                    },
                },
            }
        )
}