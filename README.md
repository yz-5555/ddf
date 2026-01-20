# ddf
A dotfiles manager written in Rust

# Usage
- You must have an environment variable `DDF_TARGET`. Put the path of dotfiles repository in it. There is no default value.
- 1st param: `push` or `pull`. `push` is original -> copy. `pull` is copy -> original. There is no default value.
- 2nd param: the directory path where `ddf-list.toml` exists. The default value is `.`.
- Examples: `ddf push win11` `ddf pull ubuntu`, ...
- Check [my dotfiles](https://github.com/yz-5555/dotfiles) to see how a `ddf-list.toml` look like.

# Why
Writing pwsh script is way too painful.

# Notes
- Not on crates.io.
- No license.
- My first Rust project.
- Not sure if this is blazingly fast.
- Unlikely to be maintained.
