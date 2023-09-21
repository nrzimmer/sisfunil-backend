use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(crate::models::group::Group))]
#[diesel(table_name = crate::database::schema::kinds)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Kind {
    pub id: u32,
    pub name: String,
    #[serde(skip)]
    pub group_id: u32,
}

pub fn missing() -> Kind {
    Kind {
        id: 0,
        name: "Not found".to_string(),
        group_id: 0,
    }
}