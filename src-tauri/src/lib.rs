use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use image_hasher::{HasherConfig, ImageHash};
use rayon::prelude::*;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use walkdir::WalkDir;
mod error;
struct AppData {
    hasher: image_hasher::Hasher,
}

type PotentialDuplicate = (String, String, u32);

const EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "webp"];

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_files(app: AppHandle) -> Result<Vec<PotentialDuplicate>, crate::error::Error> {
    let start = time::OffsetDateTime::now_utc();
    let folders = app.dialog().file().blocking_pick_folders();
    if let Some(paths) = folders {
        let files_with_hash = Arc::new(Mutex::new(HashMap::new()));
        // let mut file_count = 0;
        let mut files = Vec::new();
        for path in paths {
            let path = path.into_path().expect("Convert FilePath into PathBuf");

            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                // file_count += WalkDir::new(&path).into_iter().count();

                if let Some(ext) = entry.path().extension() {
                    if EXTENSIONS.contains(&ext.to_str().unwrap_or("")) {
                        files.push(entry.path().to_path_buf());
                    }
                }
            }
        }
        files.par_iter().for_each(|file| {
            match hash_file(app.clone(), file.as_path()) {
                Ok(hash) => {
                    let mut files = files_with_hash.lock().unwrap();
                    files.insert(file.to_string_lossy().to_string(), hash);
                }
                Err(e) => eprintln!("Failed to hash file: {:?}", e),
            };
        });
        let duplicates = compare_hashes(&files_with_hash.lock().unwrap());
        println!("{}", time::OffsetDateTime::now_utc() - start);
        Ok(duplicates)
    } else {
        Ok(Vec::new())
    }
}

fn hash_file(app: AppHandle, path: &Path) -> Result<ImageHash, image::ImageError> {
    let hasher = &app.state::<AppData>().hasher;
    Ok(hasher.hash_image(&image::open(path)?))
}

fn compare_hashes(
    file_hashes: &HashMap<String, image_hasher::ImageHash>,
) -> Vec<PotentialDuplicate> {
    let filenames: Vec<&String> = file_hashes.keys().collect();
    let mut duplicates = Vec::new();
    for i in 0..filenames.len() {
        for j in i + 1..filenames.len() {
            let filename1 = filenames[i];
            let filename2 = filenames[j];

            let hash1 = file_hashes.get(filename1).unwrap();
            let hash2 = file_hashes.get(filename2).unwrap();

            let distance = hash1.dist(hash2);

            if distance <= 4 {
                println!(
                    "Potential duplicates found: {} and {} (Hamming Distance: {})",
                    filename1, filename2, distance
                );
                duplicates.push((filename1.to_string(), filename2.to_string(), distance));
            }
        }
    }
    duplicates
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
