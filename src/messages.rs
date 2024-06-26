const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn version() {
    println!("Felicity.zsh {} - a clean ZSH prompt using the Catppuccin Mocha color scheme",
        VERSION.unwrap_or("(unknown version)"));
    println!("(c) 2024 Leo Wilson, MIT License");
    println!("See https://gitlab.com/lwilson/felicity.zsh");
}

pub fn help() {
    println!("Usage: felicity [OPTION]\n");
    println!("Options:");
    println!("  -h  display this help message");
    println!("  -v  display version information");
    println!("  -k  display the precommand hook section of the prompt");
    println!("      (generally the first line of the prompt)");
    println!("  -p  display the prompt section of the prompt");
    println!("      (generally the second line of the prompt)");
    println!("  -z  print the ZSH script to use Felicity as your prompt");
    println!("\nIf Felicity is invoked without any options, it will print the full prompt.\n");
    println!("To use Felicity as your prompt, add the following line to your .zshrc:");
    println!("eval \"$(felicity -z)\"");
}

pub fn invalid() {
    eprintln!("Invalid option. Use 'felicity -h' for help.");
}

pub fn zsh_script() {
    println!("autoload -Uz add-zsh-hook");
    println!("felicity_precmd() {{");
    println!("  export EXIT_STATUS=$?");
    println!("  felicity -k");
    println!("  PROMPT=\"$(felicity -p)\"");
    println!("}}");
    println!("add-zsh-hook precmd felicity_precmd");
}
