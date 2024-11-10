Claro! Aqui está o conteúdo completo em um único arquivo Markdown:


# Gerenciador de Tarefas

Este é um sistema de gerenciamento de tarefas simples, desenvolvido em Rust. O objetivo é permitir que o usuário gerencie suas tarefas, com a capacidade de realizar as quatro operações CRUD (Criar, Ler, Atualizar e Deletar) em uma entidade de domínio chamada `Tarefa`. As tarefas são armazenadas de forma persistente no sistema de arquivos, utilizando formatos binários eficientes, e com validação de dados de entrada.

## Funcionalidades

- **Criar Tarefa**: Adicionar novas tarefas ao sistema.
- **Ler Tarefas**: Carregar tarefas salvas a partir do sistema de arquivos.
- **Atualizar Tarefa**: Modificar o título, prioridade, data de vencimento ou status de uma tarefa.
- **Deletar Tarefa**: Remover uma tarefa do sistema.

## Estruturas de Dados

O sistema usa as seguintes estruturas de dados:

- **Tarefa**: Representa uma tarefa individual e contém:
  - `titulo`: Título da tarefa (String).
  - `prioridade`: Prioridade da tarefa (u8).
  - `data_vencimento`: Data de vencimento da tarefa (NaiveDate).
  - `status`: Status da tarefa, representado por um Enum (`Pendente`, `EmProgresso`, `Concluida`).
  
```rust
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
```

## Camada de Persistência

A camada de persistência é responsável por armazenar as tarefas de maneira eficiente no sistema de arquivos, utilizando o formato JSON. A biblioteca `serde` é usada para serialização e deserialização dos dados, e `chrono` para manipulação de datas.

As tarefas são salvas em um arquivo utilizando a função `salvar_tarefas`, e podem ser carregadas novamente com a função `carregar_tarefas`.

### Funções de Persistência:

- **salvar_tarefas**: Recebe um vetor de tarefas e salva no arquivo.
- **carregar_tarefas**: Carrega o vetor de tarefas a partir de um arquivo JSON.

```rust
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
```

## Validação de Dados

O sistema realiza validação de dados de entrada para garantir que as tarefas sejam inseridas com dados válidos. Caso os dados fornecidos não estejam no formato esperado, o sistema não os aceitará, evitando inconsistências.

## O que falta implementar


- **Operação de Atualização**: Implementar a funcionalidade para editar as tarefas existentes.
- **Operação de Deleção**: Implementar a funcionalidade para remover tarefas do sistema.
- **Validação de Entrada**: Melhorar a validação para garantir que as entradas do usuário estejam corretas (por exemplo, a data de vencimento não pode ser no passado).
- **Testes Automatizados**: Criar testes automatizados para as funções principais, garantindo o bom funcionamento do sistema.

## Como rodar

1. Clone o repositório:
   ```bash
   git clone https://github.com/username/gerenciador-de-tarefas.git
   cd gerenciador-de-tarefas
   ```

2. Compile o projeto:
   ```bash
   cargo build
   ```

3. Execute o programa:
   ```bash
   cargo run
   ```

## Tecnologias utilizadas

- **Rust**: Linguagem de programação usada para o desenvolvimento do sistema.
- **Serde**: Biblioteca para serialização e deserialização de dados JSON.
- **Chrono**: Biblioteca para manipulação de datas e horas.

## Contribuições

Este projeto está sendo desenvolvido por Paule Macedo. Você pode contribuir para o projeto através de pull requests.

## Prazo de entrega

O prazo de entrega é dia 02/12, onde haverá a apresentação dos trabalhos.



