A propriedade é o recurso mais exclusivo do Rust e tem implicações profundas para o restante da linguagem.Ele permite que o Rust ofereça garantias de segurança de memória sem a necessidade de um coletor de lixo, por isso é importante entender como funciona a propriedade.

> [!Pilha e heap]
> Propriedade é um conjunto de regras que regem como um programa rust gerencia a memória. Em rust a memória é gerenciada por meio de um sistema de propriedade com um conjunto de regras que compilador verifica.
> 
> Pilha e reap são partes da memória disponíves para seu código usar me tempo de execução.
> 
> a pilhar armazena valores na ordem em que os obtém e remove os valores na ordem opostar, last in, first out(FIFO). Dados com tamanho deconhecido em tempo de compoilação ou com tamanho que pode mudar devem ser armazenados no heap
> 
> O Heap é menos organizado: quando você coloca dado no heap, você soliciata uma certa quantidade espaço. O alocado de memória encontra m espaço vazio no heap que seja grande o suficiente, marca-o como em uso e retorna um ponteiro.como  o ponteiro para o heap tem um tamanho fico e conhecido voce pode armazena os ponteiro na pilha, mas quando quiser os dados reais, devera seguir o ponteiro
> 
> 
> ![[Pasted image 20240622122723.png]]
> 
> 
> Quando seu código chamr uma funão, os valores passados para a funão e as variáveis locias da funão são alcados na pilha. quando a funão temrina, esses valores são retirados da pilha.
> 
> O objetivo principal da propriedade é gerncia dados de heap pode ajudar a explicar pro que funciona dessa maneira 


# Regras de propriedade
- cada valor em rust tem um proprietário
- só pode haver um proprietário por vez
- Quando o proprietário sai do escopo, o valro será eleminado.

``` rust 
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

```

Existem dois pontos importante no tempo:
- quando se entra no escopo, é valido
- permanece válido até sair do escopo


## String 

 ``` java
   

// string type

// String literal is immutable

// String type is mutable and can be changed

let mut s = String::from("hello");

  

s.push_str(", world!"); // push_str() appends a literal to a String

  

println!("{s}"); // This will print `hello, world!`
  ``` 

String literais são imutaveis, O tipo string pode ser alterado, a principal diferença entre os dois é  como eles lidam com a memoria.


## Memória e alocação

No caso de uma string literal, conhecemos o conteúdo em tempo de compilação, portanto o texto é codificado diretemente no executável final, 

Com o tipo String, para suporta um trecho de texto mutável e expansível, precisamos alocar uma quantidade de memória no heap, desconhecida em tempo de compilação:
- A memória deve ser solicitada ao alocardor de memória em tempo de execução
- precisamos de uma maneira de retorna essa memória ao alocador quando terminamos nosso arquivo String

Rust segue um caminho diferente  : a memória é retornada automaticamente quando a variável que a possui sai do escopo. 

``` rust
{ let s = String::from("hello"); // s is valid from this point forward // do stuff with s }
```

Existe um ponto natural em que podemos devolver a mermória que String precisamos ao alocador: quando s sai do escopo. Quando uma variável sai do escopo, rust cham uma função especial chamada drop.


## Variáveis e dados interagindo com o Move



``` rust

    let s1 = String::from("hello");
    let s2 = s1;


```
![[Pasted image 20240622125553.png]]



quando temos duas variáveis que utilizam o mesmo espaço no heap, mas estão em escopo diferente, pode acontece de um scopo termina mas o valor do heap ainda ser usado por outro espaço de memoria, quando duas variáveis tentam liberar o mesmo espaço de memoria chamamos de duplo livre (double free)

para garantir a segurança da memória, após a linha let s2 = s1; urst considera s1 como invalido. 


Como rust invalida a primeira variável, emv vez de ser chamada de cópia superficial, ela é conhecida como **move** 

![[Pasted image 20240622130312.png]]


há uma escolha de design implcita nisso: rust nunca criará autonomamente cópias profunda


#### [Dados somente de pilha: copiar](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)

Rust tem uma anotação especial chamada `Copy`trait que podemos colocar em tipos armazenados na pilha, assim como os inteiros (falaremos mais sobre traits no [Capítulo 10](https://doc.rust-lang.org/book/ch10-02-traits.html) ). Se um tipo implementa a `Copy` característica, as variáveis ​​que a utilizam não se movem, mas são copiadas trivialmente, tornando-as ainda válidas após a atribuição a outra variável.


### [Propriedade e Funções](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)

``` rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

Se tentássemos usar `s`após a chamada para `takes_ownership`, Rust geraria um erro em tempo de compilação. Essas verificações estáticas nos protegem de erros. Tente adicionar código a `main`esses usos `s`e `x`ver onde você pode usá-los e onde as regras de propriedade impedem que você faça isso.


## [Valores de retorno e escopo](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope)

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
``` 

A propriedade de uma variável sempre segue o mesmo padrão: atribuir um valor a outra variável a move. Quando uma variável que inclui dados no heap sai do escopo, o valor será limpo, `drop`a menos que a propriedade dos dados tenha sido movida para outra variável.

Embora isso funcione, assumir a propriedade e depois devolvê-la a cada função é um pouco entediante. E se quisermos permitir que uma função use um valor, mas não assuma a propriedade? É muito chato que tudo o que passamos também precise ser devolvido se quisermos usá-lo novamente, além de quaisquer dados resultantes do corpo da função que possamos querer retornar também.

Rust nos permite retornar vários valores usando uma tupla

``` rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```