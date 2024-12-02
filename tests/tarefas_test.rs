use tasks::tarefas::{Tarefa, Status, salvar_tarefas, carregar_tarefas}; // Corrigir o nome do módulo

use chrono::NaiveDate;
use std::fs;

#[test]
fn test_adicionar_tarefa() {
    let mut tarefas: Vec<Tarefa> = Vec::new();

    // Adicionar uma tarefa
    let tarefa = Tarefa {
        id: "1".to_string(),
        titulo: "Tarefa Teste".to_string(),
        prioridade: 3,
        data_vencimento: NaiveDate::from_ymd_opt(2024, 12, 1).expect("Data inválida"),
        status: Status::Pendente,
    };
    tarefas.push(tarefa);

    // Verificar se a tarefa foi adicionada
    assert_eq!(tarefas.len(), 1);
    assert_eq!(tarefas[0].titulo, "Tarefa Teste");
}

#[test]
fn test_salvar_carregar_tarefas() {
    let caminho = "teste_tarefas.json";

    let tarefas_inicial: Vec<Tarefa> = vec![
        Tarefa {
            id: "1".to_string(),
            titulo: "Tarefa 1".to_string(),
            prioridade: 2,
            data_vencimento: NaiveDate::from_ymd_opt(2024, 12, 1).expect("Data inválida"),
            status: Status::Pendente,
        },
        Tarefa {
            id: "2".to_string(),
            titulo: "Tarefa 2".to_string(),
            prioridade: 4,
            data_vencimento: NaiveDate::from_ymd_opt(2024, 12, 2).expect("Data inválida"),
            status: Status::EmProgresso,
        },
    ];

    // Salvar as tarefas no arquivo
    salvar_tarefas(&tarefas_inicial, caminho).expect("Falha ao salvar tarefas");

    // Carregar as tarefas do arquivo
    let tarefas_carregadas = carregar_tarefas(caminho).expect("Falha ao carregar tarefas");

    // Verificar se as tarefas foram carregadas corretamente
    assert_eq!(tarefas_inicial.len(), tarefas_carregadas.len());
    assert_eq!(tarefas_inicial[0].titulo, tarefas_carregadas[0].titulo);

    // Limpar arquivo de teste após o teste
    fs::remove_file(caminho).expect("Falha ao remover arquivo de teste");
}
