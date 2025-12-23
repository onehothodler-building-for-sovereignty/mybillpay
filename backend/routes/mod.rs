use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
}

pub async fn health() -> &'static str {
    "OK"
}

pub async fn list_items() -> Json<Vec<Item>> {
    // Placeholder data — replace with DB call
    let items = vec![Item { id: 1, name: "Sample Item".into() }];
    Json(items)
}

pub async fn create_item(Json(payload): Json<Item>) -> Json<Item> {
    // Placeholder — persist the item in a real app
    Json(payload)
}

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/items", get(list_items).post(create_item))
}
