use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Clone)]
struct CacheItem {
    valor: String,
    expires_at: u64,
}

struct CacheInteligente {
    dados: Arc<RwLock<HashMap<String, CacheItem>>>,
}

impl CacheInteligente {
    fn new() -> Self {
        CacheInteligente {
            dados: Arc::new(RwLock::new(HashMap::<String, CacheItem>::new())),
        }
    }

    fn inserir(&self, chave: String, valor: String, ttl_segundos: u64) {
        let mut dados = self.dados.write().unwrap();
        let cache: CacheItem = CacheItem {
            valor,
            expires_at: ttl_segundos,
        };
        dados.insert(chave, cache);
    }

    fn obter(&self, chave: &str) -> Option<String> {
        {
            let dados = self.dados.read().unwrap();
            if let Some(item) = dados.get(chave) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                if item.expires_at > now {
                    return Some(item.valor.clone());
                }
            }
        }

        let mut dados = self.dados.write().unwrap();
        dados.remove(chave);
        None
    }

    fn limpar_expirados(&self) -> usize {
        let mut dados = self.dados.write().unwrap();
        let expired_list: Vec<String> = dados
            .iter()
            .filter(|(_, valor)| {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                valor.expires_at <= now
            })
            .map(|(chave, _)| chave.clone())
            .collect();

        for chave in &expired_list {
            dados.remove(chave);
        }

        expired_list.len()
    }

    fn estatisticas(&self) -> (usize, usize) {
        let total = self.dados.read().unwrap();
        let expired: Vec<(&String, &CacheItem)> = total
            .iter()
            .filter(|(_chave, valor)| {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                valor.expires_at <= now
            })
            .collect();
        (total.len(), expired.len())
    }
}

pub fn teste_cache_inteligente() {
    println!("=== EXERCÍCIO 4: Cache Inteligente ===");

    let cache = Arc::new(CacheInteligente::new());
    let mut handles = vec![];

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 1. Inserir alguns itens com TTL curto (2 segundos)
    cache.inserir(
        String::from("Luan"),
        String::from("Idade: 23 anos"),
        now + 2,
    );

    // 2. Inserir alguns itens com TTL longo (10 segundos)
    cache.inserir(
        String::from("Maria"),
        String::from("Idade: 21 anos"),
        now + 10,
    );
    // 3. Criar threads que leem do cache
    for _i in 0..5 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            println!(
                "[Thread] Luan: {}",
                cache_clone
                    .obter("Luan")
                    .unwrap_or_else(|| "Item expirado".to_string())
            );
            println!(
                "[Thread] Maria: {}",
                cache_clone
                    .obter("Maria")
                    .unwrap_or_else(|| "Item expirado".to_string())
            );
            println!("Statics: {:?}", cache_clone.estatisticas());
            // 4. Aguardar 3 segundos
            thread::sleep(Duration::from_secs(3));
            // 5. Verificar quais itens expiraram
            cache_clone.limpar_expirados();
            println!(
                "[Thread] Estatísticas após limpeza: {:?}",
                cache_clone.estatisticas()
            );
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Teste do cache concluído!");
}
