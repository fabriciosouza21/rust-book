fn main() {
	// Variables

	// Immutable variable
	// const
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

	// let é por padrão imutável
	// para tornar mutável, usar mut
    let mut x = 5;
    println!("The value of x is: {x}");
	x = 6;
	println!("The value of x is: {x}");

	// Shadowing
	// é possível redeclarar uma variável com o mesmo nome
	let x = 5;

    let x = x + 1;

    {
		// x é diferente do x de fora
		// cria um novo escopo
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

	// x é o mesmo atribuido na linha 19
	println!("The value of x is: {x}");

	// Tipos de dados diferentes
	// Rust é fortemente tipado
	// shadowing permite mudar o tipo de dado de uma variável, redeclarando-a
	// uma nova referência é criada - não tenho certeza
	let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of x is: {x}");

}
