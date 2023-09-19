use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::models::group::Group;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::types)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemType {
    pub id: u32,
    pub name: String,
    pub group: Group,
}
