use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(crate::models::itembox::Box))]
#[diesel(belongs_to(crate::models::category::Category))]
#[diesel(table_name = crate::database::schema::items)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category_id: u32,
    pub date: NaiveDate,
    pub sealed: bool,
    pub rate: u8,
    pub box_id: u32,
}