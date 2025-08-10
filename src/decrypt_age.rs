use std::path::Path;
use std::process::{Command, Stdio};

pub fn decrypt_file(path: &Path, output_dir: &Path, identity_file: &Path) -> Result<(), String> {
    let file_name = path.file_name().unwrap().to_string_lossy();
    let decrypted_file_name = file_name.trim_end_matches(".age");
    let output_path = output_dir.join(decrypted_file_name);

    let output = Command::new("age")
        .arg("--decrypt")
        .arg("--identity")
        .arg(identity_file)
        .arg("--output")
        .arg(&output_path)
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to start age process for {}: {}", path.display(), e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AGE failed for file {}: {}", path.display(), stderr));
    }

    Ok(())
}