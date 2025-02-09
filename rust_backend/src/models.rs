use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{users, publishers, games, platforms, categories, cartitems, gamecategories, gameplatforms, orders, reviews, orderitems, wishlists, usergames};  // Import relevant tables

// User model
#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct User {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub profile_picture_url: Option<String>,
    pub is_publisher: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "publishers"]
pub struct Publisher {
    pub publisher_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub publisher_name: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// Game model
#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "games"]
pub struct Game {
    pub game_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub discount: Option<f64>,
    pub release_date: Option<chrono::NaiveDate>,
    pub publisher_id: Option<uuid::Uuid>,
    pub rating: Option<f64>,
    pub reviews: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "platforms"]
pub struct Platform {
    pub platform_id: uuid::Uuid,
    pub name: String,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "categories"]
pub struct Category {
    pub category_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
}


// CartItem model
#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "cartitems"]
pub struct CartItem {
    pub cartitem_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub game_id: uuid::Uuid,
    pub quantity: i32,
    pub added_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "gamecategories"]
pub struct GameCategory {
    pub game_category_id: uuid::Uuid,
    pub game_id: uuid::Uuid,
    pub category_id: uuid::Uuid,
}

// Order model
#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "orders"]
pub struct Order {
    pub order_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub total_price: f64,
    pub status: String,
    pub payment_method: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "reviews"]
pub struct Review {
    pub review_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub game_id: uuid::Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub will_recommend: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "orderitems"]
pub struct OrderItem {
    pub order_item_id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub game_id: uuid::Uuid,
    pub quantity: i32,
    pub unit_price: f64,
    pub discount: f64,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "usergames"]
pub struct UserGame {
    pub user_game_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub game_id: uuid::Uuid,
    pub quantity: Option<i32>,
    pub purchased_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "gameplatforms"]
pub struct GamePlatform {
    pub game_platform_id: uuid::Uuid,
    pub game_id: uuid::Uuid,
    pub platform_id: uuid::Uuid,
}


#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "wishlists"]
pub struct Wishlist {
    pub wishlist_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub game_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

