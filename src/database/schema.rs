// @generated automatically by Diesel CLI.

diesel::table! {
    boxes (id) {
        id -> Unsigned<Integer>,
        location_id -> Unsigned<Integer>,
        description -> Text,
    }
}

diesel::table! {
    categories (id) {
        id -> Unsigned<Integer>,
        type_id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    groups (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    items (id) {
        id -> Unsigned<Integer>,
        category_id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        date -> Date,
        description -> Text,
        sealed -> Bool,
        rate -> Unsigned<Tinyint>,
        box_id -> Unsigned<Integer>,
    }
}

diesel::table! {
    locations (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    types (id) {
        id -> Unsigned<Integer>,
        group_id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(boxes -> locations (location_id));
diesel::joinable!(categories -> types (type_id));
diesel::joinable!(items -> boxes (box_id));
diesel::joinable!(items -> categories (category_id));
diesel::joinable!(types -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    boxes,
    categories,
    groups,
    items,
    locations,
    types,
);
