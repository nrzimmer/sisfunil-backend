use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(crate::models::kind::Kind))]
#[diesel(table_name = crate::database::schema::categories)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub kind_id: u32,
}
