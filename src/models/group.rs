use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = crate::database::schema::groups)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Group {
    pub id: u32,
    pub name: String,
}
