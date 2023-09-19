use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = crate::database::schema::locations)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: String,
}
