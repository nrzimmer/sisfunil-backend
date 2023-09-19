use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(crate::models::location::Location))]
#[diesel(table_name = crate::database::schema::boxes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Box {
    pub id: u32,
    pub description: String,
    pub location_id: u32
}
