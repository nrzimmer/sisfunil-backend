use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(crate::models::group::Group))]
#[diesel(table_name = crate::database::schema::types)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Type {
    pub id: u32,
    pub name: String,
    pub group_id: u32,
}
