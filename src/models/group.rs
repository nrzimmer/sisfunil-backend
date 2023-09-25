use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = crate::database::schema::groups)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Group {
    pub id: u32,
    pub name: String,
}

pub fn missing() -> Group {
    Group {
        id: 0,
        name: "Not found".to_string(),
    }
}
