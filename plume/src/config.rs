use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{DirEntry, metadata, canonicalize};
use std::path::PathBuf;
use std::env;

#[derive(Serialize, Deserialize)]
struct ParakeetConfig {
    models_path: PathBuf,
    build_path: PathBuf,
    database_path: PathBuf,
    model_limit: i64
}

impl ::std::default::Default for ParakeetConfig {
    fn default() -> Self {
        Self {
            models_path: PathBuf::new(),
            build_path: PathBuf::new(),
            database_path: PathBuf::new(),
            model_limit: 100
        }
    }
}

// Sets up configuration for plume
pub fn config(models_path: PathBuf, build_path: PathBuf, database_path: PathBuf, model_limit: i64) -> Result<(), Box<dyn Error>> {
    metadata(&models_path)?;
    metadata(&build_path)?;
    metadata(&database_path)?;

    confy::store(
        "parakeet",
        None,
        ParakeetConfig {
            models_path: canonicalize(models_path)?,
            build_path: canonicalize(build_path)?,
            database_path: canonicalize(database_path)?,
            model_limit
        },
    )?;

    Ok(())
}

// Loads the 'models' and 'build' paths from the config
pub fn get_paths() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let config: ParakeetConfig = confy::load("parakeet", None)?;

    Ok(vec![config.models_path, config.build_path, config.database_path])
}
