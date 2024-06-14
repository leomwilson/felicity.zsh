use std::env;

use crate::colorize;
use crate::colorize::RGB;
use crate::catppuccin;

pub fn precmd_portion() {
    println!("~/pwd (main)");
}

pub fn prompt_portion() {
    // capture the exit status of the last command
    let color: &RGB = match env::var("EXIT_STATUS").unwrap_or("0".into()).as_str() {
        "0" => &catppuccin::MOCHA_GREEN,
        _ => &catppuccin::MOCHA_RED,
    };

    let pchar: &str = if is_root::is_root() {
        "#"
    } else {
        "$"
    };

    print!("{} ", colorize::fg(pchar, color));
}
