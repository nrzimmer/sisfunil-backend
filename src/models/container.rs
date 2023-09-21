use diesel::prelude::{Associations, Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(crate::models::location::Location))]
#[diesel(table_name = crate::database::schema::containers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Container {
    pub id: u32,
    pub description: String,
    pub location_id: u32,
}
