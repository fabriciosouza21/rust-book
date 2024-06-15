O código rust usa Snake Case como estilo convencional para nomes de funções e variáveis


podemos definir funões em qual quer parte do arquivo rust

### declarações e expressões 

- **declarações** são instruções que executam alguma ação e não retornam um valor.
- **As expressões** são avaliadas como um valor resultante.

 ``` rust

fn main() {
    let y = {
        let x = 3;
        x + 1 // esse valor é somado 3 + 1
    };

    println!("The value of y is: {y}"); // 4
}
 
  ```

### Funções com valores de retorno

os retornos devem ter seu tipos declarados como uma seta (->), em Rust, o valor de retorno da função é sinônimo do valor da expressão final no bloco do corpo de uma função

 ``` rust

 fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
 ```
