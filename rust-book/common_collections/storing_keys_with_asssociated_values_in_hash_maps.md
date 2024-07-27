# Armazenando chaves com valores associados em mapas de hash

## Criando um novo mapa de hash

``` rust

	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

```

note que precisamos primeiro usar o use da hashMap parte de coleções da biblioteca padrão para poder usar HashMap.

hashMap são homogêneos, ou seja, todos os valores devem ser do mesmo tipo. no caso acima o rust infere que os valores são do tipo i32 e as chaves são do tipo String.

## Acessando valores em um mpa de hash

``` rust

	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let team_name = String::from("Blue");
	let score = scores.get(&team_name).copied().unwrap_or(0);

	println!("O time {} tem {} pontos", team_name, score);

//Acessando a pontuação do time Azul armazenada no mapa hash
```

o método get retorna um Option<&V>, o que significa que get retorna um valor que está dentro de um Some se houver um valor associado à chave e None se não houver. O método copiado retorna uma cópia do valor, então o valor original ainda está disponível para ser usado novamente. Se não houver um valor para a chave, o método copiado retorna um valor padrão de 0.

``` rust

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

```


## Mapas de hash e propriedade

Para tipos que implementam o Copytrait, como i32, os valores são copiados para o hash map. Para valores owned como String, os valores serão movidos e o hash map será o owner desses valores.



``` rust

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

```

## Atualizando um valor de hashMap

### Sobrescrevendo um valor



``` rust

	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Blue"), 25);

	println!("{:?}", scores);

```

se tentarmos inserir um valor com a mesma chave, o valor será sobrescrito.

### Inserindo um valor se a chave não existir

``` rust

	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);

	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);

	println!("{:?}", scores);

```
O or_insert método on Entry é definido para retornar uma referência mutável ao valor da Entry chave correspondente se essa chave existir, e se não existir, ele insere o parâmetro como o novo valor para essa chave e retorna uma referência mutável ao novo valor. Essa técnica é muito mais limpa do que escrever a lógica nós mesmos e, além disso, funciona melhor com o verificador de empréstimo.


### Atualizando um valor com base no valor antigo

Outro caso de uso comum para mapas hash é procurar o valor de uma chave e então atualizá-lo com base no valor antigo. Por exemplo, a Listagem 8-25 mostra o código que conta quantas vezes cada palavra aparece em algum texto. Usamos um mapa hash com as palavras como chaves e incrementamos o valor para manter o controle de quantas vezes vimos essa palavra. Se for a primeira vez que vemos uma palavra, primeiro inseriremos o valor 0.

``` rust

	use std::collections::HashMap;

	let text = "hello world wonderful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}

	println!("{:?}", map);

```

>Funções de hash
Por padrão, HashMapusa uma função de hash chamada SipHash que pode fornecer resistência a ataques de negação de serviço (DoS) envolvendo tabelas de hash 1 . Este não é o algoritmo de hash mais rápido disponível, mas a troca por melhor segurança que vem com a queda no desempenho vale a pena. Se você criar um perfil do seu código e descobrir que a função de hash padrão é muito lenta para seus propósitos, você pode alternar para outra função especificando um hasher diferente. Um hasher é um tipo que implementa o BuildHashertrait. Falaremos sobre traits e como implementá-los no Capítulo 10 . Você não precisa necessariamente implementar seu próprio hasher do zero; crates.io tem bibliotecas compartilhadas por outros usuários do Rust que fornecem hashers implementando muitos algoritmos de hash comuns. https://en.wikipedia.org/wiki/SipHash



## Resumo

Vetores, strings e mapas hash fornecerão uma grande quantidade de funcionalidade necessária em programas quando você precisar armazenar, acessar e modificar dados. Aqui estão alguns exercícios que você agora deve estar equipado para resolver:

1. Dada uma lista de inteiros, use um vetor e retorne a mediana (quando classificado, o valor na posição intermediária) e a moda (o valor que ocorre com mais frequência; um mapa hash será útil aqui) da lista.

2. Converta strings para pig latin. A primeira consoante de cada palavra é movida para o final da palavra e ay é adicionado, então first se torna irst-fay . Palavras que começam com uma vogal têm hay adicionado ao final em vez disso ( apple se torna apple-hay ). Tenha em mente os detalhes sobre a codificação UTF-8!

3. Usando um mapa hash e vetores, crie uma interface de texto para permitir que um usuário adicione nomes de funcionários a um departamento em uma empresa; por exemplo, “Adicionar Sally à Engenharia” ou “Adicionar Amir às Vendas”. Em seguida, deixe o usuário recuperar uma lista de todas as pessoas em um departamento ou todas as pessoas na empresa por departamento, classificadas em ordem alfabética.
A documentação da API da biblioteca padrão descreve métodos que vetores, strings e mapas de hash têm e que serão úteis para esses exercícios!

4. Estamos entrando em programas mais complexos nos quais as operações podem falhar, então é um momento perfeito para discutir o tratamento de erros. Faremos isso em seguida!
