mod bank_account;
use bank_account::teste_conta_bancaria;
mod shared_list;
use shared_list::teste_lista_compartilhada;
mod pointing;
use pointing::teste_sistema_pontuacao;
mod smart_cache;
use smart_cache::teste_cache_inteligente;

pub fn main() {
    println!("🎯 EXERCÍCIOS DE MUTEX E RWLOCK");
    println!("================================\n");

    teste_conta_bancaria();
    teste_lista_compartilhada();
    teste_sistema_pontuacao();
    teste_cache_inteligente();
}
