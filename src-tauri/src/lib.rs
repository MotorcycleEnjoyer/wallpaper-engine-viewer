use std::fs::{self, DirEntry};
use std::path::Path;
use std::{any, io};

use serde::{Deserialize, Serialize};
use serde_json::Result;

use winreg::enums::*;
use winreg::RegKey;

use dirs_next;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_config_path_registry() -> io::Result<String> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"SOFTWARE\LocalWallpaperViewer\LocalWallpaperViewer";

    let key = hklm.open_subkey(path)?;

    let install_path_string: String = key.get_value("InstallDir")?;

    Ok(install_path_string)
}

fn save_config_path_registry() {
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
        match get_config_path_registry() {
            Ok(_) => println!("Exists"),
            Err(_) => save_config_path_registry(),
        }
    } else {
        make_config_folder();
        save_config_path_registry();
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
        .invoke_handler(tauri::generate_handler![
            greet,
            store_wallpaper_directory,
            button,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
