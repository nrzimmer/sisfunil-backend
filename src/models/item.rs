use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::models::category::Category;
use crate::models::itembox::ItemBox;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::items)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category: Category,
    pub date: NaiveDate,
    pub sealed: bool,
    pub rating: u8,
    pub item_box: ItemBox,
}