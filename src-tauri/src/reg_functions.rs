use std::io;

use winreg::{enums::HKEY_CURRENT_USER, RegKey};

pub fn get_config_path_registry() -> io::Result<String> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"SOFTWARE\LocalWallpaperViewer\LocalWallpaperViewer";

    let key = hklm.open_subkey(path)?;

    let install_path_string: String = key.get_value("InstallDir")?;

    Ok(install_path_string)
}

pub fn save_config_path_registry() {
    let doc_dir = dirs_next::document_dir();
    let mut path_to_documents = match doc_dir {
        Some(info) => info,
        None => return,
    };

    path_to_documents.push("local_wallpaper_viewer");

    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"SOFTWARE\LocalWallpaperViewer\LocalWallpaperViewer";

    let key = match hklm.create_subkey(&path) {
        Ok(key) => key.0,
        Err(_) => return,
    };

    let key_result = key.set_value("InstallDir", &path_to_documents.as_os_str());
    match key_result {
        Ok(_) => println!("Made a new key for LocalWallpaperViewer"),
        Err(_) => println!("Failed to make a new key for LocalWallpaperViewer"),
    }
}
