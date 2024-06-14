// many of these functions are here for completeness
#![allow(dead_code)]

pub struct RGB(pub u8, pub u8, pub u8);

const ANSI_RESET: &str = "\x1b[0m";

fn get_ansi_fg(rgb: &RGB) -> String {
    format!("\x1b[38;2;{};{};{}m", rgb.0, rgb.1, rgb.2)
}

fn get_ansi_bg(rgb: &RGB) -> String {
    format!("\x1b[48;2;{};{};{}m", rgb.0, rgb.1, rgb.2)
}

pub fn fg(text: &str, rgb: &RGB) -> String {
    format!("{}{}{}", get_ansi_fg(rgb), text, ANSI_RESET)
}

pub fn bg(text: &str, rgb: &RGB) -> String {
    format!("{}{}{}", get_ansi_bg(rgb), text, ANSI_RESET)
}
