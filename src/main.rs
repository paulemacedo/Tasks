pub mod tarefas;
use crate::tarefas::{ salvar_tarefas, carregar_tarefas, ler_entrada, adicionar_tarefa, listar_tarefas, atualizar_tarefa, excluir_tarefa};

fn main() {
    println!("Bem-vindo ao Gerenciador de Tarefas!");
    let arquivo = "tarefas.json";
    let mut tarefas = carregar_tarefas(arquivo).unwrap_or_else(|_| Vec::new());

    loop {
        println!("\nEscolha uma opção:");
        println!("1. Adicionar tarefa");
        println!("2. Listar tarefas");
        println!("3. Atualizar tarefa");
        println!("4. Excluir tarefa");
        println!("5. Sair");

        match ler_entrada().as_str() {
            "1" => adicionar_tarefa(&mut tarefas),
            "2" => listar_tarefas(&tarefas),
            "3" => atualizar_tarefa(&mut tarefas),
            "4" => excluir_tarefa(&mut tarefas),
            "5" => break,
            _ => println!("Opção inválida!"),
        }

        if let Err(e) = salvar_tarefas(&tarefas, arquivo) {
            eprintln!("Erro ao salvar tarefas: {}", e);
        }
    }

    println!("Obrigado por usar o Gerenciador de Tarefas!");
}