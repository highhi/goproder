use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tauri::{command, InvokeError};

#[derive(Debug, Serialize)]
pub struct CustomError {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenamedFile {
    old_path: String,
    new_path: String,
    old_name: String,
    new_name: String,
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError {
            message: error.to_string(),
        }
    }
}

fn sort_files_by_creation_date(files: &mut Vec<String>) -> std::io::Result<()> {
    files.sort_by(|a, b| {
        let meta_a = fs::metadata(a).expect("Unable to read file metadata");
        let meta_b = fs::metadata(b).expect("Unable to read file metadata");

        let created_a = meta_a.created().expect("Unable to read file creation date");
        let created_b = meta_b.created().expect("Unable to read file creation date");

        created_a.cmp(&created_b)
    });

    Ok(())
}

fn generate_new_file_names(files: &[String]) -> Result<Vec<RenamedFile>, CustomError> {
    let mut renamed_files = Vec::new();

    for (index, file) in files.iter().enumerate() {
        if !file.to_lowercase().ends_with(".mp4") {
            continue;
        }

        let old_path = Path::new(file);
        let old_name = old_path
            .file_name()
            .ok_or_else(|| CustomError {
                message: "Failed to get the old file name.".to_string(),
            })?
            .to_string_lossy()
            .to_string();

        let file_ext = old_path
            .extension()
            .ok_or_else(|| CustomError {
                message: "Failed to get file extension".to_string(),
            })?
            .to_string_lossy()
            .to_string();

        let meta = fs::metadata(file)?;
        let created = meta.created()?;
        let created_date: DateTime<Utc> = created.into(); // SystemTime を DateTime に変換
        let formatted_created_date = created_date.format("%Y%m%d");

        let new_name = format!("{}-{}.{}", formatted_created_date, index + 1, file_ext);
        let new_path = old_path
            .parent()
            .ok_or_else(|| CustomError {
                message: "Failed to find the parent directory of the file.".to_string(),
            })?
            .join(&new_name);

        renamed_files.push(RenamedFile {
            old_path: old_path.to_string_lossy().to_string(),
            new_path: new_path.to_string_lossy().to_string(),
            old_name,
            new_name,
        });
    }

    Ok(renamed_files)
}

fn rename_files(renamed_files: &[RenamedFile]) -> Result<(), CustomError> {
    for renamed_file in renamed_files {
        let old_path = std::path::Path::new(&renamed_file.old_path);
        let new_path = std::path::Path::new(&renamed_file.new_path);

        if let Err(e) = fs::rename(old_path, new_path) {
            return Err(CustomError {
                message: format!("Failed to rename file: {}", e),
            });
        }
    }

    Ok(())
}

#[command]
pub fn handle_drag_and_drop_files(paths: Vec<String>) -> Result<Vec<RenamedFile>, CustomError> {
    let mut files = paths;
    if let Err(e) = sort_files_by_creation_date(&mut files) {
        eprintln!("Error sorting files by creation date: {}", e);
    }

    let renamed_files = generate_new_file_names(&files)?;

    Ok(renamed_files)
}

#[command]
pub async fn handle_rename_files(renamed_files: Vec<RenamedFile>) -> Result<(), InvokeError> {
    rename_files(&renamed_files).map_err(InvokeError::from)
}
