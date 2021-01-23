use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    print!("What is your name? ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("Hello, {}, nice to meet you!", input.trim());
    Ok(())
}
