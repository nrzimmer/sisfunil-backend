use serde::Serialize;

use crate::models::category::Category;
use crate::models::container::Container;
use crate::models::group::Group;
use crate::models::item::Item;
use crate::models::kind::Kind;
use crate::models::location::Location;

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
