use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
}

#[derive(
    Clone, Debug, PartialEq, Eq, derive_more::Display, serde::Serialize, serde::Deserialize,
)]
pub struct ProductId(uuid::Uuid);

impl ProductId {
    pub fn new(input: &str) -> Result<Self, InvalidProductId> {
        let id = uuid::Uuid::parse_str(input).map_err(|_| InvalidProductId)?;

        Ok(Self(id))
    }
}

impl FromStr for ProductId {
    type Err = InvalidProductId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
#[error("invalid product id")]
pub struct InvalidProductId;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum GetProductError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_id_new_ok() {
        let product_id = ProductId::new("8d65b1af-9ceb-410d-9594-71c14426756f").unwrap();

        assert_eq!(
            product_id.0.to_string(),
            "8d65b1af-9ceb-410d-9594-71c14426756f"
        );
    }

    #[test]
    fn test_product_id_new_err() {
        let input = "invalid-product-id";
        let invalid_product_id = ProductId::new(input);

        assert_eq!(invalid_product_id, Err(InvalidProductId));
    }
}
