fn main() {
    println!("Hello, world!");

    another_function(5);
	print_labeled_measurement(5, 'h');
	expression();

	// retornando valores de funções
	let x = five();
	println!("The value of x is: {x}");

	let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}



fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// **Instruções** são instruções que executam alguma ação e não retornam um valor.
// **As expressões** são avaliadas como um valor resultante.
fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("EXPRESSION - The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
	// se colocamos ; ele entende que é uma declaração e não uma expressão
	// não retornará o valor
	// receberar um erro
	// error[E0308]: mismatched types
    x + 1
}
