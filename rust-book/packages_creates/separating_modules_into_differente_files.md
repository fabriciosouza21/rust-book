# Separando módulos em arquivo diferentes

Primeiro, extrairemos o `front_of_house` módulo para seu próprio arquivo. Remova o código dentro das chaves para o front_of_housemódulo, deixando apenas a mod front_of_house;declaração, para que src/lib.rs . Observe que isso não será compilado até que criemos o arquivo src/front_of_house.rs

Nome do arquivo: src/lib.rs

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Em seguida, crie um novo arquivo chamado src/front_of_house.rs e adicione o seguinte código:

Nome do arquivo: src/front_of_house.rs

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```


## podemos extrari o `hosting` módulo para seu próprio arquivo também. Crie um novo arquivo chamado src/front_of_house/hosting.rs e adicione o seguinte código:

Nome do arquivo: src/front_of_house/hosting.rs

```rust
pub fn add_to_waitlist() {}
```

o arquivo src/front_of_house.rs

```rust
pub mod hosting;
```

o arquivo src/lib.rs não precisarar alterar nada em sua importação

Separando módulos em arquivos diferentes





## Resumo

Rust permite que você divida um pacote em várias caixas e uma caixa em módulos para que você possa se referir a itens definidos em um módulo de outro módulo. Você pode fazer isso especificando caminhos absolutos ou relativos. Esses caminhos podem ser trazidos para o escopo com uma usedeclaração para que você possa usar um caminho mais curto para vários usos do item naquele escopo. O código do módulo é privado por padrão, mas você pode tornar as definições públicas adicionando a pubpalavra-chave.

No próximo capítulo, veremos algumas estruturas de dados de coleção na biblioteca padrão que você pode usar em seu código bem organizado.
