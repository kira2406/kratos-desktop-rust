// @generated automatically by Diesel CLI.

diesel::table! {
    cartitems (cartitem_id) {
        cartitem_id -> Uuid,
        user_id -> Uuid,
        game_id -> Uuid,
        quantity -> Int4,
        added_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    categories (category_id) {
        category_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    gamecategories (game_category_id) {
        game_category_id -> Uuid,
        game_id -> Uuid,
        category_id -> Uuid,
    }
}

diesel::table! {
    gameplatforms (game_platform_id) {
        game_platform_id -> Uuid,
        game_id -> Uuid,
        platform_id -> Uuid,
    }
}

diesel::table! {
    games (game_id) {
        game_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        price -> Numeric,
        discount -> Nullable<Float8>,
        release_date -> Nullable<Date>,
        publisher_id -> Nullable<Uuid>,
        rating -> Nullable<Float8>,
        reviews -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    orderitems (order_item_id) {
        order_item_id -> Uuid,
        order_id -> Uuid,
        game_id -> Uuid,
        quantity -> Int4,
        unit_price -> Numeric,
        discount -> Nullable<Numeric>,
    }
}

diesel::table! {
    orders (order_id) {
        order_id -> Uuid,
        user_id -> Uuid,
        total_price -> Numeric,
        status -> Text,
        payment_method -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    platforms (platform_id) {
        platform_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    publishers (publisher_id) {
        publisher_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        publisher_name -> Varchar,
        description -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    reviews (review_id) {
        review_id -> Uuid,
        user_id -> Uuid,
        game_id -> Uuid,
        rating -> Int4,
        comment -> Nullable<Text>,
        will_recommend -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    usergames (user_game_id) {
        user_game_id -> Uuid,
        user_id -> Uuid,
        game_id -> Uuid,
        quantity -> Nullable<Int4>,
        purchased_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        password_hash -> Text,
        profile_picture_url -> Nullable<Text>,
        is_publisher -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    wishlists (wishlist_id) {
        wishlist_id -> Uuid,
        user_id -> Uuid,
        game_id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(cartitems -> games (game_id));
diesel::joinable!(cartitems -> users (user_id));
diesel::joinable!(gamecategories -> categories (category_id));
diesel::joinable!(gamecategories -> games (game_id));
diesel::joinable!(gameplatforms -> games (game_id));
diesel::joinable!(gameplatforms -> platforms (platform_id));
diesel::joinable!(games -> publishers (publisher_id));
diesel::joinable!(orderitems -> games (game_id));
diesel::joinable!(orderitems -> orders (order_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(publishers -> users (user_id));
diesel::joinable!(reviews -> games (game_id));
diesel::joinable!(reviews -> users (user_id));
diesel::joinable!(usergames -> games (game_id));
diesel::joinable!(usergames -> users (user_id));
diesel::joinable!(wishlists -> games (game_id));
diesel::joinable!(wishlists -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cartitems,
    categories,
    gamecategories,
    gameplatforms,
    games,
    orderitems,
    orders,
    platforms,
    publishers,
    reviews,
    usergames,
    users,
    wishlists,
);
