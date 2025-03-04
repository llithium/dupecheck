use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use image::GenericImageView;
use image_hasher::{HasherConfig, ImageHash};
use rayon::prelude::*;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::DialogExt;
use walkdir::WalkDir;
mod error;
struct AppData {
    hasher: image_hasher::Hasher,
}
#[derive(Serialize)]
struct PotentialDuplicate {
    filename1: String,
    filename2: String,

    file_path1: String,
    file_path2: String,
    distance: u32,
    size1: u64,
    size2: u64,
    resolution1: (u32, u32),
    resolution2: (u32, u32),
    format1: String,
    format2: String,
}

const EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "webp"];

#[tauri::command]
async fn open_files(app: AppHandle) -> Result<Vec<PotentialDuplicate>, crate::error::Error> {
    let folders = app.dialog().file().blocking_pick_folders();
    let start = time::OffsetDateTime::now_utc();
    if let Some(paths) = folders {
        app.emit("hashing-started", "").unwrap();
        let files_with_hash = Arc::new(Mutex::new(HashMap::new()));
        let mut file_count = 0;
        let mut files = Vec::new();
        for path in paths {
            let path = path.into_path().expect("Convert FilePath into PathBuf");
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if let Some(ext) = entry.path().extension() {
                    if EXTENSIONS.contains(&ext.to_str().unwrap_or("")) {
                        files.push(entry.path().to_path_buf());
                        file_count += 1;
                    }
                }
            }
        }
        app.emit("total-files", file_count).unwrap();
        files.par_iter().for_each(|file| {
            match hash_file(&app, file.as_path()) {
                Ok(hash) => {
                    files_with_hash
                        .lock()
                        .unwrap()
                        .insert(file.to_string_lossy().to_string(), hash);
                    app.emit("current-file-count", 1).unwrap();
                }
                Err(e) => eprintln!("Failed to hash file: {:?}", e),
            };
        });
        let duplicates = compare_hashes(&app, &files_with_hash.lock().unwrap());
        println!("{}", time::OffsetDateTime::now_utc() - start);
        Ok(duplicates)
    } else {
        Ok(Vec::new())
    }
}

fn hash_file(app: &AppHandle, path: &Path) -> Result<ImageHash, image::ImageError> {
    let hasher = &app.state::<AppData>().hasher;
    Ok(hasher.hash_image(&image::open(path)?))
}

fn compare_hashes(
    app: &AppHandle,
    file_hashes: &HashMap<String, image_hasher::ImageHash>,
) -> Vec<PotentialDuplicate> {
    app.emit("comparing-started", "").unwrap();
    let file_paths: Vec<&String> = file_hashes.keys().collect();
    app.emit("total-files", file_paths.len()).unwrap();
    let mut duplicates = Vec::new();
    for i in 0..file_paths.len() {
        for j in i + 1..file_paths.len() {
            let file_path1 = file_paths[i];
            let file_path2 = file_paths[j];

            let hash1 = file_hashes.get(file_path1).unwrap();
            let hash2 = file_hashes.get(file_path2).unwrap();

            let distance = hash1.dist(hash2);

            if distance <= 9 {
                let pathuf1 = PathBuf::from(file_path1);
                let pathbuf2 = PathBuf::from(file_path2);

                let (filename1, size1, resolution1, format1) = get_file_info(&pathuf1);
                let (filename2, size2, resolution2, format2) = get_file_info(&pathbuf2);

                println!(
                    "Potential duplicates found: {} and {} (Hamming Distance: {})",
                    file_path1, file_path2, distance
                );
                duplicates.push(PotentialDuplicate {
                    filename1,
                    filename2,
                    file_path1: file_path1.to_string(),
                    file_path2: file_path2.to_string(),
                    distance,
                    size1,
                    size2,
                    resolution1,
                    resolution2,
                    format1,
                    format2,
                });
            }
        }
        app.emit("current-file-count", 1).unwrap();
    }
    duplicates
}

#[tauri::command]
fn delete_file(path: &str) -> Result<(), crate::error::Error> {
    trash::delete(path)?;
    Ok(())
}

fn get_file_info(file_path: &Path) -> (String, u64, (u32, u32), String) {
    let filename = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let size = fs::metadata(file_path).unwrap().len();

    let img = image::open(file_path).unwrap();
    let (width, height) = img.dimensions();
    let format = file_path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    (filename, size, (width, height), format)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppData {
                hasher: HasherConfig::new()
                    .hash_alg(image_hasher::HashAlg::DoubleGradient)
                    .hash_size(12, 12)
                    .to_hasher(),
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_files, delete_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
