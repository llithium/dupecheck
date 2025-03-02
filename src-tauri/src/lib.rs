use std::path::Path;

use image_hasher::{HasherConfig, ImageHash};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use walkdir::WalkDir;

struct AppData {
    hasher: image_hasher::Hasher,
}

const EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "webp"];

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_files(app: AppHandle) -> String {
    app.dialog().file().pick_folders(move |file_path| {
        if let Some(paths) = file_path {
            for path in paths {
                let path = path.into_path().expect("Convert selected file into path");
                for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                    if let Some(ext) = entry.path().extension() {
                        if EXTENSIONS.contains(&ext.to_str().unwrap_or("")) {
                            println!("{:#?}", hash_file(app.clone(), entry.path()).unwrap());
                        }
                    }
                }
            }
        } else {
            eprintln!("No folder selected");
        }
    });
    format!("Hello, You've been greeted from Rust!")
}

fn hash_file(app: AppHandle, path: &Path) -> Result<ImageHash, image::ImageError> {
    let hasher = &app.state::<AppData>().hasher;
    Ok(hasher.hash_image(&image::open(path)?))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppData {
                hasher: HasherConfig::new()
                    .hash_alg(image_hasher::HashAlg::DoubleGradient)
                    .to_hasher(),
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_files, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
