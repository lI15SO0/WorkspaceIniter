use clap::{command, Parser};
use paths::{get_os_config_dir, get_os_dir_sep, get_profile_path};
use profile::DirRoot;
use settings::{Settings, Wsinit};
use std::{
    fs::{self, create_dir_all, File},
    process::exit,
};

#[derive(Parser)]
#[command(author = "lI15SO0", version, about = "Init workspace by profile file.")]
struct Args {
    #[arg(short = 'c', long)]
    profile: Option<String>,

    #[arg(short = 'd', long)]
    target: Option<String>,

    #[arg(short, long)]
    print: bool,

    #[arg(short, long)]
    force: bool,

    #[arg(short, long)]
    list: bool,

    #[arg(short, long = "set-default")]
    setdefault: bool,
}

const SETTING_NAME: &str = "settings.toml";

const PROFILES_DIR_NAME: &str = "profiles";
const _PROFILE_SUFFIX: &str = ".bincode";

fn ensure_dirs() -> Result<(), std::io::Error> {
    let p = get_os_config_dir() + PROFILES_DIR_NAME;
    if File::open(&p).is_err() {
        let _ = create_dir_all(p)?;
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    multi_operation_checker(&args);

    let settings = {
        let path = get_os_config_dir() + SETTING_NAME;
        match ensure_dirs() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("E: Failed to init config dir: {}", err);
                exit(1)
            }
        };
        Settings::read_from(&path).unwrap_or_else(|err| {
            if err.0 == 1 {
                let settings = Settings {
                    wsinit: Wsinit::new(),
                };
                settings.write(&path).expect("E: Failed to init settings.");
                return settings;
            } else {
                eprintln!("W: Error in reading settings cause: {}", err.1);
                return Settings {
                    wsinit: Wsinit::new(),
                };
            }
        })
    };

    if args.list {
        print_all_profiles();
        exit(0);
    }

    if args.setdefault {
        set_default(settings, args);
        exit(0);
    }

    if args.print {
        show_profile_content(args);
        exit(0);
    }

    build_workspace(settings, args);
}

fn multi_operation_checker(args: &Args) {
    let mut multi_checker = 0;
    if args.list {
        multi_checker <<= 1;
        multi_checker += 1;
    }
    if args.setdefault {
        multi_checker <<= 1;
        multi_checker += 1;
    }
    if args.print {
        multi_checker <<= 1;
        multi_checker += 1;
    }

    if multi_checker > 1 {
        println!("Pls do not use multi operation args.");
        exit(1);
    }
}

//==============================================================================
//   :Builder
//==============================================================================

fn build_workspace(settings: Settings, args: Args) {
    let profile_name = args
        .profile
        .clone()
        .unwrap_or(get_default_profile(&settings));
    if profile_name == "" {
        println!("E: Not give a profile name, and not set default profile.");
        exit(0);
    }

    let dir_root = {
        let profile_path = get_profile_path(profile_name.clone());
        DirRoot::read_from(&profile_path).unwrap_or_else(|err| {
            eprintln!("E: Failed to read profile. cause: {}", err);
            exit(1)
        })
    };

    let target = args.target.unwrap_or("./".to_string());

    build_workspace_from_root(dir_root, &target);
}

fn build_workspace_from_root(dir_root: DirRoot, target: &str) {
    check_repeat(&dir_root, target);

    let target = {
        if target.ends_with('/') || target.ends_with('\\') {
            target.to_string()
        } else {
            target.to_string() + &get_os_dir_sep()
        }
    };

    fn _build(dirs: &DirRoot, prefix: &str) -> Result<(), std::io::Error> {
        for f in &dirs.dirs {
            fs::create_dir_all(prefix.to_string() + &f.name)?;
            println!("Created dir: {}", prefix.to_string() + &f.name);
            _build(
                &f,
                &(prefix.to_string() + &f.name.clone() + &get_os_dir_sep()),
            )?;
        }

        for f in &dirs.files {
            let size = f.write(&(prefix.to_string() + &f.name))?;
            println!(
                "Created file: {} , size: {}",
                prefix.to_string() + &f.name,
                size
            );
        }
        return Ok(());
    }

    match _build(&dir_root, &target) {
        Ok(_) => {}
        Err(err) => {
            eprintln!(
                "E: An error has occupied when create dirs: {}",
                err.to_string()
            );
            exit(1);
        }
    };
}

fn check_repeat(dir_root: &DirRoot, target: &str) {
    println!("Checking repats.");
    let lst_of_dir_root = {
        let mut dir_lst: Vec<String> = dir_root.dirs.iter().map(|d| d.name.clone()).collect();
        let mut file_lst: Vec<String> = dir_root.files.iter().map(|f| f.name.clone()).collect();
        dir_lst.append(&mut file_lst);
        dir_lst
    };

    let mut repeat_flag = false;

    let mut repeat: Vec<String> = vec![];
    for entry in fs::read_dir(target).unwrap() {
        let f_name = {
            let entry = entry.unwrap();
            entry.file_name().to_str().unwrap().to_string()
        };
        if lst_of_dir_root.contains(&f_name) {
            repeat_flag = true;
            repeat.push(f_name);
        };
    }
    if repeat_flag {
        eprintln!("Detected repeats:");
        repeat.iter().for_each(|s| eprintln!("\t{}", s));
        eprintln!("This dir has a repeat dir, stop build workspace.(use -f to force build).");
        exit(1);
    }
    println!("Generated done!");
}

fn print_profiles_lst(profiles: &Vec<String>) {
    if profiles.is_empty() {
        println!("Not have any profiles.");
    } else {
        println!("List of profiles:");
        profiles.iter().fold(1, |num, profile| {
            println!("\t{}: {}", num, profile);
            num + 1
        });
    };
}

//------------------------------------------------------------------------------
//   Builder:
//------------------------------------------------------------------------------

//==============================================================================
//   :profiles
//==============================================================================

fn show_profile_content(args: Args) {
    let dir_root = {
        let profile_name = args.profile.unwrap_or_else(|| {
            println!("E: Not give profile name arg.");
            exit(1);
        });
        let profile_content = get_profile_path(profile_name);
        DirRoot::read_from(&profile_content).unwrap_or_else(|err| {
            eprintln!("E: Failed to read profile. cause: {}", err);
            exit(1);
        })
    };

    dir_root.info();
}

fn get_default_profile(settings: &Settings) -> String {
    settings.wsinit.get_default()
}

fn get_profiles() -> Result<Vec<String>, std::io::Error> {
    let conf_dir = get_os_config_dir() + PROFILES_DIR_NAME;
    Ok(fs::read_dir(conf_dir)?
        .map(|e| {
            let f = e.unwrap();
            f.file_name().to_str().unwrap().to_string()
        })
        .collect())
}

fn print_all_profiles() {
    print_profiles_lst(&get_profiles().unwrap_or(vec![]))
}

//------------------------------------------------------------------------------
//   profiles:
//------------------------------------------------------------------------------

//==============================================================================
//   :Settings
//==============================================================================
fn set_default(mut settings: Settings, args: Args) {
    let profile = if let Some(profile) = args.profile {
        profile
    } else {
        let mut buf = String::new();
        let profiles = match get_profiles() {
            Ok(it) => it,
            Err(err) => {
                eprintln!("E: Error in get all profiles. cause: {}", err);
                exit(1)
            }
        };

        print_profiles_lst(&profiles);
        println!("Which one: (0 for cancle)");

        std::io::stdin().read_line(&mut buf).unwrap();
        let num = match buf.trim().parse::<usize>() {
            Ok(it) => it,
            Err(_) => {
                eprintln!("Unable get a valid number.");
                exit(1);
            }
        };
        if num == 0 {
            println!("Cancle!");
            exit(0);
        }

        profiles
            .get(num - 1)
            .unwrap_or_else(|| {
                eprintln!("Please input a valid number.");
                exit(1);
            })
            .clone()
    };

    settings.wsinit.set_default(&profile);
    match settings.write(&(get_os_config_dir() + SETTING_NAME)) {
        Ok(_) => {
            println!("Success.");
            exit(0);
        }
        Err(err) => {
            eprintln!("E: Failed write settings. cause: {}", err);
            exit(1);
        }
    };
}

//------------------------------------------------------------------------------
//   Settings:
//------------------------------------------------------------------------------
