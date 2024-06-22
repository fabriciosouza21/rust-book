fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

	let mut s = String::from("hello");

    change(&mut s);
}

// &String means that the function takes a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
}

// &mut String means that the function takes a mutable reference to a String
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
