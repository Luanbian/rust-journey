use std::sync::{Arc, Mutex};
use std::thread;

struct ContaBancaria {
    saldo: Arc<Mutex<f64>>,
}

impl ContaBancaria {
    fn new(saldo_inicial: f64) -> Self {
        ContaBancaria {
            saldo: Arc::new(Mutex::new(saldo_inicial)),
        }
    }

    fn depositar(&self, valor: f64) {
        let mut saldo = self.saldo.lock().unwrap();
        *saldo += valor;
    }

    fn sacar(&self, valor: f64) -> bool {
        let mut saldo = self.saldo.lock().unwrap();
        if *saldo < valor {
            return false;
        }
        *saldo -= valor;
        return true;
    }

    fn consultar_saldo(&self) -> f64 {
        let saldo = self.saldo.lock().unwrap();
        *saldo
    }
}

pub fn teste_conta_bancaria() {
    println!("=== EXERCÍCIO 1: Conta Bancária ===");

    let conta = Arc::new(ContaBancaria::new(1000.0));
    let mut handles = vec![];

    // Múltiplas threads fazendo depósitos
    for i in 0..5 {
        let conta_clone = Arc::clone(&conta);
        let handle = thread::spawn(move || {
            conta_clone.depositar(100.0);
            if i == 4 {
                conta_clone.sacar(100.0);
                println!("Thread {} sacou 100", i);
            }
            println!("Thread {} depositou 100", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Saldo final: {}", conta.consultar_saldo());
    // Deve imprimir 1400.0
}
