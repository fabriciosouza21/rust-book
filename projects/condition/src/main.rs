fn main() {
    let number = 6;

   if number % 4 ==0 {
	println!("number is divisible by 4");
   }else if number % 3 == 0 {
	println!("number is divisible by 3");
   }else if number % 2 ==0  {
	println!("number is divisible by 2");
   }else {
	println!("number is not divisible by 4, 3, or 2");
   }

   let condition = true;
   // o if else é uma expressão, então pode ser usado para atribuir valores
   // a uma variável
   // funciona da mesma forma que o operador ternário em outras linguagens

   let number = if condition { 5 } else { 6 };

   println!("The value of number is: {}", number);

   let result = loop_return();


   println!("The result is {}", result);

   let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


   loop_aninhado();
   for_loop();
   forech_loop();
   while_loop();
   revert_loop();



}

fn loop_return() -> i32 {
	let mut counter = 0;

	loop {
		counter += 1;

		if counter == 10 {
			return counter * 2;
		}
	};


}

fn loop_aninhado() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}


fn forech_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn revert_loop() {
	// O método rev() reverte a ordem dos elementos
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
