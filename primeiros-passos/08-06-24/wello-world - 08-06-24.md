

Melhor ainda, a linguagem foi projetada para guiá-lo naturalmente em direção a um código confiável e eficiente em termos de velocidade e uso de memória.


## Equipes de desenvolvedores
No Rust, o compilador desempenha um papel de guardião, recusando-se a compilar código com esses bugs indescritíveis, incluindo bugs de simultaneidade. Trabalhando junto com o compilador, a equipe pode gastar seu tempo concentrando-se na lógica do programa em vez de perseguir bugs.

- Cargo, o gerenciador de dependências e ferramenta de construção incluído, torna a adição, compilação e gerenciamento de dependências simples e consistente em todo o ecossistema Rust.
- A ferramenta de formatação Rustfmt garante um estilo de codificação consistente entre os desenvolvedores.
- O Rust Language Server potencializa a integração do Ambiente de Desenvolvimento Integrado (IDE) para conclusão de código e mensagens de erro embutidas.


## instalação 

O comando baixa um script e inicia a instalação da `rustup` ferramenta, que instala a versão estável mais recente do Rust. Sua senha pode ser solicitada. Se a instalação for bem-sucedida, a seguinte linha aparecerá:

``` curl

$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh


 ```



## hello wold

``` rust
fn main() {
    println!("Hello, world!");
}
```

O Estilo Rust consiste em recuar com quatro espaços, não com uma tabulação

precisa saber que usar um `!` **significa que você está chamando uma macro em vez de uma função normal** e que as macros nem sempre seguem as mesmas regras que as funções .

Quarto, terminamos a linha com ponto e vírgula ( `;`)

## compilar com rustc

compilar com rustc é bom para programs simples, mas á medida que seu projeto cresce, 
você desejara gernciar todas as opções e facilitar o compartilhamento de seu código. A seguir


## Ola carga



