
``` rust
let config_max = Some(3u8)
match config_max {
	some(max) => println! ("the maximun is configured to be {max}"),
	_=> (),
}
```

Em vez disso, poderiamo escreve iss de uma forma mais curta usando if let 

``` rust
	let config_mas = some(3u8);
	if let Some(max) = config_max {
		println! ("the maximum is configured to be {max}");
	}
```

Em outras palavras, você pode pensar nisso `if let`como uma sintaxe simples para um `match`programa que executa código quando o valor corresponde a um padrão e então ignora todos os outros valores.


Podemos incluir um `else`com um `if let`. O bloco de código que vai com o `else`é o mesmo que o bloco de código que iria com o `_`case na `match`expressão que é equivalente ao `if let`and `else`.

 ``` rust
	let mut count = 0;
	if let Coin::Quarter(state) = coin {
		println ("State quarter from {state:?}!")
	}else {
		count +=1;
	}

 ```

Agora cobrimos como usar enums para criar tipos personalizados que podem ser um de um conjunto de valores enumerados. Mostramos como o `Option<T>` tipo da biblioteca padrão ajuda você a usar o sistema de tipos para evitar erros. Quando valores enum têm dados dentro deles, você pode usar `match`ou `if let`para extrair e usar esses valores, dependendo de quantos casos você precisa manipular.

Seus programas Rust agora podem expressar conceitos em seu domínio usando structs e enums. Criar tipos personalizados para usar em sua API garante a segurança de tipo: o compilador garantirá que suas funções obtenham apenas valores do tipo que cada função espera.

Para fornecer aos seus usuários uma API bem organizada, que seja fácil de usar e exponha apenas o que eles precisam, vamos agora aos módulos do Rust.
