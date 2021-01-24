extern crate glib;
extern crate gtk;

use gtk::prelude::*;
use glib::Value;

use std::io::{stdin, stdout, Write};
use std::env;

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

fn console_ui() {
    print_flush!("What is the bill? ");
    let bill = read_positive_float();

    print_flush!("What is the tip percentage? ");
    let tip_percent = read_positive_float();

    let tip = (tip_percent * bill).round() / 100.0;

    println!("The tip is {:.2}\nThe total is {:.2}\n", tip, bill + tip);
}

fn graphic_ui() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("tip_calc_window.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("MainWindow").unwrap();
    window.connect_delete_event(|_, _| {
        // Stop the main loop.
        gtk::main_quit();
        // Let the default handler destroy the window.
        gtk::Inhibit(false)
    });

    let bill_entry: gtk::Entry = builder.get_object("BillEntry").unwrap();
    let tip_entry: gtk::Entry = builder.get_object("TipEntry").unwrap();
    let compute_button: gtk::Button = builder.get_object("ComputeCmd").unwrap();
    let tip_amount_label: gtk::Label = builder.get_object("TipAmountLbl").unwrap();
    let total_bill_label: gtk::Label = builder.get_object("TotalBillLbl").unwrap();
    let tip_scale: gtk::Scale = builder.get_object("TipScale").expect("Couldn't get scale");
    let tip_scale_adj = tip_scale.get_adjustment();

    tip_scale_adj
        .bind_property("value", &tip_entry, "text")
        .flags(
            glib::BindingFlags::DEFAULT
                | glib::BindingFlags::SYNC_CREATE
                | glib::BindingFlags::BIDIRECTIONAL
        )
        .transform_from(|_, srcval| {
            let val = srcval.get::<String>().unwrap().unwrap();
            if let Ok(result) = val.trim().parse::<f64>() {
                if result >= 0.0 {return Some(result.to_value());}
            }
            Some((0.0).to_value())
        })
        .transform_to(|_, srcval| {
            let val = srcval.get::<f64>().unwrap().unwrap();
            Some(Value::from(format!("{:.2}",val).as_str()))
        })
        .build();

    compute_button.connect_clicked(move |_| {
        let bill_text = bill_entry.get_text();
        let tip_text = tip_entry.get_text();

        if let Ok(bill) = bill_text.trim().parse::<f64>() {
            if let Ok(tip_percent) = tip_text.trim().parse::<f64>() {
                if bill >= 0.0 && tip_percent >= 0.0 {
                    let tip = (tip_percent * bill).round() / 100.0;

                    tip_amount_label.set_label(format!("The tip is {:.2}",tip).as_str());
                    total_bill_label.set_label(format!("The total is {:.2}",bill+tip).as_str());
                    return;
                }
            }
        }

        tip_amount_label.set_label("Please enter numbers >= 0");
        total_bill_label.set_label("");
    });

    window.show_all();

    gtk::main();
}

fn main() {
    let param = env::args().nth(1).unwrap_or(String::new());

    if param.as_str() == "gui" {
        graphic_ui()
    } else {
        console_ui()
    }
}