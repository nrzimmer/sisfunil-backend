use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(crate::models::itemtype::Type))]
#[diesel(table_name = crate::database::schema::categories)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub type_id: u32,
}
