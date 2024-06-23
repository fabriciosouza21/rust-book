
/* é um atributo outer para imprimir o valor no modo debug sem ele recebemos um erro no console  */
#[derive(Debug)]
struct Rectangle {
	width:u32,
	height:u32
}

fn main() {

	let rect1 = Rectangle {
		width:30,
		height:50
	};
	print!(
		"The are of the rectangle is {} square pixels.",
		area(&rect1)
	);

	/* Adicionando funcionalidade útil com características derivadas*/
	let rect1 = Rectangle {
		width:30,
		height:50
	};

	// para imprimir o valor utilizando println, precisamos implementar Display
	// os tipos primitivos já possuem implementação de Display
    // imprimir no modo debug, com o atributo #[derive(Debug)]
	// e utilizar {:?} para imprimir
	println!("rect1 is {:?}", rect1);

	// imprimir no modo debug, com o atributo #[derive(Debug)]
	// e utilizar {:#?} para imprimir
	// retorna um formato mais legível
	print!("rect1 is {:#?}", rect1);


	/*
	 dbg! é uma macro que imprime o valor de uma expressão
	 e retorna o valor da expressão
	 é muito útil para debugar
	 retorna o valor e o tipo da expressão
	 */
	let scale = 2;
	let rect2 = Rectangle {
		width: dbg!(30 * scale),
		height: 50
	};

	dbg!(&rect2);


}

// &Rectangle é um emprestimo imutável
fn area(rectangle : &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}
