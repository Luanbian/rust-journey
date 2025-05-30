use std::sync::{Arc, Mutex};
use std::thread;

struct ListaCompartilhada<T> {
    itens: Arc<Mutex<Vec<T>>>,
}

impl<T> ListaCompartilhada<T> {
    fn new() -> Self {
        ListaCompartilhada {
            itens: Arc::new(Mutex::new(Vec::<T>::new())),
        }
    }

    fn adicionar(&self, item: T) {
        let mut list = self.itens.lock().unwrap();
        list.push(item);
    }

    fn tamanho(&self) -> usize {
        let list = self.itens.lock().unwrap();
        list.len()
    }
}

pub fn teste_lista_compartilhada() {
    println!("=== EXERCÍCIO 2: Lista Compartilhada ===");

    let lista = Arc::new(ListaCompartilhada::new());
    let mut handles = vec![];

    // Múltiplas threads adicionando números
    for i in 0..10 {
        let list_clone = Arc::clone(&lista);
        let handle = thread::spawn(move || list_clone.adicionar(i));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Lista final tem {} itens", lista.tamanho());
    // Deve imprimir 10
}
