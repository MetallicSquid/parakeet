use confy::ConfyError::DirectoryCreationFailed;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{DirEntry, metadata};
use std::path::PathBuf;
use std::{env, io};

#[derive(Serialize, Deserialize)]
struct PlumeConfig {
    models_path: PathBuf,
    public_path: PathBuf,
    source_path: PathBuf,
}

// Make an effort to find reasonable defaults for the needed paths
impl ::std::default::Default for PlumeConfig {
    fn default() -> Self {
        let mut models_path: PathBuf = PathBuf::new();
        let mut public_path: PathBuf = PathBuf::new();
        let mut source_path: PathBuf = PathBuf::new();

        let working_dir = env::current_dir().unwrap();
        let parent_dir = working_dir.parent().unwrap();

        let mut search_closure = |entry_result| -> Result<(), Box<dyn Error>> {
            let entry: DirEntry = entry_result?;
            if entry.metadata()?.is_dir() {
                match entry.file_name().to_str().unwrap() {
                    "models" => models_path = PathBuf::from(entry.path()),
                    "public" => public_path = PathBuf::from(entry.path()),
                    "source" => source_path = PathBuf::from(entry.path()),
                    _ => {}
                }
            }

            Ok(())
        };

        parent_dir.read_dir().unwrap().map(&mut search_closure);
        working_dir.read_dir().unwrap().map(&mut search_closure);

        Self {
            models_path,
            public_path,
            source_path,
        }
    }
}

// Sets up configuration for plume
pub fn config(models_path: PathBuf, public_path: PathBuf, source_path: PathBuf) -> Result<(), Box<dyn Error>> {
    // FIXME: Find a better way of determining if the paths exist
    metadata(&models_path)?;
    metadata(&public_path)?;
    metadata(&source_path)?;

    confy::store(
        "plume",
        PlumeConfig {
            models_path,
            public_path,
            source_path,
        },
    )?;

    Ok(())
}

// Wizard to set up configuration for plume
pub fn config_wizard() -> Result<(), Box<dyn Error>> {
    println!("***** Config Setup Wizard *****");
    println!("This wizard will set up the plume configuration. Enter your response or");
    println!("leave blank to select the (default).\n");

    let previous_config: PlumeConfig = confy::load("plume")?;

    let mut models_string: String = String::new();
    println!("Parakeet models path ({}): ", previous_config.models_path.to_str().unwrap());
    io::stdin().read_line(&mut models_string)?;
    let mut public_string: String = String::new();
    println!("Parakeet public path ({}): ", previous_config.public_path.to_str().unwrap());
    io::stdin().read_line(&mut public_string)?;
    let mut source_string: String = String::new();
    println!("Parakeet source path ({}): ", previous_config.source_path.to_str().unwrap());
    io::stdin().read_line(&mut source_string)?;

    config(
        PathBuf::from(models_string.trim()),
        PathBuf::from(public_string.trim()),
        PathBuf::from(source_string.trim()),
    )?;

    println!("\n***** All done! *****");

    Ok(())
}
