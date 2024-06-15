
Ao contrario de linguagem como ryby e javaScript, Rust não tentara converter automaticamente tipos não booleanos em booleanos


o if é uma expresão do rust, então podemos usa-lo do lado direito de let 

 ``` rust 
   
 fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
 ```
## Repetição com loops

  ``` rust
  fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
  ```


### Rótulos de loop para desambiguar entre vários loops


``` rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

Os rótulos de loop devem começar com aspas simples 

O loop externo tem o rótulo `'counting_up`e contará de 0 a 2. O loop interno sem rótulo conta regressivamente de 10 a 9. O primeiro `break`que não especificar um rótulo sairá apenas do loop interno. A `break 'counting_up;`instrução sairá do loop externo. 


### loops condicionais com while
 
 ``` rust
 fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
  ```


### Percorrendo uma coleção com for

 ```rust
 fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
 ```
