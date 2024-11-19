use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use serde_json;

use dirs_next;

mod reg_functions;
use reg_functions::{
    get_install_path_registry, get_settings_json_path_registry, set_install_path_registry,
    set_settings_json_path_registry,
};

#[derive(Serialize, Deserialize, Debug)]
struct UserPreferences {
    is_sidebar_enabled: bool,
    wallpaper_folder_location: String,
}

fn config_folder_exists() -> bool {
    let doc_dir = dirs_next::document_dir();
    let mut path_to_documents = match doc_dir {
        Some(info) => info,
        None => return false,
    };

    path_to_documents.push("local_wallpaper_viewer");
    if path_to_documents.exists() {
        return true;
    } else {
        return false;
    }
}

fn set_default_preferences() {
    let sample_preferences = UserPreferences {
        is_sidebar_enabled: false,
        wallpaper_folder_location: String::from("TestLocation"),
    };
    save_user_preferences(sample_preferences);
}

fn save_user_preferences(preferences: UserPreferences) -> io::Result<bool> {
    let str = serde_json::to_string(&preferences);

    let path_string = get_settings_json_path_registry()?;
    let new_path = Path::new(&path_string);

    let result = fs::write(new_path, str.unwrap());

    match result {
        Ok(_) => return Ok(true),
        Err(error) => return Err(error),
    }
}

fn make_config_folder() {
    let doc_dir = dirs_next::document_dir();
    let mut path_to_documents = match doc_dir {
        Some(info) => info,
        None => return,
    };

    path_to_documents.push("local_wallpaper_viewer");
    let new_path = fs::create_dir(path_to_documents);
    match new_path {
        Ok(_) => println!("Successfully made the new directory"),
        Err(error) => println!("{:?}", error),
    }
}

#[tauri::command]
fn first_time_setup() {
    if config_folder_exists() {
        match get_install_path_registry() {
            Ok(_) => println!("Exists"),
            Err(_) => set_install_path_registry(),
        }
    } else {
        make_config_folder();
        set_install_path_registry();
        set_settings_json_path_registry();
        set_default_preferences()
    }
}

fn get_user_preferences_as_struct() -> io::Result<UserPreferences> {
    let settings_string = get_settings_json_path_registry()?;
    let settings_path = Path::new(&settings_string);

    let info = fs::read_to_string(settings_path)?;
    let info_for_rust = serde_json::from_str(&info)?;
    Ok(info_for_rust)
}

fn get_user_preferences_as_string() -> io::Result<String> {
    let settings_string = get_settings_json_path_registry()?;
    let settings_path = Path::new(&settings_string);

    let info = fs::read_to_string(settings_path)?;
    Ok(info)
}

#[tauri::command]
fn get_user_preferences() -> String {
    let preferences = get_user_preferences_as_string();
    match preferences {
        Ok(info) => return info,
        Err(err) => return err.to_string(),
    }
}

#[tauri::command]
fn store_wallpaper_directory(dir: String) -> bool {
    println!("{:?}", dir);

    let preferences = get_user_preferences_as_struct();
    let mut preferences_object = match preferences {
        Ok(info) => info,
        Err(_) => return false,
    };

    preferences_object.wallpaper_folder_location = dir;
    let status = save_user_preferences(preferences_object);
    match status {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            store_wallpaper_directory,
            first_time_setup,
            get_user_preferences,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
