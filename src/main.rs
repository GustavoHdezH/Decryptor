use std::error::Error;

mod config;
mod decrypt_age;
mod decrypt_gpg;
mod decryptor;
mod file_type;
use rayon::ThreadPoolBuilder;

use colored::Colorize;
use std::fs;
use std::io::{self};

fn header() -> io::Result<()> {
    let content = fs::read_to_string("header.txt")?;
    print!("{}", content.green());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = header();
    let app_config = match config::load_config() {
        Ok(cfg) => {
            println!(
                "{}",
                format!("\nInput folder:       {:?}", cfg.input_dir).yellow()
            );
            println!(
                "{}",
                format!("Output folder:      {:?}", cfg.output_dir).yellow()
            );
            println!(
                "{}",
                format!("Passphrase file:    {:?}", cfg.passphrase_file).yellow()
            );
            println!(
                "{}",
                format!("Identity file:      {:?}", cfg.identity_file).yellow()
            );
            println!(
                "{}",
                format!("Configured threads: {}", cfg.threads).yellow()
            );
            print!(
                "-----------------------------------------------------------------------------------------------"
            );
            cfg
        }
        Err(e) => {
            eprintln!("{}", format!("Error loading configuration: {}\n", e).red());
            std::process::exit(1);
        }
    };

    if app_config.threads > 0 {
        match ThreadPoolBuilder::new()
            .num_threads(app_config.threads)
            .build_global()
        {
            Ok(_) => println!(
                "\nPool of Rayon threads set to {} threads",
                app_config.threads
            ),
            Err(e) => eprintln!(
                "Warning: Rayon thread pool could not be configured: {}.  The default value will be used.",
                e
            ),
        }
    } else {
        println!("\nRayon's default number of threads (based on CPU cores) will be used.");
    }

    let filepath = &app_config.input_dir;
    let output_dir = &app_config.output_dir;
    let passphrase_file = &app_config.passphrase_file;
    let identity_file = &app_config.identity_file;

    decryptor::decrypt_massive_file(filepath, output_dir, passphrase_file, identity_file)?;

    println!("\nDecryption process completed.");

    Ok(())
}
