use bincode;
use serde::{Deserialize, Serialize};
use std::{
    env::consts::OS,
    fs::{self, File},
    io::{Read, Write},
};

// =============================================================================

#[derive(Serialize, Deserialize)]
pub struct DirRoot {
    pub name: String,
    pub dirs: Vec<DirRoot>,
    pub files: Vec<FileInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub file_type: FileType,
    pub content: FileContent,
}

#[derive(Serialize, Deserialize)]
pub enum FileType {
    Bin,
    Text,
}

#[derive(Serialize, Deserialize)]
pub enum FileContent {
    Text(String),
    Bin(Vec<u8>),
}

impl DirRoot {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            dirs: vec![],
            files: vec![],
        }
    }

    pub fn from_dir(path: String) -> Result<Self, std::io::Error> {
        get_dirs(String::new(), path)
    }

    pub fn save_as(self: &Self, path: &str) -> Result<(), String> {
        let mut f = match File::create(path) {
            Ok(it) => it,
            Err(err) => return Err(err.to_string()),
        };

        let buf = match bincode::serialize(&self) {
            Ok(it) => it,
            Err(err) => return Err(err.to_string()),
        };

        match f.write_all(&buf) {
            Ok(it) => it,
            Err(err) => return Err(err.to_string()),
        };
        return Ok(());
    }

    pub fn read_from(path: &str) -> Result<Self, String> {
        let mut buf: Vec<u8> = vec![];
        let mut f = match File::open(path) {
            Ok(it) => it,
            Err(err) => return Err(err.to_string()),
        };
        let _ = match f.read_to_end(&mut buf) {
            Ok(it) => it,
            Err(err) => return Err(err.to_string()),
        };
        Ok(match bincode::deserialize(&buf) {
            Ok(it) => it,
            Err(err) => return Err(err.to_string()),
        })
    }

    pub fn info(self: &Self) {
        println!("This profile will create dirs: ");
        fn print_dirs(root: &DirRoot, prefix: String) {
            for i in &root.dirs {
                println!("\t{}", prefix.clone() + &get_os_dir_sep() + &i.name);
                print_dirs(&i, prefix.clone() + &get_os_dir_sep() + &i.name);
            }
        }

        print_dirs(&self, ".".to_string());

        println!("And will write files: ");
        fn print_files(root: &DirRoot, prefix: String) {
            for i in &root.files {
                println!("\t{}", prefix.clone() + &get_os_dir_sep() + &i.name);
            }
            for i in &root.dirs {
                print_files(&i, prefix.clone() + &get_os_dir_sep() + &i.name);
            }
        }

        print_files(&self, ".".to_string());
    }

    // pub fn get_info(self: &Self) -> String {
    //     let mut ret = format!("This profile will create dirs: \r\n");
    //     fn print_dirs(root: &DirRoot, prefix: String) -> String {
    //         let mut ret = "".to_string();
    //         for i in &root.dirs {
    //             ret = format!(
    //                 "{}\t{}\r\n",
    //                 ret,
    //                 prefix.clone() + &get_os_dir_sep() + &i.name
    //             );
    //             ret = ret + &print_dirs(&i, prefix.clone() + &get_os_dir_sep() + &i.name);
    //         }
    //         ret
    //     }
    //
    //     ret = format!(
    //         "{}{}And will write files: \r\n",
    //         ret,
    //         print_dirs(&self, ".".to_string())
    //     );
    //     fn print_files(root: &DirRoot, prefix: String) -> String {
    //         let mut ret = "".to_string();
    //         for i in &root.files {
    //             ret = format!(
    //                 "{}\t{}\r\n",
    //                 ret,
    //                 prefix.clone() + &get_os_dir_sep() + &i.name
    //             );
    //         }
    //         for i in &root.dirs {
    //             ret = ret + &print_files(&i, root.name.clone() + &get_os_dir_sep() + &i.name);
    //         }
    //         ret
    //     }
    //
    //     ret = ret + &print_files(&self, ".".to_string());
    //     ret
    // }
}

impl FileInfo {
    pub fn new(name: &str, file_type: FileType, content: FileContent) -> Self {
        Self {
            name: String::from(name),
            file_type,
            content,
        }
    }

    pub fn write(self: &Self, path: &str) -> Result<usize, std::io::Error> {
        let mut f = File::create(path)?;
        let content = match &self.content {
            FileContent::Text(s) => s.as_bytes().to_vec(),
            FileContent::Bin(b) => b.to_vec(),
        };
        f.write(&content)
    }
}

// =============================================================================
// =============================================================================

fn get_dirs(name: String, prefix: String) -> Result<DirRoot, std::io::Error> {
    let mut ret = DirRoot {
        name: name.clone(),
        dirs: vec![],
        files: vec![],
    };

    for entry in fs::read_dir(prefix.clone() + &name)? {
        let (f_name, f_type, f_path) = {
            let entry = entry.unwrap();
            (
                entry.file_name().to_str().unwrap().to_string(),
                entry.file_type().unwrap(),
                entry.path().display().to_string(),
            )
        };

        if f_type.is_dir() {
            // TODO: May cause unfriendly operation.
            let next_root = get_dirs(f_name, prefix.clone() + &name + &get_os_dir_sep())?;
            ret.dirs.push(next_root);
        } else if f_type.is_file() {
            let mut buf: Vec<u8> = vec![];
            let _ = File::open(f_path)?.read_to_end(&mut buf);

            if buf.is_empty() {
                continue;
            }

            let f_info = match String::from_utf8(buf.clone()) {
                Ok(content) => FileInfo::new(&f_name, FileType::Text, FileContent::Text(content)),
                Err(_) => FileInfo::new(&f_name, FileType::Bin, FileContent::Bin(buf)),
            };

            ret.files.push(f_info);
        }
    }

    Ok(ret)
}

pub fn get_os_dir_sep() -> String {
    match OS {
        "windows" => "\\",
        _ => "/",
    }
    .to_string()
}
