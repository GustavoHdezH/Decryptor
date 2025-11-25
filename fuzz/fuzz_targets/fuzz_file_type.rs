#![no_main]

use libfuzzer_sys::fuzz_target;
use std::path::PathBuf;
use decrypt_files::file_type::FileType;

fuzz_target!(|data: &[u8]| {
    // Convertir los bytes aleatorios a string
    if let Ok(s) = std::str::from_utf8(data) {
        // Crear un PathBuf desde el string
        let path = PathBuf::from(s);
        
        // Llamar a from_path - no debe causar panic
        let result = FileType::from_path(&path);
        
        // Si retorna Some, debe ser una variante válida
        if let Some(file_type) = result {
            // Verificar que extension() retorna algo válido
            let ext = file_type.extension();
            assert!(ext == "gpg" || ext == "age");
            
            // Verificar que Display funciona
            let _ = format!("{}", file_type);
        }
    }
});
