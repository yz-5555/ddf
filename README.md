# df
A dotfiles manager written in Rust

# Usage
- You must have an environment variable `DF_TARGET`. Put the path of dotfiles repository in it. There is no fallback or default value.
- 1st param: `push` or `pull`. `push` is original -> copy. `pull` is copy -> original.
- 2nd param: the directory path where `df-list.toml` exists.
- Examples: `df push win11` `df pull ubuntu`, ...
- Check [my dotfiles](https://github.com/yz-5555/dotfiles) to see how a `df-list.toml` look like.

# Why
I got bored. I don't like writing pwsh scripts. I needed a cross-platform tool.

# Notes
- Not on crates.io.
- No license.
- My first Rust project.
- Not sure if this is blazingly fast.
- Unlikely to be maintained.
