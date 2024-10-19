## Traits

Podem usar traits para definir comportamentos compartilhados de forma abastrata.

<!-- notas -->

> Nota: Traços são semelhantes a um recurso frequentemente chamado de interfaces em outras linguagens, embora com algumas diferenças.

```rust

pub trait Summary {
    fn summarize(&self) -> String;
}

```

## Implementando um Trait em um tipo

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

```

Implementar um trait em um tipo é similar a implementar métodos regulares. A diferença é que depois de `impl` , colocamos o nome do trait que queremos implementar, então usamos a `for`.


Agora que a biblioteca implementou o Summary trait em NewsArticlee Tweet, os usuários do crate podem chamar os métodos do trait em instâncias de NewsArticlee Tweetda mesma forma que chamamos métodos regulares. A única diferença é que o usuário deve trazer o trait para o escopo, assim como os tipos. Aqui está um exemplo de como um crate binário poderia usar nossa aggregator biblioteca crate:

```rust

use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

```


## Implementações Padrão

Podemos fornecer uma implementação padrão para qualquer método em um trait. Isso significa que podemos fornecer alguma funcionalidade padrão e ainda permitir que tipos implementem o trait de maneira personalizada.


Para usar uma implementação padrão para resumir instâncias de NewsArticle, especificamos um impl bloco vazio com impl Summary for NewsArticle {}.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

}
```

Implementações padrão podem chamar outros métodos no mesmo trait, mesmo que esses outros métodos não tenham uma implementação padrão.

```rust

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


```


## Traits como parâmetros

Podemos usar traits como parâmetros de função para definir funções que aceitam qualquer tipo que implemente um trait específico.

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {}


```

Também podemos especificar mais de de um trait com `+`.

```rust

pub fn notify(item: &(impl Summary + Display)) {}

```

### limites de trait com where

Rust tem uma sintaxe alternativa para especificar limites de trait dentro de uma where cláusula após a assinatura da função. Então, em vez de escrever isso:

```rust

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// podemos escrever isso:
// where torna a assinatura da função mais limpa
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ }

```

## Retornando tipos que implementam Traits
Ao usar impl Summarypara o tipo de retorno, especificamos que a returns_summarizablefunção retorna algum tipo que implementa o Summary trait sem nomear o tipo concreto. Neste caso, returns_summarizable retorna um Tweet, mas o código que chama esta função não precisa saber disso. No entanto, você só pode usar impl Traitse estiver retornando um único tipo.
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

```


Isso não funciona

``` rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

## Using Trait Bounds to Conditionally Implement Methods

Ao usar um trait vinculado a um impl bloco que usa parâmetros de tipo genérico, podemos implementar métodos condicionalmente para tipos que implementam os traits especificados

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

```

