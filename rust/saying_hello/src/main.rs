use std::io::stdin;

fn main() {
    println!("What is your name?");

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("error reading data");

    println!("Hello {}, nice to meet you!", buffer.trim());
}
