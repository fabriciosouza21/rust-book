# Aramazenando uma string codificada em utf-8 com strigs

Rust tem apenas um tipo de string na linguagem principal, que é o string slice str que geralmente é visto em sua forma emprestada &str

## Criando uma nova String

muitas operações disponíveis para vetores também estão disponíveis para strings,implementado como um wrapper em torno de um vetor de bytes com algumas garantias

``` rust

	let mut s = String::new();

```

``` rust

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

	/** Usando o to_stringmétodo para criar Stringa partir de uma string literal */

	/*--------------------------------------------------------------------------*/
	let s = String::from("initial contents");

	/* Usando a String::fromfunção para criar Stringa partir de uma string literal */

```


## Atualizando uma String

### acrescentado a uma string

``` rust

	let mut  s = String::from("foo");
	s.push_str("bar");

```

``` rust

	let mut s = String::from("foo");
	let s2 = "bar";
	s.push_str(s2);
	println!("s2 is {}", s2);

	//Usando uma fatia de string após anexar seu conteúdo a umString

```

Se o push_str método tomasse posse de s2, não poderíamos imprimir seu valor na última linha. No entanto, esse código funciona como esperávamos!



``` rust

	let mut s = String::from("lo");
	s.push('l');

```

## Concatenação com o operador +

``` rust

	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

	//Usando o + operador para combinar dois String valores em um novo String valor

	// + operador usa a assinatura de função fn add(self, s: &str) -> String {}

```

utilizar o operador + para concatenar duas strings, o Rust chama o add método, que o operador + é uma abreviação para. O add método assina a função fn add(self, s: &str) -> String, que consome o valor de s1, o que significa que s1 não está mais disponível após a adição.

``` rust

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = s1 + "-" + &s2 + "-" + &s3;

```

### Usando o método format!

``` rust

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = format!("{s1}-{s2}-{s3}");

```

format! funciona como println!, mas, em vez de imprimir o texto na tela, ele retorna uma string com o texto formatado. A string não é modificada, então podemos usá-la novamente.


## indexação em String

se tentarmos acessar partes de uma String usando indexação, obteremos um error.

``` rust

	let s1 = String::from("hello");
	let h = s1[0];

//Tentando usar sintaxe de indexação com uma String
```

rust  não suporta indexação de string

### Representação interna

'String' é um wrapper sobre um vec<u8>

```rust

	let len = String::from("Hola").len();
	// len is 4

```

o rust precisar de 4 bytes para armazena a string 'Hola' em utf-8

```rust

	let len = String::from("Здравствуйте").len();
	// len is 24

```

a resposta não vai ser  12 porque a string 'Здравствуйте' tem 12 letras, mas sim 24 porque cada letra é representada por 2 bytes. portanto nem sempre se correlacionara com um valor escalar Unicode válido

```rust

	let hello = "Здравствуйте";

	let s = &hello[0];

```

como não temos como saber qual caractere será retornado, o Rust não compila esse código e evita mal-entendidos no início do processo de desenvolvimento.


### Bytes e valores escalares e clusters de grafemas

Uma razão final pela qual Rust não nos permite indexar em a String para obter um caractere é que operações de indexação devem sempre levar tempo constante (O(1)). Mas não é possível garantir esse desempenho com a String, porque Rust teria que percorrer o conteúdo do início até o índice para determinar quantos caracteres válidos havia.


### String de corte

Indexar em uma string é frequentemente uma má ideia porque não está claro qual deve ser o tipo de retorno da operação de indexação de string: um valor de byte, um caractere, um cluster de grafema ou uma fatia de string. Se você realmente precisa usar índices para criar fatias de string, Rust pede que você seja mais específico.

``` rust

let hello = "Здравствуйте";

let s = &hello[0..4];

```

Aqui, s será um &strque contém os quatro primeiros bytes da string. Anteriormente, mencionamos que cada um desses caracteres tinha dois bytes, o que significa sque será Зд.

Se tentássemos dividir apenas parte dos bytes de um caractere com algo como &hello[0..1], o Rust entraria em pânico em tempo de execução da mesma forma que se um índice inválido fosse acessado em um vetor:

``` shel

$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/collections`
thread 'main' panicked at src/main.rs:4:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


```

## Método para iteração

A melhor maneira de operar em slice de string é ser explícito sobre se você que caracteres ou bytes. para valores escalares Unicode individuais, use o `chars`. usar o `chars` método retorna uma iteração sobre cada caractere Unicode em uma string.

``` rust

	let s = "Зд";
	for c in s.chars() {
		println!("{}", c);
	}

	//З
	//д
```

``` rust

	let s = "Зд";
	for b in s.bytes() {
		println!("{}", b);
	}

	//208
	//151
	//208
	//180
```

Para resumir, strings são complicadas. Diferentes linguagens de programação fazem escolhas diferentes sobre como apresentar essa complexidade ao programador. Rust escolheu tornar o tratamento correto de String dados o comportamento padrão para todos os programas Rust, o que significa que os programadores têm que pensar mais no tratamento de dados UTF-8 antecipadamente. Essa troca expõe mais da complexidade das strings do que é aparente em outras linguagens de programação, mas evita que você tenha que lidar com erros envolvendo caracteres não ASCII mais tarde em seu ciclo de vida de desenvolvimento.

A boa notícia é que a biblioteca padrão oferece muitas funcionalidades construídas a partir dos tipos Stringe &strpara ajudar a lidar corretamente com essas situações complexas. Certifique-se de verificar a documentação para métodos úteis, como containspara pesquisar em uma string e replacepara substituir partes de uma string por outra string.

Vamos mudar para algo um pouco menos complexo: mapas de hash!
