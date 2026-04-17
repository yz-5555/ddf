use std::path::{Path, PathBuf};
use std::{env, fs};

use clap::Parser;
use fs_extra::dir::{CopyOptions, copy};

use super::cli::{Args, Mode};
use super::list::List;

pub fn read_ddf_target() {
    let key = "DDF_TARGET";

    match env::var(key) {
        Ok(val) => env::set_current_dir(val).expect("ERR: FAILED TO CHANGE CWD"),
        Err(e) => panic!("ERR: FAILED TO READ DDF_TARGET / {}", e),
    }
}
pub fn read_ddf_list() {
    let args = Args::parse();
    let mut list_path = untildify(&args.list_dir);
    list_path.push("ddf-list.toml");

    let contents = fs::read_to_string(&list_path).expect("ERR: WRONG PATH");

    let list: List = toml::from_str(&contents).expect("ERR: FAILED TO PARSE");

    env::set_current_dir(list_path.parent().expect("ERR: FAILED TO FIND PARENT"))
        .expect("ERR: FAILED TO CHANGE CWD");

    for dotfile in list.dotfiles {
        match args.mode {
            Mode::Push => update(&dotfile.original, &dotfile.copy),
            Mode::Pull => update(&dotfile.copy, &dotfile.original),
        }
    }
}
fn update(src: &str, dst: &str) {
    let s = untildify(src);
    let d = untildify(dst);
    let mut options = CopyOptions::new();
    options.content_only = true;
    options.overwrite = true;

    if d.exists() {
        if d.is_file() {
            fs::remove_file(&d)
                .unwrap_or_else(|e| panic!("ERR: FAILED TO REMOVE FILE {} / {}", dst, e));
            println!("LOG: REMOVED FILE {}", dst);
        } else if d.is_dir() {
            fs::remove_dir_all(&d)
                .unwrap_or_else(|e| panic!("ERR: FAILED TO REMOVE DIR {} / {}", dst, e));
            println!("LOG: REMOVED DIR {}", dst);
        }
    }

    if s.is_file() {
        fs::copy(s, d)
            .unwrap_or_else(|e| panic!("ERR: FAILED TO COPY FILE {} to {} / {}", src, dst, e));
        println!("LOG: COPIED FILE {} TO {}", src, dst);
    } else if s.is_dir() {
        copy(s, d, &options)
            .unwrap_or_else(|e| panic!("ERR: FAILED TO COPY DIR {} TO {} / {}", src, dst, e));
        println!("LOG: COPIED DIR {} TO {}", src, dst);
    } else {
        println!("WARN: {} NOT FOUND", src);
    }
}
fn untildify(input_path: &str) -> PathBuf {
    let path = Path::new(input_path);

    if path.starts_with("~") {
        #[allow(deprecated)]
        if let Some(home) = std::env::home_dir() {
            let mut components = path.components();
            components.next();
            return home.join(components.collect::<PathBuf>());
        }
    }
    PathBuf::from(input_path)
}
