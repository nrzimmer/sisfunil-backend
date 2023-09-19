use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::models::location::Location;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::boxes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemBox {
    pub id: u32,
    pub description: String,
    pub location: Location,
}
