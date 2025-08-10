use std::env;
use std::path::PathBuf;
use dotenv::dotenv;
use colored::Colorize;

//  Structure to store the loaded configuration.
pub struct AppConfig {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub passphrase_file: Option<PathBuf>,
    pub identity_file: Option<PathBuf>,
    pub threads: usize,
}

//  Function to load configuration from .env file
pub fn load_config() -> Result<AppConfig, String> {
    // Step 1. Load the .env file.
    match dotenv() {
        Ok(path) => {
            print!("\n-----------------------------------------------------------------------------------------------");
            print!("{}", format!("\nEnvironment variables loaded from: {:?}", path).yellow());
        }
        Err(e) => {
            return Err(format!("Critical error: The .env file could not be loaded. Details: {}", e));
        }
    }

    // Step 2. Read specific variables.
    let input_dir_str = env::var("INPUT_DIR")
        .map_err(|e| format!("Error: The variable 'INPUT_DIR' not found in the .env file. Details: {} ", e))?;
    let output_dir_str = env::var("OUTPUT_DIR")
        .map_err(|e| format!("Error: The variable 'OUTPUT_DIR' not found in the .env file. Details: {} ", e))?;

    let passphrase_file_str = env::var("PASSPHRASE_FILE").ok();
    let passphrase_file = passphrase_file_str.map(PathBuf::from);

    let identity_file_str = env::var("IDENTITY_FILE").ok();
    let identity_file = identity_file_str.map(PathBuf::from);

    let threads_str = env::var("SERVER_THREADS").unwrap_or_else(|_| "0".to_string());
    let threads = threads_str
        .parse::<usize>()
        .map_err(|e| format!("Error parsing THREADS: {}", e))?;

    // Convert strings to PathBuf.
    let input_dir = PathBuf::from(input_dir_str);
    let output_dir = PathBuf::from(output_dir_str);

    // Step 3. Validate the folders and files.
    if !input_dir.is_dir() {
        return Err(format!("Invalid Configuration: The input dir {:?} define in .env does not exists\n", input_dir));
    }

    if !output_dir.is_dir() {
        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            return Err(format!("The output dir '{:?}' define in .env does not exists. Trying create {:?}\n", output_dir, e));
        }
        println!("Output Directory {:?} created", output_dir);
    }

    // Validate the files.
    if let Some(ref path) = passphrase_file {
        if !path.is_file() {
            return Err(format!("The passphrase file '{:?}' defined in .env does not exist.", path));
        }
    }
    // Validate the files.
    if let Some(ref path) = identity_file {
        if !path.is_file() {
            return Err(format!("The identity file '{:?}' defined in .env does not exist.", path));
        }
    }

    Ok(AppConfig { input_dir, output_dir, passphrase_file, identity_file, threads })
}