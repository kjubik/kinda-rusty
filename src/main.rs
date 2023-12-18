use std::io;


fn main() {
    println!("Please, enter in a word.");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input = user_input.trim_end_matches("\n");

    let chars: Vec<char> = user_input.chars().collect();

    let length = user_input.len();

    for i in 0..(length / 2) {
        if chars[i] != chars[length-1-i] {
            println!("{} is not a palindrome.", user_input);
            return;
        }
    }

    println!("{} is a palindrome!", user_input);
}
