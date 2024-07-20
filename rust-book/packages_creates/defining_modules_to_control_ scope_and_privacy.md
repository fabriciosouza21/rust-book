# Folha de dicas de módulos

- **Comece pela raiz do crate** : Ao compilar um crate, o compilador primeiro procura no arquivo raiz do crate (geralmente src/lib.rs para um crate de biblioteca ou src/main.rs para um crate binário) o código a ser compilado.

- **Declarando módulos** : No arquivo raiz crate, você pode declarar novos módulos; digamos, você declara um módulo “garden” com `mod garden`;. O compilador procurará o código do módulo nestes lugares:
	- Em linha, dentro de chaves que substituem o ponto e vírgula a seguir `mod garden`
    - No arquivo src/garden.rs
	- No arquivo src/garden/mod.rs

- **Declarando submódulos** : Em qualquer arquivo que não seja o crate root, você pode declarar submódulos. Por exemplo, você pode declarar `mod vegetables`;em src/garden.rs . O compilador procurará o código do submódulo dentro do diretório nomeado para o módulo pai nestes lugares:
  - Em linha, seguindo diretamente `mod vegetables`, entre chaves em vez do ponto e vírgula
  - No arquivo src/garden/vegetables.rs
  - No arquivo src/garden/vegetables/mod.rs

- **Caminhos para o código** : Uma vez que um módulo é parte do seu crate, você pode se referir ao código naquele módulo de qualquer outro lugar naquele mesmo crate, desde que as regras de privacidade permitam, usando o caminho para o código. Por exemplo, um Asparagustipo no módulo garden vegetables seria encontrado em `crate::garden::vegetables::Asparagus`.
- **Privado vs público** : O código dentro de um módulo é privado de seus módulos pais por padrão. Para tornar um módulo público, declare-o com `pub mod` em vez de `mod`. Para tornar os itens dentro de um módulo público públicos também, use `pub` antes de suas declarações.


**A use palavra-chave** : Dentro de um escopo, a `use` palavra-chave cria atalhos para itens para reduzir a repetição de caminhos longos. Em qualquer escopo que possa se referir a `crate::garden::vegetables::Asparagus`, você pode criar um atalho com use `crate::garden::vegetables::Asparagus`;e, a partir daí, você só precisa escrever `Asparaguspara` fazer uso desse tipo no escopo.

## exemplo backyard

``` uml
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

## src/main.rs

```rust

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

A `pub mod garden`informa ao compilador para incluir o código encontrado em src/garden.rs

Nome do arquivo: src/garden.rs

```rust
pub mod vegetables;
```

A `pub mod vegetables`informa ao compilador para incluir o código encontrado em src/garden/vegetables.rs

Nome do arquivo: src/garden/vegetables.rs

```rust
#[derive(Debug)]
pub struct Asparagus;
```

## Agrupando código relacionado em módulos

## primeiro passo

### criar uma biblioteca chamada restaurant

`cargo new restaurant --lib`

inserirno src/lib.rs

```rust

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

definimos um módulo com a declaração `mod` e, em seguida, definimos outros módulos dentro dele. O código dentro dos módulos é privado por padrão, então precisamos tornar os itens públicos com a palavra-chave `pub` se quisermos acessá-los de fora do módulo.

``` uml
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

```

Esta árvore mostra como alguns dos módulos aninham-se uns dentro dos outros; por exemplo, `hosting` aninha-se dentro de `front_of_house`. A árvore também mostra que alguns módulos são irmãos uns dos outros, o que significa que são definidos no mesmo módulo; `hostinge` `serving` são irmãos definidos dentro de `front_of_house`. Se o módulo A estiver contido dentro do módulo B, dizemos que o módulo A é filho do módulo B e que o módulo B é pai do módulo A. Observe que toda a árvore de módulos está enraizada sob o módulo implícito denominado `crate`.

A árvore de módulos pode lembrá-lo da árvore de diretórios do sistema de arquivos no seu computador; esta é uma comparação muito adequada! Assim como diretórios em um sistema de arquivos, você usa módulos para organizar seu código. E assim como arquivos em um diretório, precisamos de uma maneira de encontrar nossos módulos.
