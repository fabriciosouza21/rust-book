
> [!Variaveis]
> 
> Por padrão, as variáveis são imutáveis  

quando queremos ter uma variável mutável precisamos utilizar o mut

``` rust
let mut x = 5; 
println!("The value of x is: {x}"); x = 6; 
println!("The value of x is: {x}");
```


## Constantes

 ``` rust
 const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
 ```

As contantes são válidas durante todo o tempo de execução de um programa, dentro do escopo em que foram declaradas, 

acredito que seja mais semântica 

## Shadowing

é basicamente retribuir a variável, basicamente criar uma nova referencia para a mesma variavel no mesmo escopo, a logica de shadowing leva em consideração o scopo

o let reatribuir o valor, a referencia e alterada para o escopo


 ``` rust
 fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
 ```

``` rust

    let spaces = "   ";
    let spaces = spaces.len();

```

Quando utilizamos o let podemos atribuir um novo valor sempre sem se preocupar com tipagem, sem entra em detalhes e reset na tipagem, podemos utiliza o mesmo nome para tipos diferente por que criamos novas variaveis no scopo