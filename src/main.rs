use std::io;

fn is_even_iterate(number: &i32) -> bool {
    if number % 2 == 0 { return true }
    else { return false }
}

fn is_even_recurent(number: &mut i32) -> bool {
    if *number == 0 { return true }
    if *number == 1 { return false }
    is_even_recurent(&mut (*number - 2))
}

fn main() {
    println!("Enter in a number");
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input = user_input.trim_end_matches("\n");

    let mut number: i32 = match user_input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        },
    };

    if is_even_iterate(&number) { println!("{} is even", number) }
    else { println!("{} is odd", number) }

    if is_even_recurent(&mut number) { println!("{} is even", number) }
    else { println!("{} is odd", number) }
}
