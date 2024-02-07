use std::env::consts::OS;

pub fn get_os_config_dir() -> String {
    match OS {
        "windows" => std::env::var("APPDATA").unwrap() + "\\workspace-configures\\",
        _ => std::env::var("HOME").unwrap() + "/.config/workspace_configures/",
    }
}

pub fn get_os_dir_sep() -> String {
    match OS {
        "windows" => "\\",
        _ => "/",
    }
    .to_string()
}

pub fn get_profile_path(profile_name: String) -> String {
    if profile_name.ends_with("bincode") {
        get_os_config_dir() + "profiles" + &get_os_dir_sep() + &profile_name
    } else {
        get_os_config_dir() + "profiles" + &get_os_dir_sep() + &profile_name + ".bincode"
    }
}

#[cfg(test)]
mod tests {}
