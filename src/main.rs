use std::io;


fn isEvenIterate(number) {
    if number % 2 == 0 { return true }
    else { return false }
}

fn isEvenRecurent(number) {
    while number > 1 {
        if number == 0 { return true }
        if number == 1 { return false }
        isEvenRecurent(number - 2)
    }
}

fn main() {
    println!("Enter in a number");
    
}
