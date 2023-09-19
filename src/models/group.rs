use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::groups)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: u32,
    pub name: String,
}
