use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::database::schema::{containers, locations};
use crate::dtos::ContainerDTO;
use crate::models::container::Container;
use crate::models::location;
use crate::models::location::Location;
use crate::web::router::Pageable;
use crate::web::types::WDPool;

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<ContainerDTO>> {
    let conn = &mut pool.get().unwrap();
    let limit = page.size.unwrap_or(50);
    let offset = page.start.unwrap_or(0) * limit;

    let result = containers::table
        .left_join(locations::table)
        .limit(limit.into())
        .offset(offset.into())
        .select((Container::as_select(), Option::<Location>::as_select()))
        .get_results(conn);

    match result {
        Ok(v) => Ok(v.into_iter()
            .map(|(container, y)|
                ContainerDTO {
                    container,
                    location: y.unwrap_or_else(
                        || location::missing()
                    ),
                }
            )
            .collect::<Vec<ContainerDTO>>()),
        Err(e) => Err(e),
    }
}

pub fn find_by_id(container_id: u32, pool: &WDPool) -> QueryResult<ContainerDTO> {
    let conn = &mut pool.get().unwrap();
    containers::table
        .left_join(locations::table)
        .filter(containers::id.eq(container_id))
        .select((Container::as_select(), Option::<Location>::as_select()))
        .first::<(Container, Option::<Location>)>(conn)
        .map(|(x, y)|
            ContainerDTO {
                container: x,
                location: y.unwrap_or_else(
                    || location::missing()
                ),
            }
        )
}