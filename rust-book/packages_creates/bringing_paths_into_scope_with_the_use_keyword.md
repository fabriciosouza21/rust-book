# Bringing Paths into Scope with the use Keyword

Há uma meneira de simplificar o processso de caminho relativo ou absoluto.


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

adiciona `use` um caminho em um é semelhante a criar um link simbólico no sistema de arquivos.

Note que `use` apenas cria o atalho para o escopo específico em que o `use` ocorre. a eat_at_restaurant função para um novo módulo filho chamado customer, que é então um escopo diferente da use declaração, então o corpo da função não será compilado:


error failed to resolve: use of undeclared crate or module  `hosting`


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

Observe que também há um aviso de que o usenão é mais usado em seu escopo! Para corrigir esse problema, mova o usedentro do customermódulo também, ou referencie o atalho no módulo pai com super::hostingdentro do módulo filho customer.


## Criando `use` Caminhos Idiomáticos

Trazer o módulo pai da função para o escopo com `use` significa que temos que especificar o módulo pai ao chamar a função. Especificar o módulo pai ao chamar a função deixa claro que a função não é definida localmente, ao mesmo tempo em que minimiza a repetição do caminho completo.

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

Não há uma razão forte por trás dessa expressão: é apenas uma convenção que surgiu, e as pessoas se acostumaram a ler e escrever código Rust dessa maneira.

## Fornecendo novos nomes com a as palavra-chave

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## Reexportando Nomes com `pub use`

Essa técnica é chamada de reexportação porque estamos trazendo um item para o escopo, mas também tornando esse item disponível para outros trazerem para seus escopos.

``` rust

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

```
Antes dessa mudança, o código externo teria que chamar a `add_to_waitlist` função usando o caminho `restaurant::front_of_house::hosting::add_to_waitlist()`, o que também exigiria que o `front_of_house` módulo fosse marcado como `pub`. Agora que isso pub use reexportou o hosting módulo do módulo raiz, o código externo pode usar o caminho `restaurant::hosting::add_to_waitlist()`em vez disso.

## Usando pacotes externos

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```


Note que a std biblioteca padrão também é uma caixa externa ao nosso pacote. Como a biblioteca padrão é enviada com a linguagem Rust, não precisamos alterar Cargo.toml para include std. Mas precisamos nos referir a ela com usepara trazer itens de lá para o escopo do nosso pacote. Por exemplo, com HashMap usaríamos esta linha:

```rust
use std::collections::HashMap;
```

Usando caminhos aninhados para limpar use listas grandes

```rust
use std::cmp::Ordering;
use std::io;


// --snip--
// modo simplificado
use std::{cmp::Ordering, io};
// --snip--

```

```rust
use std::io;
use std::io::Write;

// --snip--

// modo simplificado
use std::io::{self, Write};

```

A parte comum desses dois caminhos é std::io, e esse é o primeiro caminho completo. Para mesclar esses dois caminhos em uma useinstrução, podemos usar selfno caminho aninhado,

esta linha traz std::io e std::io::Write para o escopo.

## Operador o glob

```
use std::collections::*;
```

Esta usedeclaração traz todos os itens públicos definidos em std::collections para o escopo atual. Tenha cuidado ao usar o operador glob! Glob pode dificultar a identificação de quais nomes estão no escopo e onde um nome usado no seu programa foi definido.







