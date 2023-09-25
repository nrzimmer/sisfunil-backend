use diesel::QueryResult;
use serde::Serialize;

use crate::models::category::Category;
use crate::models::container::Container;
use crate::models::group::Group;
use crate::models::item::Item;
use crate::models::kind::Kind;
use crate::models::location::Location;

pub type QueryResultDTO<T> = QueryResult<ResultDTO<Vec<T>>>;

#[derive(Serialize)]
pub struct ItemDTO {
    #[serde(flatten)]
    pub item: Item,
    pub container: ContainerDTO,
    pub category: CategoryDTO,
}

#[derive(Serialize)]
pub struct ContainerDTO {
    #[serde(flatten)]
    pub container: Container,
    pub location: Location,
}

#[derive(Serialize)]
pub struct CategoryDTO {
    #[serde(flatten)]
    pub category: Category,
    pub kind: KindDTO,
}

#[derive(Serialize)]
pub struct KindDTO {
    #[serde(flatten)]
    pub kind: Kind,
    pub group: Group,
}

#[derive(Serialize)]
pub struct ResultDTO<T> {
    pub count: i64,
    pub data: T,
}

pub fn get_results_dto<T>(data: QueryResult<Vec<T>>, count: QueryResult<i64>) -> QueryResultDTO<T> {
    match data {
        Ok(v) => Ok(ResultDTO {
            count: count.unwrap(),
            data: v,
        }),
        Err(e) => Err(e),
    }
}

pub fn get_result_dto<T>(data: QueryResult<T>) -> QueryResultDTO<T> {
    get_results_dto(
        match data {
            Ok(v) => Ok(vec![v]),
            Err(e) => Err(e),
        },
        Ok(1),
    )
}
