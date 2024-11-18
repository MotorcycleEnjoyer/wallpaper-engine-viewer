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

fn save_user_preferences() -> io::Result<bool> {
    let sample_preferences = UserPreferences {
        is_sidebar_enabled: false,
        wallpaper_folder_location: String::from("TestLocation"),
    };

    let str = serde_json::to_string(&sample_preferences);

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
fn button() {
    if config_folder_exists() {
        match get_install_path_registry() {
            Ok(_) => println!("Exists"),
            Err(_) => set_install_path_registry(),
        }
    } else {
        make_config_folder();
        set_install_path_registry();
        set_settings_json_path_registry();
    }
    let saved_user_preferences = save_user_preferences();
    match saved_user_preferences {
        Ok(_) => println!("Saved user preferences"),
        Err(error) => println!("{}", error),
    }
}

#[tauri::command]
fn store_wallpaper_directory(dir: String) -> bool {
    println!("{:?}", dir);
    return true;
}

#[derive(Serialize, Deserialize)]
struct Project {
    file: String,
    preview: String,
    title: String,
}

fn visit_dirs(dir: &Path) -> io::Result<Vec<String>> {
    let mut data = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path)?;
            } else if path.file_name().unwrap() == "project.json" {
                // println!("Found project.json in: {:?}", path);
                let content = fs::read_to_string(path).unwrap();
                let info: Project = serde_json::from_str(&content).unwrap();
                println!("{}", info.title);
                data.push(content.to_string());
            }
        }
    }
    Ok(data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![store_wallpaper_directory, button,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
