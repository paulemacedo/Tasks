use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::str::FromStr;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Clone)]
struct Tarefa {
    titulo: String,
    prioridade: u8,
    data_vencimento: NaiveDate,
    status: Status,
}

#[derive(Serialize, Deserialize, Clone)]
enum Status {
    Pendente,
    EmProgresso,
    Concluida,
}

fn salvar_tarefas(tarefas: &Vec<Tarefa>, caminho: &str) -> std::io::Result<()> {
    let arquivo = File::create(caminho)?;
    let writer = BufWriter::new(arquivo);
    serde_json::to_writer(writer, &tarefas)?;
    Ok(())
}

fn carregar_tarefas(caminho: &str) -> std::io::Result<Vec<Tarefa>> {
    let arquivo = File::open(caminho)?;
    let reader = BufReader::new(arquivo);
    let tarefas = serde_json::from_reader(reader)?;
    Ok(tarefas)
}

fn ler_entrada() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler entrada");
    entrada.trim().to_string()
}

fn adicionar_tarefa(tarefas: &mut Vec<Tarefa>) {
    println!("Digite o título da tarefa:");
    let titulo = ler_entrada();

    println!("Digite a prioridade (1-5):");
    let prioridade: u8 = loop {
        match u8::from_str(&ler_entrada()) {
            Ok(num) if num >= 1 && num <= 5 => break num,
            _ => println!("Por favor, digite um número entre 1 e 5"),
        }
    };

    println!("Digite a data de vencimento (AAAA-MM-DD):");
    let data_vencimento: NaiveDate = loop {
        match NaiveDate::parse_from_str(&ler_entrada(), "%Y-%m-%d") {
            Ok(data) => break data,
            Err(_) => println!("Formato inválido. Use AAAA-MM-DD"),
        }
    };

    let tarefa = Tarefa {
        titulo,
        prioridade,
        data_vencimento,
        status: Status::Pendente,
    };

    tarefas.push(tarefa);
    println!("Tarefa adicionada com sucesso!");
}

fn listar_tarefas(tarefas: &Vec<Tarefa>) {
    if tarefas.is_empty() {
        println!("Nenhuma tarefa encontrada.");
        return;
    }

    for (i, tarefa) in tarefas.iter().enumerate() {
        let status_str = match tarefa.status {
            Status::Pendente => "Pendente",
            Status::EmProgresso => "Em Progresso",
            Status::Concluida => "Concluída",
        };

        println!(
            "{}. {} (Prioridade: {}, Vencimento: {}, Status: {})",
            i + 1,
            tarefa.titulo,
            tarefa.prioridade,
            tarefa.data_vencimento,
            status_str
        );
    }
}

fn atualizar_status(tarefas: &mut Vec<Tarefa>) {
    listar_tarefas(tarefas);

    if tarefas.is_empty() {
        return;
    }

    println!("Digite o número da tarefa que deseja atualizar:");
    let indice: usize = loop {
        match usize::from_str(&ler_entrada()) {
            Ok(num) if num >= 1 && num <= tarefas.len() => break num - 1,
            _ => println!("Por favor, digite um número válido"),
        }
    };

    println!("Escolha o novo status:");
    println!("1. Pendente");
    println!("2. Em Progresso");
    println!("3. Concluída");

    let novo_status = loop {
        match ler_entrada().as_str() {
            "1" => break Status::Pendente,
            "2" => break Status::EmProgresso,
            "3" => break Status::Concluida,
            _ => println!("Por favor, escolha uma opção válida (1-3)"),
        }
    };

    tarefas[indice].status = novo_status;
    println!("Status atualizado com sucesso!");
}

fn main() {
    println!("Bem-vindo ao Gerenciador de Tarefas!");
    let arquivo = "tarefas.json";
    let mut tarefas = carregar_tarefas(arquivo).unwrap_or_else(|_| Vec::new());

    loop {
        println!("\nEscolha uma opção:");
        println!("1. Adicionar tarefa");
        println!("2. Listar tarefas");
        println!("3. Atualizar status");
        println!("4. Sair");

        match ler_entrada().as_str() {
            "1" => adicionar_tarefa(&mut tarefas),
            "2" => listar_tarefas(&tarefas),
            "3" => atualizar_status(&mut tarefas),
            "4" => break,
            _ => println!("Opção inválida!"),
        }

        if let Err(e) = salvar_tarefas(&tarefas, arquivo) {
            eprintln!("Erro ao salvar tarefas: {}", e);
        }
    }

    println!("Obrigado por usar o Gerenciador de Tarefas!");
}