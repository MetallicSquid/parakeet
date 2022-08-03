use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{DirEntry, metadata, canonicalize};
use std::path::PathBuf;
use std::env;

#[derive(Serialize, Deserialize)]
struct PathConfig {
    models_path: PathBuf,
    public_path: PathBuf,
    source_path: PathBuf,
}

impl ::std::default::Default for PathConfig {
    fn default() -> Self {
        Self {
            models_path: PathBuf::new(),
            public_path: PathBuf::new(),
            source_path: PathBuf::new(),
        }
    }
}

// Sets up configuration for plume
pub fn config(models_path: PathBuf, public_path: PathBuf, source_path: PathBuf) -> Result<(), Box<dyn Error>> {
    metadata(&models_path)?;
    metadata(&public_path)?;
    metadata(&source_path)?;

    confy::store(
        "parakeet",
        PathConfig {
            models_path: canonicalize(models_path)?,
            public_path: canonicalize(public_path)?,
            source_path: canonicalize(source_path)?,
        },
    )?;

    Ok(())
}

// Loads the 'models', 'public' and 'source' paths from the config
pub fn get_paths() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let config: PathConfig = confy::load("parakeet")?;

    Ok(vec![config.models_path, config.public_path, config.source_path])
}
