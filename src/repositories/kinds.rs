use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::database::schema::{kinds, groups};
use crate::dtos::KindDTO;
use crate::models::group;
use crate::models::group::Group;
use crate::models::kind::Kind;
use crate::web::types::WDPool;

pub fn find_all(pool: &WDPool) -> QueryResult<Vec<KindDTO>> {
    let conn = &mut pool.get().unwrap();
    let result = kinds::table
        .left_join(groups::table)
        .select((Kind::as_select(), Option::<Group>::as_select()))
        .get_results(conn);

    match result {
        Ok(v) => Ok(v.into_iter()
            .map(|(x, y)|
                KindDTO {
                    kind: x,
                    group: y.unwrap_or_else(
                        || group::missing()
                    ),
                }
            )
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
        .map(|(x, y)|
            KindDTO {
                kind: x,
                group: y.unwrap_or_else(
                    || group::missing()
                )
            }
        )
}