# Felicity.zsh
A clean ZSH prompt using the Catppuccin Mocha color scheme.

Note that Felicity is very much a personal project; it is designed
according to my personal taste and the tools I use. The codebase is
intentionally modular, so you're more than welcome to fork it and
tailor things to your own preferences.

## Contributing
The project is in its early stages, so while I'm open to contributions,
you're probably better off using a more mature project. I recommend
[Pista](https://github.com/nerdypepper/pista) if you're looking for a
similar vibe. If you do decide to contribute, or if you'd like to open
an issue, I highly encourage you to do so using the
[GitLab repository](https://gitlab.com/lwilson/felicity.zsh)
rather than the mirror on GitHub.

I use ZSH on MacOS, and I have not tested this on other platforms. If
you run into any issues on another OS, please let me know. For the time
being, support for other shells is outside the scope of this project.

## Installation

Install Felicity using Cargo:
```sh
cargo install --git https://gitlab.com/lwilson/felicity.zsh.git
```
Then, add this line to your `.zshrc`:
```sh
eval "$(felicity -z)"
```
Make sure you don't have any other prompt themes installed,
as they may conflict with Felicity.
