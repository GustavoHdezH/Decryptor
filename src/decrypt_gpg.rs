use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn decrypt_file(path: &Path, output_dir: &Path, passphrase: &str) -> Result<(), String> {
    let file_name_os = path
        .file_name()
        .ok_or_else(|| format!("Failed to get file name for {}", path.display()))?
        .to_os_string();
    let file_name = file_name_os.to_string_lossy();
    let decrypted_file_name = file_name.trim_end_matches(".gpg");
    let output_path = output_dir.join(decrypted_file_name);

    let mut gpg_command = Command::new("gpg")
        .arg("--verbose")
        .arg("--output")
        .arg(&output_path)
        .arg("--decrypt")
        .arg("--pinentry-mode")
        .arg("loopback")
        .arg("--passphrase-fd")
        .arg("0")
        .arg(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start gpg process for {}: {}", path.display(), e))?;

    if let Some(mut stdin) = gpg_command.stdin.take()
        && let Err(e) = stdin.write_all(passphrase.as_bytes())
    {
        return Err(format!(
            "Error typing passphrase in stdin for {}: {}",
            path.display(),
            e
        ));
    }

    let output = gpg_command.wait_with_output().map_err(|e| {
        format!(
            "Error waiting for gpg process for {}: {}",
            path.display(),
            e
        )
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "GPG failed for file {}: {}",
            path.display(),
            stderr
        ));
    }

    Ok(())
}
