/* métodos são semelhantes a funções, os métodos são definidos no contexto de uma strct
 */#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl é uma palavra-chave que declara um bloco de implementação, associação
impl Rectangle {
	// &self é uma referência imutavel ao próprio objeto
	fn area(&self) -> u32 {
		self.width * self.height
	}


	/*
		Dado o receptor e o nome de um método, rust pode descubri definitivamente se o método está lendo(&self),
		escrevendo(&mut self) ou tomando posse(self)
	 */

	 fn width(&self) -> bool {

		self.width > 0
	}

	// &self é uma referência imutavel ao próprio objeto
	// &other é uma referência imutavel ao outro objeto
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}


/* Funções Associadas*/

// Funções associadas são funções que não tem um receptor, são chamadas diretamente na struct
// São usadas para funções que não precisam de um objeto para funcionar
// Similar ao método estático do Java
// Ou variáveis de classe em outras linguagens como Python e ruby
// a sintaxe para chamar uma função associada é <Struct>::<função>
// utilizada geralmente para construtores
// podemos ter varias implementações para uma struct e cada uma pode ter suas funções associadas
impl Rectangle {
	fn square (size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}



fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

	/*
		Rust possuir um recurso de referência e desreferência automaticas
	 */
	if rect1.width() {
		print!("the rectangle has a nonzero width; it is  {}", rect1.width);
	}

	let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

	let sq = Rectangle::square(3);


}
