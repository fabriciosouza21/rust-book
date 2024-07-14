14-07-24

match permiter que você compare um valor com uma série de padrões(switch).


 ``` rust
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

 ```


Coin::Penny é usado como o case em switch convencionais

Quando a match expressão é executada, ela compara o valor resultante com o padrão de cada braço em ordem.

podemos colocar chaves quando precisamos executar um bloco

 ```rust 
fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println("Lucky penny!")
		}
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarte => 25,	
	}		
} 
```

## padrões que se vinculam a valores

podemo usar informações de valores ligados a enum

``` rust  
#[derive(Degug)]
enum UsState {
	labama,
	alaska,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarte(usState),
}

fn value_in_cents(coin: Coin)-> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel=> 5,
		Coin::Dime => 10,
		Coin::Quarte(state)=> {
			println!("State quarter from {state:?}!");
			25
		}
	}
}
```


## Combinando com Option

``` rust
fn plus_one (x: Option<i32) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i +1)
	}
}

fn main () {
	let five = Some(5);
	let sex = plus_one(five);
	let none = plus_one(none);
}

```

Combina match e enums é util em muitas situações; Você verá esse padrão muito no código rust: match contra um enum, vincule uma variável aos dados dentro dele e, em seguida, execute o código com base nele.

## correspondência são exaustivas

Versão com bug

``` rust

fn plus_one(x: Option<i32) -> Option<i32> {
	match x {
		Some(i) => Some(i + 1),
	}
}
```

não lidamos com None caso, então esse códig causará um bug. as correspondencia em rust são exaustiva precisamos esgota todas as últimas possibilidade para que o código seja válido.

## Padrões de captura geral e _placeholder

``` rust
	let dice_roll = 9;
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other => move_player(other),
	}
	fn add_fancy_hat() {}
	fn remove _fancy_hat() {}
	fn move_player(enum_spaces: u8) {}
```

Observe que temos que colocar o braço catch-all por último por que os padrões são avaliado em ordem. o catch all  sempre deve está no final `other => move_player(other)`

``` rust 
	let dice_roll = 9;
	match dice_roll {
	3 => add_fancy_hat(),
	7 => remove_fancy_hat(),
	_ => reroll(),_
	}

	fn add_fancy_hat(){}
	fn remove_fancy_hat() {}
	fn reroll(){}
```

se não precisamos utilizar o valor podemos usar a sintaxe `_ => reroll()`

podemos querer não fazer nada 

``` rust 
	let dice_roll = 9;
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => reroll(),
	}

	fn add_fancy_hat(){}
	fn remove_fancy_hat() {}
	fn reroll(){}
```

