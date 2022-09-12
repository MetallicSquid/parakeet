// ***** Plume *****
// Tool for the management of models in parakeet.
// Commands:
//  * config        -> Sets up the plume configuration with the provided paths
//  * index         -> Traverses and indexes the models in the models directory

mod config;
mod parse;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use sqlx::{SqlitePool};
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
        /// Build directory path
        build_path: PathBuf,
        /// Database file (.sqlite) path
        database_path: PathBuf,
        /// Maximum number of .stl models stored at any one time
        model_limit: i64
    },
    /// Index the models directory and output an 'index.json' file
    #[structopt(name = "index")]
    Index {},
}

#[tokio::main]
async fn main() {
    let paths = config::get_paths().expect("Could not read config information.");
    let config_models_path = &paths[0];
    let config_build_path= &paths[1];
    let config_database_path = &paths[2];

    // Development: For use upon changing ParakeetConfig format
    // let config_models_path = &PathBuf::from("../models");
    // let config_build_path= &PathBuf::from("../build");
    // let config_database_path = &PathBuf::from("../database");

    match Commands::from_args() {
        Commands::Config {
            models_path,
            build_path,
            database_path,
            model_limit
        } => match config::config(models_path, build_path, database_path, model_limit) {
            Ok(_) => println!("Successfully configured plume. Plume is now ready to use."),
            Err(error) => println!("Failed to configure plume: [{}]", error),
        },
        Commands::Index {} => {
            let path_str = config_models_path.to_str().unwrap();
            let pool: SqlitePool = SqlitePool::connect(&format!("sqlite:{}", &config_database_path.to_str().unwrap()))
                .await
                .expect("Failed to connect to database.");
            match index(config_build_path, config_models_path, pool).await {
                Ok(_) => println!(
                    "Successfully indexed `{}`. Outputted to `{}`",
                    path_str,
                    &config_database_path.to_str().unwrap()
                ),
                Err(error) => println!("Failed to index `{}`: [{}]", path_str, error),
            }
        }
    }
}

// Create an `index.json` file in the models directory linking to the relevant information
async fn index(build_path: &PathBuf, models_path: &PathBuf, pool: SqlitePool) -> Result<(), Box<dyn Error>> {
    let scad_path = build_path.join("scad/");
    if !scad_path.exists() {
        fs::create_dir(&scad_path)?;
    }
    for entry in fs::read_dir(scad_path)? {
        let entry_path = entry?.path();
        fs::remove_file(entry_path)?;
    }

    let images_path = build_path.join("images/");
    if !images_path.exists() {
        fs::create_dir(&images_path)?;
    }
    for entry in fs::read_dir(images_path)? {
        let entry_path = entry?.path();
        fs::remove_file(entry_path)?;
    }

    let stls_path = build_path.join("stls/");
    if !stls_path.exists() {
        fs::create_dir(&stls_path)?;
    }

    let flattened_models = parse::traverse_models_dir(models_path, false)?;

    for entry in flattened_models {
        let info_string = fs::read_to_string(&entry.2)?;
        let info_json: Value = serde_json::from_str(&info_string)?;

        let model_id: i64 = parse::db_add_model(
            &pool,
            info_json["name"].as_str().unwrap(),
            info_json["date"].as_str().unwrap(),
            info_json["description"].as_str().unwrap(),
            info_json["author"].as_str().unwrap(),
            build_path.join(format!("images/{}.jpg", info_json["name"].as_str().unwrap())).to_str().unwrap(),
            build_path.join(format!("scad/{}.scad", info_json["name"].as_str().unwrap())).to_str().unwrap(),
        ).await?;

        fs::metadata(&entry.1)?;
        fs::copy(
            &entry.1,
            build_path.join(format!("scad/{}.scad", info_json["name"].as_str().unwrap())),
        )?;
        fs::metadata(&entry.0)?;
        fs::copy(
            &entry.0,
            build_path.join(format!("images/{}.jpg", info_json["name"].as_str().unwrap())),
        )?;

        parse::parse_parts(
            &pool,
            &info_json["parts"].as_array().unwrap(),
            info_json["name"].as_str().unwrap(),
            model_id,
            &entry.1
        ).await?;
    }

    Ok(())
}
