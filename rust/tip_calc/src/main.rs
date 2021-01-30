mod tip_calculator;
mod tip_calc_console;
mod tip_calc_gui;

use std::env;
use crate::tip_calc_console::console_ui;
use crate::tip_calc_gui::graphic_ui;

fn main() {
    let param = env::args().nth(1).unwrap_or(String::new());

    if param.as_str() == "gui" {
        graphic_ui()
    } else {
        console_ui()
    }
}