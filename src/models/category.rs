use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::models::itemtype::ItemType;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::categories)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub item_type: ItemType,
}
