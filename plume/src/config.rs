use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{DirEntry, metadata};
use std::path::PathBuf;
use std::env;

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

// Loads the 'models', 'public' and 'source' paths from the config
pub fn get_paths() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let config: PlumeConfig = confy::load("plume")?;

    Ok(vec![config.models_path, config.public_path, config.source_path])
}
