use std::{fs, io, path::Path, process::Command};

use serde::{Deserialize, Serialize};
use serde_json;

use walkdir::WalkDir;

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

#[derive(Serialize, Deserialize, Debug)]
struct ProjectJSON {
    title: String,
    preview: String,
    file: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WallpaperInfo {
    project_json: ProjectJSON,
    project_id: String,
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
fn get_all_wallpapers() -> String {
    // iterate over all folders
    let preferences_result = get_user_preferences_as_struct();
    let preferences = match preferences_result {
        Ok(info) => info,
        Err(_) => return String::from("{}"),
    };

    let mut all_info: Vec<WallpaperInfo> = Vec::new();
    for entry in WalkDir::new(&preferences.wallpaper_folder_location) {
        let entry = entry.unwrap();
        if entry.file_name() == "project.json" {
            let folder_name = entry
                .path()
                .strip_prefix(&preferences.wallpaper_folder_location)
                .unwrap()
                .parent()
                .unwrap()
                .to_string_lossy();

            let data = fs::read_to_string(entry.path()).unwrap();
            let info_as_json: serde_json::Result<ProjectJSON> = serde_json::from_str(&data);
            match info_as_json {
                Ok(data) => {
                    if data.r#type.to_lowercase() == "video" {
                        println!("{:?}", data);

                        let wallpaper_info = WallpaperInfo {
                            project_json: data,
                            project_id: String::from(folder_name),
                        };

                        all_info.push(wallpaper_info);
                    }
                }
                Err(_) => println!("JSON file didn't have file specified."),
            };
        }
    }

    return serde_json::to_string(&all_info).unwrap();
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
fn set_wallpaper_directory(dir: String) -> bool {
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

#[tauri::command]
fn set_sidebar_status(status: bool) -> bool {
    let preferences = get_user_preferences_as_struct();
    let mut preferences_object = match preferences {
        Ok(info) => info,
        Err(_) => return false,
    };

    preferences_object.is_sidebar_enabled = status;
    let status = save_user_preferences(preferences_object);
    match status {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

#[tauri::command]
fn play_video(video_path: String) {
    Command::new("cmd")
        .args(["/C", &video_path])
        .output()
        .expect("failed to execute process");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            set_wallpaper_directory,
            set_sidebar_status,
            first_time_setup,
            get_user_preferences,
            get_all_wallpapers,
            play_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
