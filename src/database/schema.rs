// @generated automatically by Diesel CLI.

diesel::table! {
    boxes (id) {
        id -> Unsigned<Integer>,
        #[sql_name = "location_id"]
        location -> Nullable<Unsigned<Integer>>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    categories (id) {
        id -> Unsigned<Integer>,
        #[sql_name = "type_id"]
        item_type -> Nullable<Unsigned<Integer>>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    groups (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    items (id) {
        id -> Unsigned<Integer>,
        #[sql_name = "category_id"]
        category -> Nullable<Unsigned<Integer>>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        date -> Nullable<Date>,
        description -> Nullable<Text>,
        sealed -> Bool,
        #[sql_name = "rate"]
        rating -> Unsigned<Smallint>,
        #[sql_name = "box_id"]
        item_box -> Nullable<Unsigned<Integer>>,
    }
}

diesel::table! {
    locations (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    types (id) {
        id -> Unsigned<Integer>,
        #[sql_name = "group_id"]
        group -> Nullable<Unsigned<Integer>>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
    }
}

diesel::joinable!(boxes -> locations (location));
diesel::joinable!(categories -> types (item_type));
diesel::joinable!(items -> boxes (item_box));
diesel::joinable!(items -> categories (category));
diesel::joinable!(types -> groups (group));

diesel::allow_tables_to_appear_in_same_query!(
    boxes,
    categories,
    groups,
    items,
    locations,
    types,
);
