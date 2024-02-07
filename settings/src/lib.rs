use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub wsinit: Wsinit,
}

impl Settings {
    pub fn read_from(path: &str) -> Result<Self, (usize, String)> {
        let mut f = match File::open(path) {
            Ok(it) => it,
            Err(err) => return Err((1, err.to_string())),
        };
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        Ok(match toml::from_str(&s) {
            Ok(it) => it,
            Err(err) => return Err((2, err.to_string())),
        })
    }

    pub fn new() -> Self {
        Settings {
            wsinit: Wsinit::new(),
        }
    }

    pub fn write(self: &Self, path: &str) -> Result<(), std::io::Error> {
        let mut f = File::open(path).unwrap_or(File::create(path)?);
        let s = toml::to_string(self).unwrap();
        write!(f, "{}", s)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Wsinit {
    default_config: String,
}

impl Wsinit {
    pub fn new() -> Self {
        Wsinit {
            default_config: String::new(),
        }
    }

    pub fn get_default(self: &Self) -> String {
        self.default_config.clone()
    }

    pub fn set_default(self: &mut Self, default: &str) {
        self.default_config = default.to_string();
    }
}
