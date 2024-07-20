# Caminhos para referenciar um item na árvore de módulos

Um caminho pode assumir duas formas:

- Um caminho absoluto é o cmainho completo começando na raiz de uma crate, para código de uma caixa externa, o caminho absoluto começa com o nome da crate, para  código da crate atual, ele começa com o literal `crate`
- Um caminho relativo começa com `self`, `super`, ou um identificador na mesma crate.

os caminhos absolutos e relativos são seguidos por um ou mais identificadores separados por `::` para acessar um item em um módulo.


digamos que queremos chamr a `add_to_waitlist` função a partir de uma nova função `eat_at_restaurant`  definida na raiz do crate.

```rust
mod front_of_house {
	// modulo publico
    pub mod hosting {
	    // função publica
       pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

A primeira vez que chamamos a `add_to_waitlist` função em `eat_at_restaurant`, usamos um caminho absoluto. A `add_to_waitlist` função é definida no mesmo crate que `eat_at_restaurant`, o que significa que podemos usar a `crate` palavra-chave para iniciar um caminho absoluto. Então, incluímos cada um dos módulos sucessivos até chegarmos a `add_to_waitlist`. Você pode imaginar um sistema de arquivos com a mesma estrutura: especificaríamos o caminho `/front_of_house/hostinga/dd_to_waitlist` para executar o `add_to_waitlist` programa;**usar o crate nome para iniciar a partir da raiz do crate é como usar /** para iniciar a partir da raiz do sistema de arquivos no seu shell.

dd_to_waitlistNa segunda vez que chamamos `eat_at_restaurant`, usamos um caminho relativo. O caminho começa com `front_of_house`, o nome do módulo definido no mesmo nível da árvore de módulos que `eat_at_restaurant`. Aqui, o equivalente do sistema de arquivos seria usar o caminho `front_of_house/hosting/add_to_waitlist`. Começar com um nome de módulo significa que o caminho é relativo.

Escolher se usará um caminho relativo ou absoluto é uma decisão que você tomará com base no seu projeto e depende se você tem mais probabilidade de mover o código de definição de item separadamente ou junto com o código que usa o item. Por exemplo, se movermos o front_of_housemódulo e a eat_at_restaurant função para um módulo chamado customer_experience, precisaríamos atualizar o caminho absoluto para add_to_waitlist, mas o caminho relativo ainda seria válido. No entanto, se movermos a eat_at_restaurantfunção separadamente para um módulo chamado dining, o caminho absoluto para a add_to_waitlistchamada permaneceria o mesmo, mas o caminho relativo precisaria ser atualizado. Nossa preferência em geral é especificar caminhos absolutos porque é mais provável que queiramos mover definições de código e chamadas de item independentemente umas das outras.


No Rust, todos os itens (funções, métodos, structs, enums, módulos e constantes) são privados para os módulos pais por padrão. Se você quiser tornar um item como uma função ou struct privado, você o coloca em um módulo.

## Iniciando caminhos relativos com super

Você pode usar o super palavra-chave para acessar o módulo pai de um módulo.sso é como iniciar um caminho de sistema de arquivos com a ..sintaxe.

```rust

fn deliver_order(){}

mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}
	fn cook_order() {}
}
```

## Tornando Estruturas e Enums Públicas

### Structs
Também podemos usar pubpara designar structs e enums como públicos, mas há alguns detalhes extras para o uso de pubwith structs e enums. **Se usarmos pub antes de uma definição de struct, tornamos o struct público, mas os campos do struct ainda serão privados**. Podemos tornar cada campo público ou não, caso a caso.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

 note que como back_of_house::Breakfasttem um campo privado, a struct precisa fornecer uma função pública associada que constrói uma instância de Breakfast(nós a nomeamos summer aqui). Se Breakfastnão tivesse tal função, não poderíamos criar uma instância de Breakfastin eat_at_restaurant porque não poderíamos definir o valor do seasonal_fruit campo privado em eat_at_restaurant.


### Enum

Em contraste, se tornarmos um enum público, todas as suas variantes serão públicas. Precisamos apenas do pubantes da enumpalavra-chave

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```
