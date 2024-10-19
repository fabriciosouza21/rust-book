## Tipos de dados genéricos

Usamos genéricos para criar definições para itens como asinatura de função ou structs, que podemos usar com muitos tipos de dados concretos diferentes.

## Em definições de funções


```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}
```


```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
```


## Em definições de structs

```rust
struct Point<T> {
	x: T,
	y: T,
}

fn main() {
	let integer = Point { x: 5, y: 10 };
	let float = Point { x: 1.0, y: 4.0 };
}
```

Note que, como usamos apenas um tipo genérico para definir `Point<T>`, essa definição diz que a `Point<T>` struct é genérica sobre algum tipo T, e os campos x e y são ambos do mesmo tipo,

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

## Em definições de enums

```rust
enum Option<T> {
    Some(T),
    None,
}

```

```rust

enum Result<T, E> {
    Ok(T),
    Err(E),
}

```

## Em métodos de structs

```rust

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

Aqui, definimos um método chamado xon Point<T>que retorna uma referência aos dados no campo x.

```rust

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

```



## Em definições de structs com múltiplos tipos genéricos

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

O propósito deste exemplo é demonstrar uma situação na qual alguns parâmetros genéricos são declarados com imple alguns são declarados com a definição do método. Aqui, os parâmetros genéricos X1 e Y1 são declarados depois imp porque eles vão com a definição da struct. Os parâmetros genéricos X2 e Y2são declarados depois fn mixup porque eles são relevantes apenas para o método.
