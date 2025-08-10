use std::fs;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::decrypt_gpg;
use crate::decrypt_age;

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
        fs::read_to_string(path)
            .map_err(|e| format!("Error reading passphrase file '{:?}': {}", path.display(), e))?
    } else {
        "".to_string()
    };

    let files_to_decrypt: Vec<PathBuf> = fs::read_dir(filepath)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension()?.to_str()?;
                if ext == "gpg" || ext == "age" {
                    return Some(path);
                }
            }
            None
        })
        .collect();

    if files_to_decrypt.is_empty() {
        println!("No .gpg or .age files were found for decryption in '{}'.", filepath.display());
        return Ok(());
    }

    println!("Found {} files to decrypt. Parallel processing...", files_to_decrypt.len());

    files_to_decrypt.par_iter().for_each(|path| {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let result = match ext {
            "gpg" => {
                if passphrase_file.is_some() {
                    let passphrase_for_thread = passphrase_content.clone();
                    decrypt_gpg::decrypt_file(path, output_dir, &passphrase_for_thread)
                } else {
                    Err("La variable de entorno 'PASSPHRASE_FILE' no está definida para archivos .gpg.".to_string())
                }
            }
            "age" => {
                if let Some(identity) = identity_file {
                    decrypt_age::decrypt_file(path, output_dir, identity)
                } else {
                    Err("La variable de entorno 'IDENTITY_FILE' no está definida para archivos .age.".to_string())
                }
            }
            _ => Err(format!("Unsupported file extension: {}", ext)),
        };

        match result {
            Ok(_) => println!("File decrypted successfully: {}", path.display()),
            Err(e) => eprintln!("Failed to decrypt file {}: {}", path.display(), e),
        }
    });

    Ok(())
}