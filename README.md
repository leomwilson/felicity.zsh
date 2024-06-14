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

Felicity is very much a work in progress. Don't install it quite yet,
unless you're ready to deal with bugs and missing features.

Felicity is written in Rust, so you'll need to have Rust installed.
See [rustup.rs](https://rustup.rs) for instructions.

Install Felicity using Cargo:
```sh
cargo install --git https://gitlab.com/lwilson/felicity.zsh.git
```
If you'd like, you can run `felicity` to see what the prompt looks like.
You can try it in a few different environments to test its features and behavior.

Once you're ready, add this line to your `.zshrc`:
```sh
eval "$(felicity -z)"
```
Make sure you don't have any other prompt themes installed,
as they may conflict with Felicity.

## Features

Felicity displays the following information:
- (nothing yet)

## Dependencies

My goal is to keep Felicity as lightweight as possible. As such,
I've only included dependencies when there's no clean way to implement
a feature myself. Currently, Felicity depends on these crates:
- [is_root](https://crates.io/crates/is_root) for detecting root users
  (using this crate makes the process far less painful for cross-platform)
