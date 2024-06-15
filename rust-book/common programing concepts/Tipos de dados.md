
15-06-24

rust é uma linguagem de tipagem estática

## conversão 

### tipos escalares

- inteiros
- pontos flutuates
- booleanos 
- caracteres

### Tipos inteiros

![[Pasted image 20240615122645.png]]

i8 pode armazena valores entre -128 a 127 
u8 pode armazena entre 0 a 255

uma forma simples de pensar é que os valores não assinados sometem aceitam valores possitivos e valores assinados permitem valores possitivos e negativos


além disso, os vlaores isize e usize dependem da arquitetura do computador 

### literais inteiros

Literais numéricos também podem ser usados `_`​​como separadores visuais para facilitar a leitura do número, como `1_000`, que terá o mesmo valor como se você tivesse especificado `1000`.



![[Pasted image 20240615123449.png]]

## over flow 

Digamos que você tenha uma variável do tipo `u8`que pode conter valores entre 0 e 255. Se você tentar alterar a variável para um valor fora desse intervalo, como 256, ocorrerá _um estouro de número inteiro_ , o que pode resultar em um de dois comportamentos. Quando você está compilando no modo de depuração, Rust inclui verificações de estouro de número inteiro que fazem com que seu programa entre em _pânico_ em tempo de execução se esse comportamento ocorrer. Rust usa o termo _pânico_ quando um programa é encerrado com um erro; discutiremos os pânicos com mais detalhes na seção [“Erros irrecuperáveis ​​com `panic!`”](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html) no Capítulo 9.

Quando você está compilando no modo de liberação com o `--release`sinalizador, Rust _não_ inclui verificações de estouro de número inteiro que causa pânico. Em vez disso, se ocorrer overflow, Rust executa _o empacotamento de complemento de dois_ . Resumindo, valores maiores que o valor máximo que o tipo pode conter “envolvem” o mínimo dos valores que o tipo pode conter. No caso de a `u8`, o valor 256 torna-se 0, o valor 257 torna-se 1 e assim por diante.

Para lidar explicitamente com a possibilidade de overflow, você pode usar estas famílias de métodos fornecidas pela biblioteca padrão para tipos numéricos primitivos:

- Envolva todos os modos com os `wrapping_*`métodos, como `wrapping_add`.
- Retorne o `None`valor se houver overflow nos `checked_*`métodos.
- Retorna o valor e um booleano indicando se houve overflow nos `overflowing_*`métodos.
- Sature nos valores mínimo ou máximo do valor com os `saturating_*` métodos.

## Tipos de ponto flutuante

 ``` rust
 fn main() { 
	 let x = 2.0; // f64 
	 let y: f32 = 3.0; // f32 
 }
  ```

## Operações numéricas

A divisão inteira trunca em direção a zero para o número inteiro mais próximo

 ``` rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
 
 ```

### Tipo booleano
como qual quer outra linguagem

### char

 ``` rust 

fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
 
 ```

char literais com aspas simples, O tipo de Rust `char`tem quatro bytes de tamanho e representa um valor escalar Unicode.


## compund types

- tuplas 
- arrays

### Tipo tupla

agrupa valores, as tuplas tem comprimentos fixos 


 ``` rust
 fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
 ``` 

a variável tup se liga á tupla é considerada um único elemento composto, para obter o valor podemos usar a desestruturação

 ``` rust
 fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
 ``` 


acesso direto 

 ```rust 
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
} 
```


### Tipo array

tem tamanho fixo e vetor tem tamanho variavel

 ``` rust
 fn main() {
    let a = [1, 2, 3, 4, 5];
}
  ```


é possível determina o tipo do array

 ``` rust
 let a: [i32; 5] = [1, 2, 3, 4, 5];
 ```

é possível iniciarlizando um array com o mesmo valor especificando a quantidade de elementos e o valor inicial

``` rust
let a = [3; 5];

```


### Acessando elemento do array


 ``` rust 
 fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
  ```
