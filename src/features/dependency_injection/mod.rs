use std::sync::Arc;

use crate::features::dependency_injection::{
    models::Product, repository::MemoryProductRepository, service::ProductService,
};

pub mod models;
pub mod repository;
pub mod service;
pub mod traits;

pub async fn main() {
    println!("Dependency Injection Example");

    // Example usage of the repository and service
    let repository = MemoryProductRepository::new();
    let service = ProductService::new(Arc::new(repository));

    // Example product
    let product = Product {
        id: "1".to_string(),
        name: "Example Product".to_string(),
        price: 10.0,
        stock: 5,
    };

    // Create a product
    match service.create_product(product.clone()).await {
        Ok(_) => println!("Product created successfully."),
        Err(e) => println!("Failed to create product: {}", e),
    }

    // Get the product
    match service.get_product(&product.id).await {
        Ok(p) => println!("Product found: {:?}", p),
        Err(e) => println!("Product not found: {}", e),
    }
}
