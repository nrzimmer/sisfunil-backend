use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(crate::models::container::Container))]
#[diesel(belongs_to(crate::models::category::Category))]
#[diesel(table_name = crate::database::schema::items)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    #[serde(skip)]
    pub category_id: u32,
    pub date: NaiveDate,
    pub sealed: bool,
    pub rate: u8,
    #[serde(skip)]
    pub container_id: u32,
}