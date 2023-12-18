use std::io;


fn main() {
    println!("Please, enter in a word.");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input = user_input.trim_end_matches("\n");
}
