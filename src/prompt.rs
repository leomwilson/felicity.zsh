use crate::colorize;
use crate::catppuccin;

pub fn precmd_portion() {
    println!("~/pwd (main)");
}

pub fn prompt_portion() {
    print!("{}", colorize::fg(">", &catppuccin::MOCHA_RED));
}
