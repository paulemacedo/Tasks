use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize)]
struct Tarefa {
    titulo: String,
    prioridade: u8,
    data_vencimento: NaiveDate,
    status: Status,
}

#[derive(Serialize, Deserialize)]
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

fn main() {
    println!("Bem-vindo ao Gerenciador de Tarefas!");
}
