use std::{
    fs::{create_dir_all, File},
    io::{self, stdin, Write},
    process::exit,
};

use clap::{command, Parser};
use paths::{get_os_config_dir, get_os_dir_sep};

use profile::*;

/// Save current dir as a profile file.
#[derive(Parser, Debug)]
#[command(author = "lI15SO0", version, about)]
struct Args {
    /// Profile name
    #[arg(short, long)]
    name: Option<String>,

    /// Force create profile.
    #[arg(short, long)]
    force: bool,

    /// Allow empty files.
    #[arg(short, long)]
    raw: bool,
}

const PROFILES_DIR_NAME: &str = "profiles";
const PROFILE_SUFFIX: &str = ".bincode";

fn main() {
    let args = Args::parse();

    let path = "./".to_string();
    let name = args.name.unwrap_or_else(|| {
        let mut line = String::new();
        print!("profile file name: ");
        let _ = io::stdout().flush();
        stdin().read_line(&mut line).unwrap();
        line.trim().to_owned()
    });

    let profile_target =
        get_os_config_dir() + PROFILES_DIR_NAME + &get_os_dir_sep() + &name + PROFILE_SUFFIX;

    let profile = match args.raw {
        true => DirRoot::from_dir_raw(path).unwrap_or_else(|err| {
            // TODO: Maybe not friendly.
            eprintln!("E: Failed to get dir infomations. cause: {}", err);
            exit(1);
        }),
        false => DirRoot::from_dir(path).unwrap_or_else(|err| {
            eprintln!("E: Failed to get dir infomations. cause: {}", err);
            exit(1);
        }),
    };

    match ensure_dirs() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("E: Failed to init config dir: {}", err);
            exit(1)
        }
    };

    is_exists(&profile_target, args.force);
    match profile.save_as(&profile_target) {
        Ok(_) => println!("Saved successfully: {}", profile_target),
        Err(err) => eprintln!("E: Failed to save profile file: {}", err),
    };
}

fn ensure_dirs() -> Result<(), std::io::Error> {
    let p = get_os_config_dir() + PROFILES_DIR_NAME;
    if File::open(&p).is_err() {
        let _ = create_dir_all(p)?;
    }
    Ok(())
}

fn is_exists(path: &str, force: bool) {
    if !force && File::open(path).is_ok() {
        print!("Profile file exists, Write anyway? (Y/N): ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        let line = line.trim().to_lowercase();
        if line != "y" {
            println!("Give up!");
            exit(0);
        }
    }
}
