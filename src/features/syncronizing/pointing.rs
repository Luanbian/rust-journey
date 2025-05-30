use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
};

struct SistemaPontuacao {
    pontuacoes: Arc<RwLock<HashMap<String, u32>>>,
}

impl SistemaPontuacao {
    fn new() -> Self {
        SistemaPontuacao {
            pontuacoes: Arc::new(RwLock::new(HashMap::<String, u32>::new())),
        }
    }

    fn atualizar_pontuacao(&self, jogador: String, pontos: u32) {
        let mut points = self.pontuacoes.write().unwrap();
        points.insert(jogador.clone(), pontos);
    }

    fn obter_pontuacao(&self, jogador: &str) -> Option<u32> {
        let points_all = self.pontuacoes.read().unwrap();
        let point = points_all.get(jogador).copied();
        point
    }

    fn listar_top_5(&self) -> Vec<(String, u32)> {
        let list = self.pontuacoes.read().unwrap();
        let mut ordered_list: Vec<(String, u32)> = list
            .iter()
            .map(|(jogador, pontos)| (jogador.clone(), *pontos))
            .collect();
        ordered_list.sort_by(|a, b| b.1.cmp(&a.1));
        ordered_list.truncate(5);
        ordered_list
    }

    fn total_jogadores(&self) -> usize {
        let list = self.pontuacoes.read().unwrap();
        list.len()
    }
}

pub fn teste_sistema_pontuacao() {
    println!("=== EXERCÍCIO 3: Sistema de Pontuação ===");

    let sistema = Arc::new(SistemaPontuacao::new());
    let mut handles = vec![];

    // Threads atualizando pontuações
    let jogadores = vec!["Alice", "Bob", "Charlie", "Diana"];
    for (i, jogador) in jogadores.iter().enumerate() {
        let sistema_clone = Arc::clone(&sistema);
        let jogador = jogador.to_string();
        let handle = thread::spawn(move || {
            sistema_clone.atualizar_pontuacao(jogador, (i * 100) as u32);
        });
        handles.push(handle);
    }

    // Threads lendo pontuações simultaneamente
    for _i in 0..5 {
        let sistema_clone = Arc::clone(&sistema);
        let handle = thread::spawn(move || {
            let alice_points = sistema_clone.obter_pontuacao("Alice");
            println!("pontuação da Alice: {:?}", alice_points)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Top 5: {:?}", sistema.listar_top_5());
    println!("Total: {:?}", sistema.total_jogadores());
}
