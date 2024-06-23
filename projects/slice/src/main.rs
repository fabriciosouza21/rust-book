/*
Fatias permitem fazer referência a uma sequênia contínua de elemntos em uma coleção.

*/

use std::slice;

fn main() {
    let mut s = String::from("Hello world");
    let word = firt_word(&s);
    s.clear(); // Isso esvazia a String, tornando inválido o índice de word
    println!("The first word is: {}", word); // o valor de word é 5, o que não é mais válido

    // solução é usar fatias
    // uma fatia é uma referência a uma sequência de elementos em uma coleção
    // uma fatia cotem o ponteiro para o primeiro elemento e o comprimento da fatia
    // fatias contem a um ponteiro para a posição inicial e um comprimento
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];


    // podemos usar uma sintaxe mais concisa
    let s = String::from("Hello world");
    let len = s.len();
    let hello = &s[..5];
    let world = &s[6..len];
    let world = &s[6..]; // se omitirmos o segundo número, a fatia irá até o final da String
    let hello_world = &s[..]; // se omitirmos ambos os números, a fatia será a referência a toda a String



    // Usando fatias como retorno de funções
    let s = String::from("Hello world");
    let word = first_word_slice(&s);
    
    // não funcionar por que clear() gerar uma referência mutavel para a String e word é uma referência imutável
    // utilizando os conceitos da aulas anteriores, isso seria um problema de empréstimo
    //s.clear(); // error!

    println!("the first word is: {word}");


    // Literal de string como fatias 
    // O tipo &str é uma fatia de String
    // &str é uma referência imutável 
    let s = "Hello world";


    // fatia de string como parâmetros
    let my_string = String::from("Hello world");

    // first_word_slice funcionar em slice de String parcial ou total
    let word = first_word_slice(&my_string[0..6]);
    let word = first_word_slice(&my_string[..]);

    // first_word_slice também funcionar em referência de String, que é equivalent de String slice total
    let word = first_word_slice(&my_string);

    let my_string_literal = "Hello world";

    // first_word_slice funcionar em slice de string literal parcial ou total
    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);

    // por que string literal é uma fatia de String, first_word_slice também funcionar em referência de string literal
    // funcionar sem utilizar a sintaxe de slice
    let word = second_word_slice(my_string_literal);


    /* OUTRAS FATIAS */
    // fatias de arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    // resumo 
    /*
    Os conceitos de de propriedade, empréstimo e fatis garantem a seguranaça da memória em programas Rust em tempo de compilação
     */



}


fn firt_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    return s.len();
}

// podemos usar o &str para aceitar tanto String quanto &str
// &str é uma fatia de String
// e String é uma coleção de bytes

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    return &s[..];
}

// podemos usar o &str para aceitar tanto String quanto &str
// &str é uma fatia de String
// e String é uma coleção de bytes
fn second_word_slice(s:  &str) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            if start != 0 {
                end = i;
                return &s[start..end];
            }
            start = i + 1;
        }
    }
    return &s[..];
}