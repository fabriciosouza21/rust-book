## Validando referências com lifetimes

Anotar tempos de vida não é um conceito que a maioria das outras linguagens de programação tem, então isso vai parecer estranho. Embora não abordemos tempos de vida em sua totalidade neste capítulo, discutiremos maneiras comuns pelas quais você pode encontrar a sintaxe de tempo de vida para que você possa se sentir confortável com o conceito.

## O verificador de empréstimos

``` rust

fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+

```
Aqui, anotamos o tempo de vida de r with 'a e o tempo de vida de x with 'b. Como você pode ver, o 'b bloco interno é muito menor que o 'abloco externo do tempo de vida. No momento da compilação, Rust compara o tamanho dos dois tempos de vida e vê que rtem um tempo de vida de 'abut que se refere à memória com um tempo de vida de 'b. O programa é rejeitado porque 'bé menor que 'a: o assunto da referência não vive tanto quanto a referência.

``` rust

fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+

```
Agora que você sabe quais são os tempos de vida das referências e como o Rust analisa os tempos de vida para garantir que as referências sempre serão válidas, vamos explorar os tempos de vida genéricos de parâmetros e valores de retorno no contexto de funções.


## Lifetimes em funções

``` rust

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

```
Se tentarmos implementar a longestfunção como mostrado na Listagem 10-20, ela não será compilada.


Queremos que a assinatura expresse a seguinte restrição: a referência retornada será válida enquanto ambos os parâmetros forem válidos. Essa é a relação entre os tempos de vida dos parâmetros e o valor de retorno. Nomearemos o tempo de vida 'ae então o adicionaremos a cada referência,
``` rust

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

```

Neste exemplo, string1 é válido até o fim do escopo externo, string2 é válido até o fim do escopo interno e result faz referência a algo que é válido até o fim do escopo interno. Execute este código e você verá que o verificador de empréstimo aprova; ele compilará e imprimirá The longest string


o código abaixo não compila porque a string2 é invalidada antes de result ser impresso. é preciso que a string1 e string2 tenha o mesmo tempo de vida para que o código de result seja válido.
``` rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}

```

Por exemplo, se alterássemos a implementação da longestfunção para sempre retornar o primeiro parâmetro em vez da maior fatia de string, não precisaríamos especificar um tempo de vida no y parâmetro.

``` rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

não compilar

 ``` rust

 fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

## Lifetimes em Structs

``` rust

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

```
A mainfunção aqui cria uma instância da ImportantExcerpt struct que contém uma referência à primeira frase do String owned by the variable novel. Os dados em novelexistem antes da ImportantExcerpt instância ser criada. Além disso, novelnão sai do escopo até depois que o ImportantExcerpt sai do escopo, então a referência na ImportantExcerpt instância é válida.

## Elisão vitalícia

### Definição
Mecanismo que permite omitir anotações de tempo de vida explícitas em casos comuns.

### Objetivo
Simplificar o código, permitindo que o compilador infira tempos de vida automaticamente.

### Regras Principais

1. **Parâmetros de Referência**:
   - Cada parâmetro recebe seu próprio tempo de vida.

2. **Um Único Parâmetro de Entrada**:
   - Seu tempo de vida é atribuído a todos os parâmetros de saída.

3. **Múltiplos Parâmetros de Entrada**:
   - Se um for `&self` ou `&mut self`, seu tempo de vida é atribuído a todos os parâmetros de saída.

### Benefícios
- Código mais limpo e legível
- Redução de anotações explícitas
- Mantém a segurança de memória de Rust

### Observação
Aplicável em muitos casos comuns, mas anotações explícitas podem ser necessárias em situações mais complexas.


 ``` rust
 fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

```


## Anotações de tempo de vida em definições de métodos

A declaração do parâmetro de tempo de vida após imple seu uso após o nome do tipo são obrigatórios, mas não somos obrigados a anotar o tempo de vida da referência por self causa da primeira regra de elisão.

``` rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

```

Aqui está um exemplo onde a terceira regra de elisão de tempo de vida se aplica:

``` rust

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

```

## A vida útil estática

``` rust
let s: &'static str = "I have a static lifetime.";

```


## Parâmetros de tipo genérico, limites de características e tempos de vida juntos

``` rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```


Esta é a longestfunção da Listagem 10-21 que retorna a maior das duas fatias de string. Mas agora ela tem um parâmetro extra chamado anndo tipo genérico T, que pode ser preenchido por qualquer tipo que implemente o Display trait conforme especificado pela wherecláusula. Este parâmetro extra será impresso usando {}, e é por isso que o Displaytrait bound é necessário. Como os lifetimes são um tipo de genérico, as declarações do parâmetro lifetime 'ae do parâmetro do tipo genérico Tvão na mesma lista dentro dos colchetes angulares após o nome da função.


## resumo

Abordamos muita coisa neste capítulo! Agora que você sabe sobre parâmetros de tipo genérico, traits e limites de traits, e parâmetros de tempo de vida genéricos, você está pronto para escrever código sem repetição que funciona em muitas situações diferentes. Parâmetros de tipo genérico permitem que você aplique o código a diferentes tipos. Traits e limites de traits garantem que, mesmo que os tipos sejam genéricos, eles terão o comportamento que o código precisa. Você aprendeu a usar anotações de tempo de vida para garantir que este código flexível não tenha nenhuma referência pendente. E toda essa análise acontece em tempo de compilação, o que não afeta o desempenho do tempo de execução!

Acredite ou não, há muito mais a aprender sobre os tópicos que discutimos neste capítulo: o Capítulo 17 discute objetos de trait, que são outra maneira de usar traits. Há também cenários mais complexos envolvendo anotações de tempo de vida que você só precisará em cenários muito avançados; para esses, você deve ler a Rust Reference . Mas, em seguida, você aprenderá como escrever testes em Rust para ter certeza de que seu código está funcionando da maneira que deveria.
