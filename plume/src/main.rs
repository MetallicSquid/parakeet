// ***** Plume *****
// Tool for the management of models in parakeet.
// Commands:
//  * config        -> Sets up the plume configuration with the provided paths
//  * config-wizard -> Wizard to set up the plume configuration
//  * index         -> Traverses and indexes the models in the models directory

mod config;

use std::error::Error;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use structopt::StructOpt;
use chrono::{DateTime, NaiveDate};
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(StructOpt)]
#[structopt(name = "plume", about = "Tool for the management of models in parakeet")]
enum Commands {
    /// Configure plume with the relevant path information
    #[structopt(name = "config")]
    Config {
        /// Models directory path
        models_path: PathBuf,
        /// Public directory path
        public_path: PathBuf,
        /// Source directory path
        source_path: PathBuf
    },
    /// Wizard to configure plume with the relevant path information
    #[structopt(name = "config-wizard")]
    ConfigWizard {},
    /// Index the models directory and output an 'index.json' file
    #[structopt(name = "index")]
    Index {
        /// Models directory path
        models_path: PathBuf
    }
}

fn main() {
    match Commands::from_args() {
        Commands::Config { models_path, public_path, source_path } => {
            match config::config(models_path, public_path, source_path) {
                Ok(_) => println!("Successfully configured plume. Plume is now ready to use."),
                Err(error) => println!("Failed to configure plume: [{}]", error)
            }
        },
        Commands::ConfigWizard {} => {
            match config::config_wizard() {
                Ok(_) => {},
                Err(error) => println!("Failed to configure plume: [{}]", error)
            }
        },
        Commands::Index { models_path } => {
            let path_str = models_path.to_str().unwrap();
            match index(&models_path) {
                Ok(_) => println!("Successfully indexed `{}`. Outputted to `{}{}`", path_str, path_str, "index.json"),
                Err(error) => println!("Failed to index `{}`: [{}]", path_str , error)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Model {
    name: String,
    date: NaiveDate,
    description: String,
    image_path: PathBuf,
    scad_path: PathBuf
}

// Traverse the provided models directory and extract the relevant files
fn flatten_models_dir(path: &PathBuf, valid_model: bool) -> Result<Vec<(PathBuf, PathBuf, PathBuf)>, Box<dyn Error>> {
    let mut image_path: PathBuf = PathBuf::new();
    let mut scad_path: PathBuf = PathBuf::new();
    let mut info_path: PathBuf = PathBuf::new();
    let mut model_vec: Vec<(PathBuf, PathBuf, PathBuf)> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();
        if entry_path.is_dir() && !valid_model {
            let entry_contents = flatten_models_dir(&entry_path, true)?;
            model_vec.extend(entry_contents);
        } else if entry_path.extension().unwrap() == "jpg" && valid_model {
            image_path = entry_path;
        } else if entry_path.extension().unwrap() == "scad" && valid_model {
            scad_path = entry_path;
        } else if entry_path.extension().unwrap() == "json" && valid_model {
            info_path = entry_path;
        }
    }

    if valid_model {
        Ok(vec!((image_path, scad_path, info_path)))
    } else {
        Ok(model_vec)
    }
}

// Create an `index.json` file in the models directory linking to the relevant information
fn index(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let flattened_models = flatten_models_dir(path, false)?;
    let mut models: Vec<Model> = Vec::new();

    for entry in flattened_models {
        let info_string = fs::read_to_string(entry.2)?;
        let info_json : Value = serde_json::from_str(&info_string)?;
        models.push(
            Model {
                name: info_json["name"].as_str().unwrap().to_string(),
                date: NaiveDate::parse_from_str(info_json["date"].as_str().unwrap(),"%Y-%m-%d")?,
                description: info_json["description"].as_str().unwrap().to_string(),
                image_path: entry.0,
                scad_path: entry.1
            }
        );
    }

    let mut index_file = fs::File::create(path.join("index.json")).unwrap();
    writeln!(index_file, "{}",&serde_json::to_string(&models)?);

    Ok(())
}
