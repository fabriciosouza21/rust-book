# Organizando testes

## Teste unitario

Você colocará os testes unitários no diretório src em cada arquivo com o código que eles estão testando. A convenção é criar um módulo nomeado tests em cada arquivo para conter as funções de teste e anotar o módulo com cfg(test).

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

## Teste de funções privads

```rust
pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
```


este teste, trazemos todos os testsitens do pai do módulo para o escopo com use super::*, e então o teste pode chamar internal_adder

## Teste de integração

Criamos um diretório de testes no nível superior do diretório do nosso projeto, ao lado de src . O Cargo sabe procurar por arquivos de teste de integração neste diretório. Podemos então fazer quantos arquivos de teste quisermos, e o Cargo compilará cada um dos arquivos como uma caixa individual.

src/lib.rs

```bash
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs

```

tests/integration_test.rs

Não precisamos anotar nenhum código em tests/integration_test.rs com #[cfg(test)]. O Cargo trata o diretório tests de forma especial e compila arquivos neste diretório somente quando executamos


```rust
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```



```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.31s
     Running unittests src/lib.rs (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```
As três seções de saída incluem os testes unitários, o teste de integração e os testes de documentação. Observe que se qualquer teste em uma seção falhar, as seções seguintes não serão executadas. Por exemplo, se um teste unitário falhar, não haverá nenhuma saída para testes de integração e documentação porque esses testes só serão executados se todos os testes unitários forem aprovados.


Ainda podemos executar uma função de teste de integração específica especificando o nome da função de teste como um argumento para cargo test. Para executar todos os testes em um arquivo de teste de integração específico, use o --testargumento de cargo test seguido pelo nome do arquivo:

```bash

$ cargo test --test integration_test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running tests/integration_test.rs (target/debug/deps/integration_test-82e7799c1bc62298)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## Submodulos de teste

Queríamos apenas compartilhar algum código com os outros arquivos de teste de integração. Para evitar que commonapareça na saída do teste, em vez de criar tests/common.rs , criaremos tests/common/mod.rs .

```bash
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

Nomear o arquivo dessa forma diz ao Rust para não tratar o common módulo como um arquivo de teste de integração. Quando movemos o setup código da função para tests/common/mod.rs e excluímos o arquivo tests/common.rs , a seção na saída do teste não aparecerá mais. Arquivos em subdiretórios do diretório tests não são compilados como caixas separadas ou têm seções na saída do teste.

```rust

use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
```
