// ***** Plume *****
// Tool for the management of models in parakeet.
// Commands:
//  * config        -> Sets up the plume configuration with the provided paths
//  * index         -> Traverses and indexes the models in the models directory
//  * dist          -> Distributes the indexed files to their relevant directories

// TODO: Implement a distribution function that copies the files to their required directories
// TODO: Make some kind of git hook system that checks commits to the models directory

mod config;
mod parse;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "plume",
    about = "Tool for the management of models in parakeet"
)]
enum Commands {
    /// Configure plume with the relevant path information
    #[structopt(name = "config")]
    Config {
        /// Models directory path
        models_path: PathBuf,
        /// Public directory path
        public_path: PathBuf,
        /// Source directory path
        source_path: PathBuf,
    },
    /// Index the models directory and output an 'index.json' file
    #[structopt(name = "index")]
    Index {},
    /// Distribute the indexed model files to their relevant directories
    #[structopt(name = "dist")]
    Dist {},
}

fn main() {
    let paths = config::get_paths().expect("Could not read config information.");
    let config_models_path = &paths[0];
    let config_public_path = &paths[1];
    let config_src_path = &paths[2];

    match Commands::from_args() {
        Commands::Config {
            models_path,
            public_path,
            source_path,
        } => match config::config(models_path, public_path, source_path) {
            Ok(_) => println!("Successfully configured plume. Plume is now ready to use."),
            Err(error) => println!("Failed to configure plume: [{}]", error),
        },
        Commands::Index {} => {
            let path_str = config_models_path.to_str().unwrap();
            match index(config_models_path) {
                Ok(_) => println!(
                    "Successfully indexed `{}`. Outputted to `{}/{}`",
                    path_str, path_str, "index.json"
                ),
                Err(error) => println!("Failed to index `{}`: [{}]", path_str, error),
            }
        }
        Commands::Dist {} => {
            match distribute(config_models_path, config_public_path, config_src_path) {
                Ok(_) => {
                    println!("Successfully distributed the files. Parakeet is now ready to use.")
                }
                Err(error) => println!("Failed to distribute the files: [{}]", error),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Model {
    id: String,
    name: String,
    date: NaiveDate,
    description: String,
    author: String,
    parameters: Vec<parse::Parameter>,
    image_path: PathBuf,
    scad_path: PathBuf,
}

// Generate a 6 digit ID padded to the left by 0s
fn generate_id(i: i32) -> String {
    let mut num_string: String = i.to_string();
    for _j in 0..(6 - num_string.len()) {
        num_string = String::from("0") + &num_string;
    }
    return num_string;
}

// Create an `index.json` file in the models directory linking to the relevant information
fn index(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let flattened_models = parse::parse_models_dir(path, false)?;
    let mut models: Vec<Model> = Vec::new();
    let mut counter: i32 = 0;

    for entry in flattened_models {
        let info_string = fs::read_to_string(&entry.2)?;
        let info_json: Value = serde_json::from_str(&info_string)?;

        let parameters = parse::parse_parameters(
            &info_json["parameters"].as_array().unwrap(),
            info_json["name"].to_string(),
            &entry.1
        )?;

        models.push(Model {
            id: generate_id(counter),
            name: info_json["name"].as_str().unwrap().to_string(),
            date: NaiveDate::parse_from_str(&info_json["date"].as_str().unwrap().to_string(), "%Y-%m-%d")?,
            description: info_json["description"].as_str().unwrap().to_string(),
            author: info_json["author"].as_str().unwrap().to_string(),
            parameters,
            image_path: entry.0,
            scad_path: entry.1,
        });
        counter += 1;
    }

    let mut index_file = fs::File::create(path.join("index.json")).unwrap();
    writeln!(index_file, "{}", &serde_json::to_string(&models)?)?;

    Ok(())
}

// Move the indexed model files to their relevant directories for access by parakeet
// TODO: Add support for non .jpg images
fn distribute(
    models_path: &PathBuf,
    public_path: &PathBuf,
    src_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let index_string: String = fs::read_to_string(PathBuf::from(models_path).join("index.json"))?;
    let index_json: Vec<Model> = serde_json::from_str(&index_string)?;

    let scad_path = public_path.join(format!("scad/"));
    if !scad_path.exists() {
        fs::create_dir(&scad_path)?;
    }
    for entry in fs::read_dir(scad_path)? {
        let entry_path = entry?.path();
        fs::remove_file(entry_path)?;
    }

    let images_path = public_path.join(format!("images/"));
    if !images_path.exists() {
        fs::create_dir(&images_path)?;
    }
    for entry in fs::read_dir(images_path)? {
        let entry_path = entry?.path();
        fs::remove_file(entry_path)?;
    }

    let mut dist_index: Vec<Model> = Vec::new();
    for model in index_json {
        fs::metadata(&model.scad_path)?;
        fs::copy(
            &model.scad_path,
            public_path.join(format!("scad/{}.scad", &model.id)),
        )?;
        fs::metadata(&model.image_path)?;
        fs::copy(
            &model.image_path,
            public_path.join(format!("images/{}.jpg", &model.id)),
        )?;

        dist_index.push(Model {
            name: model.name,
            date: model.date,
            description: model.description,
            author: model.author,
            parameters: model.parameters,
            image_path: PathBuf::from(format!("images/{}.jpg", &model.id)),
            scad_path: PathBuf::from(format!("scad/{}.scad", &model.id)),
            id: model.id,
        });
    }

    let mut index_file = fs::File::create(src_path.join("index.json")).unwrap();
    writeln!(index_file, "{}", &serde_json::to_string(&dist_index)?)?;

    Ok(())
}
