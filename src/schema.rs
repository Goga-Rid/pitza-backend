// @generated automatically by Diesel CLI.

diesel::table! {
    complaints (id) {
        id -> Int4,
        order_id -> Int4,
        user_id -> Int4,
        reason -> Text,
        comment -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    favorites (id) {
        id -> Int4,
        user_id -> Int4,
        product_id -> Int4,
    }
}

diesel::table! {
    order_items (id) {
        id -> Int4,
        order_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
        price -> Numeric,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Int4,
        total -> Numeric,
        status -> Text,
        address -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        price -> Numeric,
        category -> Text,
        image_url -> Nullable<Text>,
        available -> Bool,
        weight -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        user_id -> Int4,
        product_id -> Int4,
        rating -> Int4,
        comment -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        name -> Nullable<Text>,
        address -> Nullable<Text>,
        created_at -> Timestamp,
        role -> Text,
    }
}

diesel::joinable!(complaints -> orders (order_id));
diesel::joinable!(complaints -> users (user_id));
diesel::joinable!(favorites -> products (product_id));
diesel::joinable!(favorites -> users (user_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(reviews -> products (product_id));
diesel::joinable!(reviews -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    complaints,
    favorites,
    order_items,
    orders,
    products,
    reviews,
    users,
);
