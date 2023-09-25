// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Unsigned<Integer>,
        kind_id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    containers (id) {
        id -> Unsigned<Integer>,
        location_id -> Unsigned<Integer>,
        description -> Text,
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
        container_id -> Unsigned<Integer>,
    }
}

diesel::table! {
    kinds (id) {
        id -> Unsigned<Integer>,
        group_id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
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

diesel::joinable!(categories -> kinds (kind_id));
diesel::joinable!(containers -> locations (location_id));
diesel::joinable!(items -> categories (category_id));
diesel::joinable!(items -> containers (container_id));
diesel::joinable!(kinds -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories, containers, groups, items, kinds, locations,
);
