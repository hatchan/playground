use axum::routing::get;

mod handlers;

pub fn create_api() -> axum::Router {
    let router = axum::Router::new().route("/products/:product_id", get(handlers::get_product));

    router
}
