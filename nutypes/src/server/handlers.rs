use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;

use crate::models::{Product, ProductId};

pub(crate) async fn get_product(Path(product_id): Path<ProductId>) -> impl IntoResponse {
    Json(Product {
        id: product_id,
        name: "Example Product".to_string(),
    })
}
