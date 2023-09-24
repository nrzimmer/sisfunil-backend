use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::database::schema::{containers, locations};
use crate::dtos::ContainerDTO;
use crate::models::container::Container;
use crate::models::location;
use crate::models::location::Location;
use crate::web::router::Pageable;
use crate::web::types::WDPool;

macro_rules! get_select {
    () => {
        containers::table
        .left_join(locations::table)
        .select((Container::as_select(), Option::< Location >::as_select()))
    };
}

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
    let limit = page.size.unwrap_or(50);
    let offset = page.start.unwrap_or(0) * limit;

    let result = get_select!()
        .limit(limit.into())
        .offset(offset.into())
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
