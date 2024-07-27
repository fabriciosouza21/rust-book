# Armazenado listas de valores com vetores

Vetores podem armazena paenas valores do mesmo tipo.

## Criando um novo vetor

``` rust

	let v: vec<i32> = Vec::new();

```

Quando estamos iniciando um vertor sem valores, é necessário informar o tipo de dado que será armazenado.

podemos também inicializar valores no vertor, para isso utilizamos a macro vec!

``` rust

	let v = vec![1,2,3];

```
rust infere o tipo de dado que será armazenado no vetor.

## Atualizando um vetor

``` rust

	let mut v = Vec::new();

	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);

```

para tornar o vetor mutável, é necessário declarar a variável como mutável. além disso não precisamos informa o tipo do vetor, os numero que são adicionados são do tipo i32


## leitura de vetores

ha duas maneira de referenciar um valor armazenado em um vetor, a primeira é utilizando o operador [] e a segunda é utilizando o método get.

``` rust

	let v = vec![1,2,3,4,5];

	let third: &i32 = &v[2];

	println!("O terceiro elemento é {}", third);

	let third: Option<&i32> = v.get(2);

	match third {
		some(third )=> println!("O terceiro elemento é {}", third),
		None => println!("Não existe terceiro elemento"),
	}

```

a diferença entre [] e get é que [] retorna um erro e encerra o programa caso o valor não exista, já o get retorna um Option<&T> que pode ser None ou Some(&T).


Quando o programa tem uma referência válida, o verificador de emprestimo aplica as regras de propriedade e empréstimo.

``` rust

	let mut v = vec![1,2,3,4,5];
	let first = &v[0];

	v.push(6);

	println!("O primeiro elemento é: {}", first);

	// Tentando adicionar um elemento a um vetor enquanto mantém uma referência a um item

```


// sitaxe pra nota pfv

> pode parecer que deveria funcionar: por que uma referência ao primeiro elemento deveria se  importar com mudanças no final do vetor? Esse erro é devido à maneira como os vetores funcionam: como os vetores colocam os valores próximos uns dos outros na memória, adicionar um novo elemento no final do vetor pode exigir alocar nova memória e copiar os elementos antigos para o novo espaço, se não houver espaço suficiente para colocar todos os elementos próximos uns dos outros onde o vetor está armazenado atualmente. Nesse caso, a referência ao primeiro elemento estaria apontando para a memória desalocada. As regras de empréstimo impedem que os programas acabem nessa situação.


### Iterando sobre os valores em um vetor

``` rust

	let v = vec![100, 32, 57];
	for i in &v {
		println!("{}", i);
	}

```


## Iterando sobre os valores em um vetor mutável

``` rust

	let mut v = vec![100, 32, 57];
	for i in &mut v {
		*i += 50;
	}

```

para alterar o valor ao qual a referência mutável se refere, temos que usar o * operador de desreferência para chegar ao valor em i antes de podemos usar o operador += para adicionar 50 a esse valor.

> Iterar sobre um vetor, seja imutável ou mutável, é seguro por causa das regras do verificador de empréstimo. Se tentássemos inserir ou remover itens nos for corpos do loop na Listagem 8-7 e Listagem 8-8, obteríamos um erro do compilador semelhante ao que obtivemos com o código na Listagem 8-6. A referência ao vetor que o forloop contém impede a modificação simultânea de todo o vetor.


## Usando um Enum para armazenar multiplos tipos

para armazenar multiplos tipos em um vetor, podemos utilizar um enum, pois o enum pode armazenar multiplos tipos.

``` rust

	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String)
	}

	let row = vec! [
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];

```

## Soltar um vetor Elimina seu elementos

o vetor é liberado da memória quando ele sai do escopo, assim como qualquer outra variável em Rust.

``` rust

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here


```



