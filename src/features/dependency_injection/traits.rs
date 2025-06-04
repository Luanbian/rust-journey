use crate::features::dependency_injection::models::Product;
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepositoryTrait {
    async fn get_product(&self, product_id: &str) -> Result<Product, String>;
    async fn add_product(&self, product: Product) -> Result<(), String>;
}
