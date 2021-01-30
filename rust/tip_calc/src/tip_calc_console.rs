use std::io::{stdin, stdout, Write};
use crate::tip_calculator::*;

macro_rules! print_flush {
    ( $($t:tt)* ) => {
        {
            let mut h = stdout();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}

fn read_positive_float() -> f64 {
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("error reading data");

        if let Ok(result) = buffer.trim().parse::<f64>() {
            if result >= 0.0 {return result;}
        }

        print_flush!("Please enter a number >= 0 : ");
    }
}

pub fn console_ui() {
    print_flush!("What is the bill? ");
    let bill = read_positive_float();

    print_flush!("What is the tip percentage? ");
    let tip_percent = read_positive_float();

    let PaymentDetails {tip, total_amount} = calculate_payment_details(bill, tip_percent);

    println!("The tip is {:.2}\nThe total is {:.2}\n", tip, total_amount);
}
