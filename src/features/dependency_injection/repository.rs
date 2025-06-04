use crate::features::dependency_injection::{models::Product, traits::ProductRepositoryTrait};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

pub struct MemoryProductRepository {
    products: Arc<Mutex<Vec<Product>>>,
}

impl MemoryProductRepository {
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl ProductRepositoryTrait for MemoryProductRepository {
    async fn get_product(&self, product_id: &str) -> Result<Product, String> {
        println!("Buscando produto com ID no repository: {}", product_id);
        let products = self.products.lock().unwrap();
        let found_product = products.iter().find(|prod| prod.id == product_id);
        match found_product {
            Some(found) => Ok(found.clone()),
            None => Err("Not found".to_string()),
        }
    }

    async fn add_product(&self, product: Product) -> Result<(), String> {
        println!("Adicionando produto no repository: {:?}", product);
        let mut products = self.products.lock().unwrap();
        if products.iter().any(|p| p.id == product.id) {
            return Err("Produto já existe".to_string());
        }

        products.push(product);
        Ok(())
    }
}
