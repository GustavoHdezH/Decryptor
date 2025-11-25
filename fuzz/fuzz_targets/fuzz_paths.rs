#![no_main]

use libfuzzer_sys::fuzz_target;
use std::path::PathBuf;

fuzz_target!(|data: &[u8]| {
    // Convertir bytes a string
    if let Ok(s) = std::str::from_utf8(data) {
        // Probar construcción de PathBuf
        let path = PathBuf::from(s);
        
        // Verificar operaciones comunes que no deben causar panic
        let _ = path.exists();
        let _ = path.is_file();
        let _ = path.is_dir();
        let _ = path.extension();
        let _ = path.file_name();
        let _ = path.parent();
        
        // Probar conversión a string
        let _ = path.to_str();
        let _ = path.display();
        
        // Probar componentes del path
        for _ in path.components() {
            // Iterar sobre componentes no debe causar panic
        }
        
        // Si tiene extensión, extraerla de forma segura
        if let Some(ext) = path.extension() {
            let _ = ext.to_str();
        }
    }
});
