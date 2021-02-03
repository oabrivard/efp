use std::io::stdin;
use chrono::{Utc, Datelike};

fn read_positive_int() -> i32 {
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("error reading data");

        if let Ok(result) = buffer.trim().parse::<i32>() {
            if result >= 0 {return result;}
        }

        println!("Please enter a number >= 0 :");
    }
}

fn main() {
    println!("What is your current age?");
    let age = read_positive_int();

    println!("At what age would you like to retire?");
    let retirement_age = read_positive_int();

    let left_to_work = retirement_age - age;

    if left_to_work < 0 {
        println!("You can already retire!")
    } else {
        println!("You have {} years left until you can retire.", left_to_work);

        let current_year = Utc::now().year();
        let retirement_year = current_year + left_to_work;
        println!("It's {}, so you can retire in {}.", current_year, retirement_year);
    }
}
