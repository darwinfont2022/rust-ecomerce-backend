use serde::{Serialize};
use crate::modules::products::model::domain::product::Product;

#[derive(Serialize)]
pub struct ProductPage {
    pub page: i64,
    pub limit: i64,
    pub size: usize,
    pub results: Vec<Product>
}

