use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::database::schema::{groups, kinds};
use crate::dtos::KindDTO;
use crate::models::group;
use crate::models::group::Group;
use crate::models::kind::Kind;
use crate::web::router::Pageable;
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
        group: group.unwrap_or_else(
            || group::missing()
        ),
    }
}

pub fn find_all(page: Pageable, pool: &WDPool) -> QueryResult<Vec<KindDTO>> {
    let conn = &mut pool.get().unwrap();
    let limit = page.size.unwrap_or(50);
    let offset = page.start.unwrap_or(0) * limit;

    let result = get_select!()
        .limit(limit.into())
        .offset(offset.into())
        .get_results(conn);

    match result {
        Ok(v) => Ok(v.into_iter()
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
        .first::<(Kind, Option::<Group>)>(conn)
        .map(|(kind, group)| to_kind_dto(kind, group))
}
