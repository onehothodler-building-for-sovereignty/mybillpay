use axum::Router;

pub mod routes;

pub fn app() -> Router {
    routes::routes()
}
