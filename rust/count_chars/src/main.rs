use std::io::{stdin, stdout, Result, Write};
use std::env;

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

fn console_ui() -> Result<()> {
    print!("What is the input string? ");
    stdout().flush()?;
    let mut buffer = String::new();
    loop {
        stdin().read_line(&mut buffer)?;

        if buffer.trim().len() > 0 {
            break;
        } else {
            print!("Please enter a valid string: ");
            stdout().flush()?;
        }
    }
    let buffer = buffer.trim();
    println!("{} has {} characters", buffer, buffer.len());
    Ok(())
}

fn graphic_ui() -> Result<()> {
    let application = gtk::Application::new(
        Some("fr.abrivard.count_char"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Char Counter");
        window.set_default_size(350, 70);

        let string_label = gtk::Label::new(Some("Enter a string:"));
        string_label.set_halign(gtk::Align::Start);
        let string_input = gtk::Entry::new();
        let count_label = gtk::Label::new(Some("0 character"));
        count_label.set_halign(gtk::Align::Start);


        let count_label_clone = count_label.clone();
        let string_input_clone = string_input.clone();
        string_input.connect_changed(move |_| {
            if let Some(value) = string_input_clone.get_text() {
                if value.len() > 1 {
                    count_label_clone.set_label(format!("{} characters",value.len()).as_str());
                } else {
                    count_label_clone.set_label(format!("{} character",value.len()).as_str());
                }
            }
        });

        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        container.add(&string_label);
        container.add(&string_input);
        container.add(&count_label);

        window.add(&container);
        window.show_all();
    });

    application.run(&[]);
    Ok(())
}

fn main() -> Result<()> {
    let param = env::args().nth(1).unwrap_or(String::new());

    if param.as_str() == "gui" {
        graphic_ui()
    } else {
        console_ui()
    }
}
