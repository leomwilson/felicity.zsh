mod messages;
mod prompt;
mod colorize;
mod catppuccin;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        prompt::precmd_portion();
        prompt::prompt_portion();
        println!(); // add a newline at the end
        return;
    } else if args.len() == 2 {
        match args[1].as_str() {
            "-h" | "--help" => messages::help(),
            "-v" => messages::version(),
            "-k" => prompt::precmd_portion(),
            "-p" => prompt::prompt_portion(),
            "-z" => messages::zsh_script(),
            _ => messages::invalid(),
        }
        return;
    } else {
        messages::invalid();
        return;
    }
}
