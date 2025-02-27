## Requisitos
- Garanta que tenha instalado Rust lang, e Cargo.
- Se necessário siga a documentação oficial: **https://doc.rust-lang.org/book/ch01-01-installation.html**

## Descrição dos Arquivos

- **main.rs**: Arquivo principal que inicializa a aplicação.
- **gui.rs**: Contém a lógica da interface gráfica da calculadora.
- **style.rs**: Define o estilo personalizado da interface.
- **Open_Sans/**: Diretório contendo a fonte Open Sans utilizada na interface.

## Funcionalidades

A calculadora suporta as seguintes operações:

- Soma (`+`)
- Subtração (`-`)
- Multiplicação (`*`)
- Divisão (`/`)
- Raiz quadrada (`sqrt`)
- Raiz cúbica (`cbrt`)
- Potenciação (`pow`)
- Logaritmo (`log`)
- Logaritmo natural (`ln`)
- Exponencial (`exp`)

## Como Executar

1. Clone o repositório:
    ```sh
    git clone https://github.com/seu-usuario/calculadora-terminal.git
    cd calculadora-terminal
    ```

2. Execute o projeto:
    ```sh
    cargo build
    cargo run
    ```

## Estilo Personalizado

O estilo da interface foi personalizado utilizando a fonte Open Sans e cores específicas presentes em **styles.rs**.

## Contribuição

Sinta-se à vontade para abrir issues e pull requests. Toda contribuição é bem-vinda!
