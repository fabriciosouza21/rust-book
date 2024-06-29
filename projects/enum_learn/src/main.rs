

/**
 * Nomes de classes, enums , devem ser pascal case PascalCase
 */

 /**
  * as variaveis, atributos de struct e funções devem ser snake case snake_case
  */
enum IpAddrKind {
	V4,
	V6,
}

struct IpAddr{
	kind: IpAddrKind,
	address: String
}

/**
 * Podemos associar valores a cada variante do enum
 * Nesse caso, associamos um valor do tipo String a cada variante
 * O IpAddrKind::V4() é uma chamada que recebe um valor do tipo String
 * e retorna um valor do tipo IpAddrKind
 */

enum IpAddrKind2 {
	V4(String),
	V6(String)
}

/**
 * a vantagem em usar enums é que podemos associar valores de tipos diferentes
 */
enum IpAddr3 {
	V4(u8, u8,u8,u8),
	V6(String)
}

struct Ipv4Addr {
	// campos
}

struct Ipv6Addr {
	// campos
}

/**
 * podemos associar structs a enums
 *
 */
enum IpAddr4 {
	V4(Ipv4Addr),
	V6(Ipv6Addr)
}

/**
 * podemos associar uma variedade de tipos
 */

enum Message {
	Quit,
	Move {x: i32, y:i32},
	Write(String),
	ChangeColor(i32, i32, i32)
}

/** As estuturas abaixo contem os mesmo dados que a varieates de enum acima,
 * não precisamos criar estrutura para cada tipo
 * */

//struct QuitMessage; // unit struct
//struct MoveMessage {
//    x: i32,
//    y: i32,
//}
//struct WriteMessage(String); // tuple struct
//struct ChangeColorMessage(i32, i32, i32); // tuple struct

// podemos implementar métodos em enums
impl Message {
	fn call(&self){
		// método body
	}
}

fn main() {
    let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;

	route(four);
	route(six);

	let home = IpAddr {
		kind: IpAddrKind::V4,
		address: String::from("127.0.0.1")
	};

	let loopback = IpAddr {
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	};

	let home2 = IpAddrKind2::V4(String::from("127.0.0.1"));
	let loopback2 = IpAddrKind2::V6(String::from("::1"));

	let home3 = IpAddr3::V4(127,0,0,1);
	let loopback3 = IpAddr3::V6(String::from("::1"));


	let m = Message::Write(String::from("hello"));
	m.call();

	/**
	 * OptionsEnum é enum definido pela biblioteca padrão
	 * Rust não tem valores nulos, mas tem um enum que pode codifica o coneito de um
	 * valor esta presente ou ausente Option<t>
	 * consigo fazer uma conparação com Optional do Java
	 */
	 let some_number = Some(5);
	 let some_string = Some("a string");

	 let absent_number: Option<i32> = None;

	 let x: i8 = 5;
	 let y: Option<i8> = Some(5);
	 // não podemos somar x + y, pois y é um Option<i8> e x é um i8
	 //let sum = x + y;
}

fn route(ip_kind: IpAddrKind){}
