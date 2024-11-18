use std::{ffi::OsString, io};

use winreg::{enums::HKEY_CURRENT_USER, RegKey};

pub fn get_settings_json_path_registry() -> io::Result<String> {
    let install_path = get_reg_value("SettingsJson");
    install_path
}

pub fn set_settings_json_path_registry() {
    let path_to_folder = get_install_path_registry();
    let mut path_to_settings_json = match path_to_folder {
        Ok(path) => path,
        Err(_) => return,
    };
    path_to_settings_json.push_str("\\settings.json");

    set_reg_value("SettingsJson", &OsString::from(path_to_settings_json));
}

pub fn get_install_path_registry() -> io::Result<String> {
    let install_path = get_reg_value("InstallDir");
    install_path
}

pub fn set_install_path_registry() {
    let doc_dir = dirs_next::document_dir();
    let mut path_to_documents = match doc_dir {
        Some(info) => info,
        None => return,
    };

    path_to_documents.push("local_wallpaper_viewer");
    set_reg_value("InstallDir", path_to_documents.as_mut_os_string());
}

fn get_reg_value(keyname: &str) -> io::Result<String> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"SOFTWARE\LocalWallpaperViewer";

    let key = hklm.open_subkey(path)?;
    let reg_value = key.get_value(keyname)?;
    Ok(reg_value)
}

fn set_reg_value(keyname: &str, keyvalue: &OsString) -> io::Result<bool> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"SOFTWARE\LocalWallpaperViewer";

    let (key, _disp) = hklm.create_subkey(&path)?;

    let key_result = key.set_value(keyname, keyvalue);
    match key_result {
        Ok(_) => Ok(true),
        Err(err) => panic!("{:?}", err),
    }
}
