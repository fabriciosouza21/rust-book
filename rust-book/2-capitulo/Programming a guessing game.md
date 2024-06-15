
15-06-24

Nesse capitulo vamos aprender sobre let métodos match, funções associadas, caixas externas.

Será um jogo de adivinhação, o programa era gerar um inteiro aleatório entre 1 e 100, depois de inserida o programa inidicara se a estimativa é muito baixa ou muito alta.


## processando o input   

``` rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

## variaveis

Em Rust as variáveis são imutáveis por padrão, para tornar uma variável mutáve, adicionamos mut antes do nome da variável 

``` rust

let apples = 5; // immutable
let mut bananas = 5; // mutable
```

String é um tipo de string fornecido pela biblioteca padrão que é um pedaço de texto codificado em UTF-8 que pode ser ampliado.

### Recebendo entrada do usuário

 ``` rust
    io::stdin()
        .read_line(&mut guess)

 
 ```

### Lidando com falhas potenciasi com Resultado

``` rust
   io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
```


read_line coloca tudo o que o usuário digita na string que passamos para ele, mas também retorna um Result valor, que é um enum,  os possiveis valores para o resulte são OK e Err.

se não ligar expect, o programa será compilado, mas você recebera um aviso

o expect é forma de tratamento de erro de um result, 

### interpolação

 ``` rust
 println!("You guessed: {}", guess);
 ```


## Gerando um número secreto

 ``` toml
[dependencies]
rand = "0.8.5"

```
O especificado 0.8.5 é, na verdade, uma abreviação de  ^0 .8.5, o que significa qualquer versão que seja pelo menos 0.8.5, mas inferior a 0.9.0


utilizar o `cargo build` para build o projeto 


crates.io é onde as pessoas no ecossistema rust publica seus projeto rust de codigo aberto para outros usarem.

### Garantido consturções reproduzíveis com arquivos cargo.lock

é o packjson.lock do javascript, é uma forma de garantir que as depencias funcionais


### cargo update 
atualiza as versões

## Gerando um número aleatório


 ``` rust
 use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
 ```


rand::thread_rng função que no fornce o gerador de número aleatórios especifico que usaremos


1..=100 explicitar um numero 1 a 100 


## Comparando a suposição com número secreto

match é swich 

Rust é fortimente tipado, mas faz inferencias, precisamos ajusta o código para converter,utiizar u parser para inteiro, o rust tentar converte qual quer string para um inteiro de 32 bytes



 ``` rust
 let guess: u32 = guess.trim().parse().expect("Please type a number!");
 ```



### loop

igual a quaquer outro


### Tratamento de entrada inválida

pode-se alterar uma expect chamda par auma match expressão para passar da falhaem um erro para o tratamento do error

``` rust
let guess: u32 = match guess.trim().parse() { Ok(num) => num, Err(_) => continue, };
```



Este projeto foi uma forma prática de apresentar muitos novos conceitos do Rust: `let`, `match`, funções, o uso de caixas externas e muito mais. Nos próximos capítulos, você aprenderá sobre esses conceitos com mais detalhes. O Capítulo 3 aborda conceitos que a maioria das linguagens de programação possui, como variáveis, tipos de dados e funções, e mostra como usá-los em Rust. O Capítulo 4 explora a propriedade, um recurso que torna o Rust diferente de outras linguagens. O Capítulo 5 discute estruturas e sintaxe de método, e o Capítulo 6 explica como funcionam as enumerações.