use std::fs;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::decrypt_age;
use crate::decrypt_gpg;
use crate::file_type::FileType;

pub fn decrypt_massive_file(
    filepath: &Path,
    output_dir: &Path,
    passphrase_file: &Option<PathBuf>,
    identity_file: &Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !filepath.is_dir() {
        return Err(format!("Filepath '{}' is not a directory.", filepath.display()).into());
    }

    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    let passphrase_content = if let Some(path) = passphrase_file {
        fs::read_to_string(path).map_err(|e| {
            format!(
                "Error reading passphrase file '{:?}': {}",
                path.display(),
                e
            )
        })?
    } else {
        "".to_string()
    };

    let files_to_decrypt: Vec<(PathBuf, FileType)> = fs::read_dir(filepath)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                let file_type = FileType::from_path(&path)?;
                return Some((path, file_type));
            }
            None
        })
        .collect();

    if files_to_decrypt.is_empty() {
        println!(
            "No .gpg or .age files were found for decryption in '{}'.",
            filepath.display()
        );
        return Ok(());
    }

    println!(
        "Found {} files to decrypt. Parallel processing...",
        files_to_decrypt.len()
    );

    files_to_decrypt.par_iter().for_each(|(path, file_type)| {
        let result = match file_type {
            FileType::Gpg => {
                if passphrase_file.is_some() {
                    let passphrase_for_thread = passphrase_content.clone();
                    decrypt_gpg::decrypt_file(path, output_dir, &passphrase_for_thread)
                } else {
                    Err("La variable de entorno 'PASSPHRASE_FILE' no está definida para archivos .gpg.".to_string())
                }
            }
            FileType::Age => {
                if let Some(identity) = identity_file {
                    decrypt_age::decrypt_file(path, output_dir, identity)
                } else {
                    Err("La variable de entorno 'IDENTITY_FILE' no está definida para archivos .age.".to_string())
                }
            }
        };

        match result {
            Ok(_) => println!("File decrypted successfully: {}", path.display()),
            Err(e) => eprintln!("Failed to decrypt file {}: {}", path.display(), e),
        }
    });

    Ok(())
}
