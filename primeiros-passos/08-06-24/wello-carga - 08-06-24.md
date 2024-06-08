
``` bash 
cargo new hello_cargo 

cargo new --vcs=git # para adicionar o gitiginore
``` 

TOML - ( Tom's Obvious, Minimla language), que é o formato de configução do cargo


No rust, os Pacotes de código são chamados de crates

## Construind e executando um porjeto cargo

``` bash
cargo build

cargo run

cargo check

```

Cargo é inteligente se não houver mudança no arquivo ele não vai recompilar


cargo check é muito mais rapido que o `cargo build` por que bular a etapa de build

## Recapitulando os comandos

- Podemos criar um prjeto usando `cargo new`
- podemos construir um projeto usando cargo build
- podemos construir um e executar um porjeto em uma única etapa usando o ` cargo run `
- podemos construir um prjeto sem produzir um binário verificar erros usando  `cargo check`
- em vez de salva o resultado da construção no mesmo diretório do nosso códig , CArgo o armazena no diretório target/debug

## 