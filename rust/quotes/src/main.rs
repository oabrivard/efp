use std::io::stdin;

fn main() {
    let quotes = vec![("Olivier","quote numero 1"),("Soiz","quote numero 2")];

    println!("What is the quote ?");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("unable to read data");
    let quote = buffer.trim();

    println!("Who said it ?");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("unable to read data");
    let author = buffer.trim();

    let sentence = String::new() + author + " says, \"" + quote + "\"";
    println!("{}", sentence);
    //println!("{} says, \"{}\"", author, quote);

    for (author, quote) in quotes {
        let sentence = String::new() + author + " says, \"" + quote + "\"";
        println!("{}", sentence);
    }
}
