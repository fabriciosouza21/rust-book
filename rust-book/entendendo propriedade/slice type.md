Fatias permitem fazer referência a uma sequênia contínua de elementos em uma coleção.

 ``` rust 
     let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

 ```

Portanto, no caso de let world = &s[6..11];, worldseria uma fatia que contém um ponteiro para o byte no índice 6 scom um valor de comprimento de 5.

![[Pasted image 20240623103358.png]]

