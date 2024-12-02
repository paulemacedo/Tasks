use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::str::FromStr;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct Tarefa {
    id: String,  // Mudança chave: usar String em vez de Uuid diretamente
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
        id: Uuid::new_v4().to_string(),  // Converte UUID para String
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

    for tarefa in tarefas {
        let status_str = match tarefa.status {
            Status::Pendente => "Pendente",
            Status::EmProgresso => "Em Progresso",
            Status::Concluida => "Concluída",
        };

        println!(
            "ID: {} | {} (Prioridade: {}, Vencimento: {}, Status: {})",
            tarefa.id,
            tarefa.titulo,
            tarefa.prioridade,
            tarefa.data_vencimento,
            status_str
        );
    }
}

fn atualizar_tarefa(tarefas: &mut Vec<Tarefa>) {
    listar_tarefas(tarefas);

    if tarefas.is_empty() {
        return;
    }

    println!("Digite o ID da tarefa que deseja atualizar:");
    let id_str = ler_entrada();

    if let Some(tarefa) = tarefas.iter_mut().find(|t| t.id == id_str) {
        println!("Escolha o que deseja atualizar:");
        println!("1. Título");
        println!("2. Prioridade");
        println!("3. Data de Vencimento");
        println!("4. Status");

        match ler_entrada().as_str() {
            "1" => {
                println!("Digite o novo título:");
                tarefa.titulo = ler_entrada();
            },
            "2" => {
                println!("Digite a nova prioridade (1-5):");
                tarefa.prioridade = loop {
                    match u8::from_str(&ler_entrada()) {
                        Ok(num) if num >= 1 && num <= 5 => break num,
                        _ => println!("Por favor, digite um número entre 1 e 5"),
                    }
                };
            },
            "3" => {
                println!("Digite a nova data de vencimento (AAAA-MM-DD):");
                tarefa.data_vencimento = loop {
                    match NaiveDate::parse_from_str(&ler_entrada(), "%Y-%m-%d") {
                        Ok(data) => break data,
                        Err(_) => println!("Formato inválido. Use AAAA-MM-DD"),
                    }
                };
            },
            "4" => {
                println!("Escolha o novo status:");
                println!("1. Pendente");
                println!("2. Em Progresso");
                println!("3. Concluída");

                tarefa.status = match ler_entrada().as_str() {
                    "1" => Status::Pendente,
                    "2" => Status::EmProgresso,
                    "3" => Status::Concluida,
                    _ => {
                        println!("Opção inválida!");
                        return;
                    }
                };
            },
            _ => {
                println!("Opção inválida!");
                return;
            }
        }

        println!("Tarefa atualizada com sucesso!");
    } else {
        println!("Tarefa não encontrada!");
    }
}

fn excluir_tarefa(tarefas: &mut Vec<Tarefa>) {
    listar_tarefas(tarefas);

    if tarefas.is_empty() {
        return;
    }

    println!("Digite o ID da tarefa que deseja excluir:");
    let id_str = ler_entrada();

    let tamanho_inicial = tarefas.len();
    tarefas.retain(|t| t.id != id_str);

    if tarefas.len() < tamanho_inicial {
        println!("Tarefa excluída com sucesso!");
    } else {
        println!("Tarefa não encontrada!");
    }
}

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