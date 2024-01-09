use std::io;

fn main() {
    println!("Input your name: ");

    let mut buffer = String::new();
    let result = io::stdin().read_line(&mut buffer);
    result.expect("An error has ocurred while reading the line");
    let name = buffer.replace("\n", "");

    println!("Hello, {name}!")
}
