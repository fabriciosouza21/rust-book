
podemos explicitar que queremos usar uma referencia, quando utilizamos a referencia o proprietario do espaço de memoria e o valor que chamou a função então o valor não desalocado quando o escopo e finalizado 

``` rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}



```


assim como variáveis referencias também são

## Referencias mutáveis

 ``` rust 

 fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
 ```

não pode existir duas referencias mutáveis para o mesmo  valor, o código baixo gerar um error

 ``` rust  
 let mut s = String::from("hello"); let r1 = &mut s; let r2 = &mut s; println!("{}, {}", r1, r2);
 ```

rust tentar evitar a corrida por dados, quando trabalhamos com referencias mutáveis, os três principais aspectos são:

- dois ou mais ponteiros acessam os mesmo dados ao mesmo tempo.
- pelo meno um dos ponteiro está sendo usado para gravar nos dados.
- não ha nenhum mecanismo sendo usado para sincronizar o acesso aos dados

podemos criar um novo escopo para ter múltiplas referencias mutáveis, mas ela não vao ser simulantes 

 ``` rust
 let mut s = String::from("hello"); { let r1 = &mut s; } // r1 goes out of scope here, so we can make a new reference with no problems. let r2 = &mut s;
  ```

Rust impõe regras semelhantes para combinar referências mutáveis e imutáveis, o codigo abaixo resulta em error, não possivel ter uma referencia mutável, enquanto tivermos uma imutável com o mesmo valor

``` rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

```

O escopo de referencia começa onde ela é introduzida e continua até última vez que a referência é usada.

``` rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

```


## Referência pendentes

Em Rust o compilador garante que as referências nunca serão referências pendentes, o compilador garantira que os dados não sairão do escopo antes de referncia os dados

``` rust   

fn main() { let reference_to_nothing = dangle(); }

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

# revisão

- A qualquer momento, você pode ter uma referência mutavel ou qualquer numero de referencia imutáveis 
- as referências devem ser sempre validas