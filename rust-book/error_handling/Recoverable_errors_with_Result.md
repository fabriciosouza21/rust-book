
## Recoverable erros with Result

A maioria dos erros não são o suficiente para exigir que o programe pare completamente.


 ``` rust
 enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

O `Result` é um tipo enumerado que é definido pela biblioteca padrão. Ele tem dois valores possíveis: `Ok`, que indica que a operação foi bem-sucedida e contém um valor do tipo `T`, e `Err`, que indica que houve um erro e contém um valor do tipo `E`.


```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

O tipo de retorno de  `File::open` é a  `Result<T, E> `. O parâmetro genérico T foi preenchido pela implementação de  `File::open` com o tipo do valor de sucesso,  `std::fs::File`, que é um identificador de arquivo. O tipo de E usado no valor de erro é  `std::io::Error`


```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

O `match` é uma expressão que permite que você compare um valor com uma série de padrões e, dependendo do padrão, execute o código associado. O `match` é exaustivo, o que significa que você deve cobrir todos os casos possíveis. Se o compilador não puder garantir que você cobriu todos os casos, ele emitirá um erro.

## Matching on Different Errors


``` rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

A condição que queremos verificar na correspondência interna é se o valor retornado por  `error.kind()` é a `NotFound` variante do `ErrorKind` enum. Se for, tentamos criar o arquivo com `File::create`. No entanto, como `File::create` também pode falhar, precisamos de um segundo braço na `match` expressão interna. Quando o arquivo não pode ser criado, uma mensagem de erro diferente é impressa. O segundo braço do externo `match` permanece o mesmo, então o programa entra em pânico em qualquer erro além do erro de arquivo ausente.



### Alternatives to Using match with Result<T, E>

```rust

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

Embora esse código tenha o mesmo comportamento da Listagem 9-5, ele não contém nenhuma `match` expressão e é mais limpo para ler.


## Shortcuts for Panic on Error: unwrap and expect

```rust

use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

O método `unwrap` é uma função de conveniência fornecida pela biblioteca padrão que é implementada para `Result`. Se o `Result` for `Ok`, `unwrap` retornará o valor dentro de `Ok`. Se o `Result` for `Err`, `unwrap` chamará a macro `panic!` para nós. Se chamarmos `unwrap` em um `Err`, o `panic!` mensagem padrão incluirá a mensagem de erro que foi passada para `Err`. Isso é útil para mensagens de erro que são úteis para os desenvolvedores que estão depurando o código, mas não são úteis para os usuários finais do código.

```rust

use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

O método `expect` é semelhante a `unwrap`, mas permite que você forneça uma mensagem de erro personalizada que será impressa se o `Result` for `Err`.

```bash
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```


## Propagating Errors

```rust

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
		// retorno antecipado
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username), //returno sucesso
        Err(e) => Err(e), // retorno erro
    }
}
```


## A Shortcut for Propagating Errors: the ? Operator

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")? // ? retorna o error antecipadamente;
    let mut username = String::new();
    username_file.read_to_string(&mut username)? // ? retorna o error antecipadamente;
    Ok(username)
}
```

O operador `?` pode ser usado em funções que retornam `Result`, simplificando a manipulação de erros. Se o valor de retorno for `Ok`, o valor dentro de `Ok` será retornado da expressão e o programa continuará. Se o valor de retorno for `Err`, `Err` será retornado da função inteira, assim como se tivéssemos usado a palavra-chave `return` e o valor de erro. O `?` operador pode ser usado em funções que retornam `Result`, simplificando a manipulação de erros. Se o valor de retorno for `Ok`, o valor dentro de `Ok` será retornado da expressão e o programa continuará. Se o valor de retorno for `Err`, `Err` será retornado da função inteira, assim como se tivéssemos usado a palavra-chave `return` e o valor de erro.


``` rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

   // utilizando operador ? de forma encadeada
   // em qual quer erro a função retornará o erro
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```


```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

O código acima é equivalente ao código anterior, mas é mais conciso e mais legível. A função `fs::read_to_string` abre o arquivo, lê seu conteúdo para uma `String` e retorna o `Result`. Se o arquivo não puder ser aberto ou lido, o `Result` será `Err` e o erro será retornado para o chamador.

## Where The ? Operator Can Be Used

``` rust

use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

O código acima não compilará porque o `?` operador só pode ser usado em funções que retornam `Result` ou `Option` ou em expressões que retornam `Result` ou `Option`. Para resolver esse problema, podemos adicionar um `main` função que retorna `Result` ou `Option`. o File::open retorna um Result, mas o main não retorna um Result, então não podemos usar o operador `?` diretamente.


```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

O código acima é um exemplo de como o `?` operador pode ser usado em funções que retornam `Option`. O método `lines` retorna um iterador que produz uma linha por vez. O método `next` retorna o próximo elemento do iterador, que neste caso é a primeira linha. Se `text` for uma `String` vazia, `lines` retornará um iterador vazio, então `next` retornará `None`. Se `text` não for uma `String` vazia, `next` retornará `Some`, e o `?` operador continuará. O método `chars` retorna um iterador que produz um caractere por vez. O método `last` retorna o último elemento do iterador, que neste caso é o último caractere da primeira linha. Se a primeira linha for vazia, `chars` retornará um iterador vazio, então `last` retornará `None`. Se a primeira linha não for vazia, `last` retornará `Some`, e o `?` operador continuará. Se `text` for uma `String` vazia ou se a primeira linha for vazia, a função retornará `None`. Se `text` não for uma `String` vazia e a primeira linha não for vazia, a função retornará o último caractere da primeira linha.

A `main` pode retornar `Result<(), E>`

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

Por enquanto, você pode ler como “qualquer tipo de erro”. Usar `?` um Result valor em uma mainfunção com o tipo de erro `Box<dyn Error>` é permitido porque permite que qualquer Errvalor seja retornado antecipadamente.

Quando uma mainfunção retorna um Result<(), E>, o executável sairá com um valor de 0if mainretorna Ok(())e sairá com um valor diferente de zero if mainretorna um Errvalor. Executáveis ​​escritos em C retornam inteiros quando saem: programas que saem com sucesso retornam o inteiro 0, e programas que retornam erroneamente algum inteiro diferente de 0. Rust também retorna inteiros de executáveis ​​para ser compatível com essa convenção.



