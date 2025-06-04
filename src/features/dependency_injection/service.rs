use crate::features::dependency_injection::{models::Product, traits::ProductRepositoryTrait};
use std::sync::Arc;

pub struct ProductService {
    repository: Arc<dyn ProductRepositoryTrait>,
}

impl ProductService {
    pub fn new(repository: Arc<dyn ProductRepositoryTrait>) -> Self {
        Self { repository }
    }

    pub async fn create_product(&self, data: Product) -> Result<(), String> {
        println!("Criando novo produto");
        let insert = self.repository.add_product(data).await;
        match insert {
            Ok(_) => {
                println!("Produto criado com sucesso");
                Ok(())
            }
            Err(e) => {
                println!("Erro ao criar produto: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_product(&self, product_id: &str) -> Result<Product, String> {
        println!("Buscando produto com ID: {}", product_id);
        let product = self.repository.get_product(product_id).await;
        match product {
            Ok(prod) => {
                println!("Produto encontrado: {:?}", prod);
                Ok(prod)
            }
            Err(e) => {
                println!("Erro ao buscar produto: {}", e);
                Err(e)
            }
        }
    }
}
