use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{DirEntry, metadata, canonicalize};
use std::path::PathBuf;
use std::env;

#[derive(Serialize, Deserialize)]
struct ParakeetConfig {
    models_path: PathBuf,
    build_path: PathBuf,
}

impl ::std::default::Default for ParakeetConfig {
    fn default() -> Self {
        Self {
            models_path: PathBuf::new(),
            build_path: PathBuf::new(),
        }
    }
}

// Sets up configuration for plume
pub fn config(models_path: PathBuf, build_path :PathBuf) -> Result<(), Box<dyn Error>> {
    metadata(&models_path)?;
    metadata(&build_path)?;

    confy::store(
        "parakeet",
        ParakeetConfig {
            models_path: canonicalize(models_path)?,
            build_path: canonicalize(build_path)?,
        },
    )?;

    Ok(())
}

// Loads the 'models' and 'build' paths from the config
pub fn get_paths() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let config: ParakeetConfig = confy::load("parakeet")?;

    Ok(vec![config.models_path, config.build_path])
}
