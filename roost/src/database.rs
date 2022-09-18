use rocket::serde::Serialize;
use rocket::futures;
use rocket_db_pools::sqlx::{self, pool::PoolConnection, Sqlite, SqlitePool};
use rocket_db_pools::{Database, Connection};

use futures::{stream::TryStreamExt, future::TryFutureExt};
use sqlx::Acquire;

#[derive(Database)]
#[database("sqlx")]
pub struct Db(SqlitePool);

type DbResult<T, E = rocket::response::Debug<sqlx::Error>> = Result<T, E>;

#[derive(Serialize)]
pub struct DisplayModel {
    pub model_id: i64,
    pub name: String,
    pub creation_date: String,
    pub description: String,
    pub author: String,
    pub image_path: String,
}

pub async fn get_display_models(db: &Db) -> DbResult<Vec<DisplayModel>> {
    Ok(sqlx::query!("SELECT model_id, name, creation_date, description, author, image_path, scad_path FROM Models")
        .fetch(&mut db.0.acquire().await?)
        .map_ok(|model| {
            DisplayModel {
                model_id: model.model_id,
                name: model.name,
                creation_date: model.creation_date,
                description: model.description,
                author: model.author,
                image_path: model.image_path
            }
        })
        .try_collect::<Vec<DisplayModel>>()
        .await?)
}

#[derive(Serialize)]
pub struct Model {
    pub model_id: i64,
    pub name: String,
    pub scad_path: String,
    pub parts: Vec<Part>,
}

pub async fn get_model(db: &Db, model_id: i64) -> DbResult<Model> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    let model_info: (String, String) = sqlx::query!("SELECT name, scad_path FROM Models WHERE model_id = ?", model_id)
        .fetch_one(&mut connection)
        .map_ok(|model| (model.name, model.scad_path))
        .await?;

    Ok(Model {
        model_id,
        name: model_info.0,
        scad_path: model_info.1,
        parts: get_parts(db, model_id).await?
    })
}

#[derive(Serialize)]
pub struct Part {
    pub part_id: i64,
    pub name: String,
    pub parameters: Vec<Parameter>
}

pub async fn get_parts(db: &Db, model_id: i64) -> DbResult<Vec<Part>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    let parts_info: Vec<(i64, String)> = sqlx::query!("SELECT part_id, name FROM Parts WHERE model_id = ?", model_id)
        .fetch(&mut connection)
        .map_ok(|part| (part.part_id, part.name))
        .try_collect::<Vec<(i64, String)>>()
        .await?;

    let mut parts: Vec<Part> = Vec::new();
    for part in parts_info {
        parts.push(Part {
            part_id: part.0,
            name: part.1,
            parameters: get_parameters(db, part.0).await?
        });
    }

    Ok(parts)
}

#[derive(Serialize)]
pub enum Parameter {
    IntRange(IntRangeParameter),
    FloatRange(FloatRangeParameter),
    StringLength(StringLengthParameter),
    Bool(BoolParameter),
    IntList(IntListParameter),
    FloatList(FloatListParameter),
    StringList(StringListParameter)
}

pub async fn get_parameters(db: &Db, part_id: i64) -> DbResult<Vec<Parameter>> {
    let mut parameters: Vec<Parameter> = Vec::new();

    parameters.append(
        &mut get_int_range_parameters(db, part_id).await?
    );
    parameters.append(
        &mut get_float_range_parameters(db, part_id).await?
    );
    parameters.append(
        &mut get_string_length_parameters(db, part_id).await?
    );
    parameters.append(
        &mut get_bool_parameters(db, part_id).await?
    );
    parameters.append(
        &mut get_int_list_parameters(db, part_id).await?
    );
    parameters.append(
        &mut get_float_list_parameters(db, part_id).await?
    );
    parameters.append(
        &mut get_string_list_parameters(db, part_id).await?
    );

    Ok(parameters)
}

#[derive(Serialize)]
pub struct IntRangeParameter {
    pub parameter_id: i64,
    pub name: String,
    pub default_value: i64,
    pub lower: i64,
    pub upper: i64
}

pub async fn get_int_range_parameters(db: &Db, part_id: i64) -> DbResult<Vec<Parameter>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    Ok(sqlx::query!("SELECT parameter_id, name, default_value, lower, upper FROM IntRangeParameters WHERE part_id = ?", part_id)
        .fetch(&mut connection)
        .map_ok(|parameter| {
            Parameter::IntRange(
                IntRangeParameter {
                    parameter_id: parameter.parameter_id,
                    name: parameter.name,
                    default_value: parameter.default_value,
                    lower: parameter.lower,
                    upper: parameter.upper
                }
            )
        })
        .try_collect::<Vec<Parameter>>()
        .await?)
}

#[derive(Serialize)]
pub struct FloatRangeParameter {
    pub parameter_id: i64,
    pub name: String,
    pub default_value: f64,
    pub lower: f64,
    pub upper: f64
}

pub async fn get_float_range_parameters(db: &Db, part_id: i64) -> DbResult<Vec<Parameter>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    Ok(sqlx::query!("SELECT parameter_id, name, default_value, lower, upper FROM FloatRangeParameters WHERE part_id = ?", part_id)
        .fetch(&mut connection)
        .map_ok(|parameter| {
            Parameter::FloatRange(
                FloatRangeParameter {
                    parameter_id: parameter.parameter_id,
                    name: parameter.name,
                    default_value: parameter.default_value as f64,
                    lower: parameter.lower as f64,
                    upper: parameter.upper as f64
                }
            )
        })
        .try_collect::<Vec<Parameter>>()
        .await?)
}

#[derive(Serialize)]
pub struct StringLengthParameter {
    pub parameter_id: i64,
    pub name: String,
    pub default_value: String,
    pub length: i64,
}

pub async fn get_string_length_parameters(db: &Db, part_id: i64) -> DbResult<Vec<Parameter>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    Ok(sqlx::query!("SELECT parameter_id, name, default_value, length FROM StringLengthParameters WHERE part_id = ?", part_id)
        .fetch(&mut connection)
        .map_ok(|parameter| {
            Parameter::StringLength(
                StringLengthParameter{
                    parameter_id: parameter.parameter_id,
                    name: parameter.name,
                    default_value: parameter.default_value,
                    length: parameter.length,
                }
            )
        })
        .try_collect::<Vec<Parameter>>()
        .await?)
}

#[derive(Serialize)]
pub struct BoolParameter {
    pub parameter_id: i64,
    pub name: String,
    pub default_value: bool,
}

pub async fn get_bool_parameters(db: &Db, part_id: i64) -> DbResult<Vec<Parameter>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    Ok(sqlx::query!("SELECT parameter_id, name, default_value FROM BoolParameters WHERE part_id = ?", part_id)
        .fetch(&mut connection)
        .map_ok(|parameter| {
            Parameter::Bool(
                BoolParameter{
                    parameter_id: parameter.parameter_id,
                    name: parameter.name,
                    default_value: parameter.default_value,
                }
            )
        })
        .try_collect::<Vec<Parameter>>()
        .await?)
}

#[derive(Serialize)]
pub struct IntListParameter {
    pub parameter_id: i64,
    pub name: String,
    pub default_value: i64,
    pub items: Vec<i64>,
}

pub async fn get_int_list_parameters(db: &Db, part_id: i64) -> DbResult<Vec<Parameter>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    let parameters_info: Vec<(i64, String, i64)> = sqlx::query!("SELECT parameter_id, name, default_value FROM IntListParameters WHERE part_id = ?", part_id)
        .fetch(&mut connection)
        .map_ok(|parameter| (parameter.parameter_id, parameter.name, parameter.default_value))
        .try_collect::<Vec<(i64, String, i64)>>()
        .await?;

    let mut parameters: Vec<Parameter> = Vec::new();
    for parameter in parameters_info {
        parameters.push(
            Parameter::IntList(
                IntListParameter {
                    parameter_id: parameter.0,
                    name: parameter.1,
                    default_value: parameter.2,
                    items: get_int_list_items(db, parameter.0).await?

                }
            )
        );
    }

    Ok(parameters)
}

pub async fn get_int_list_items(db: &Db, parameter_id: i64) -> DbResult<Vec<i64>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    Ok(sqlx::query!("SELECT value FROM IntListItems WHERE parameter_id = ?", parameter_id)
        .fetch(&mut connection)
        .map_ok(|item| item.value)
        .try_collect::<Vec<i64>>()
        .await?)
}

#[derive(Serialize)]
pub struct FloatListParameter {
    pub parameter_id: i64,
    pub name: String,
    pub default_value: f64,
    pub items: Vec<f64>,
}

pub async fn get_float_list_parameters(db: &Db, part_id: i64) -> DbResult<Vec<Parameter>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    let parameters_info: Vec<(i64, String, f64)> = sqlx::query!("SELECT parameter_id, name, default_value FROM FloatListParameters WHERE part_id = ?", part_id)
        .fetch(&mut connection)
        .map_ok(|parameter| (parameter.parameter_id, parameter.name, parameter.default_value as f64))
        .try_collect::<Vec<(i64, String, f64)>>()
        .await?;

    let mut parameters: Vec<Parameter> = Vec::new();
    for parameter in parameters_info {
        parameters.push(
            Parameter::FloatList(
                FloatListParameter {
                    parameter_id: parameter.0,
                    name: parameter.1,
                    default_value: parameter.2,
                    items: get_float_list_items(db, parameter.0).await?
                }
            )
        );
    }

    Ok(parameters)
}

pub async fn get_float_list_items(db: &Db, parameter_id: i64) -> DbResult<Vec<f64>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    Ok(sqlx::query!("SELECT value FROM FloatListItems WHERE parameter_id = ?", parameter_id)
        .fetch(&mut connection)
        .map_ok(|item| item.value as f64)
        .try_collect::<Vec<f64>>()
        .await?)
}

#[derive(Serialize)]
pub struct StringListParameter {
    pub parameter_id: i64,
    pub name: String,
    pub default_value: String,
    pub items: Vec<String>,
}

pub async fn get_string_list_parameters(db: &Db, part_id: i64) -> DbResult<Vec<Parameter>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    let parameters_info: Vec<(i64, String, String)> = sqlx::query!("SELECT parameter_id, name, default_value FROM StringListParameters WHERE part_id = ?", part_id)
        .fetch(&mut connection)
        .map_ok(|parameter| (parameter.parameter_id, parameter.name, parameter.default_value))
        .try_collect::<Vec<(i64, String, String)>>()
        .await?;

    let mut parameters: Vec<Parameter> = Vec::new();
    for parameter in parameters_info {
        parameters.push(
            Parameter::StringList(
                StringListParameter {
                    parameter_id: parameter.0,
                    name: parameter.1,
                    default_value: parameter.2,
                    items: get_string_list_items(db, parameter.0).await?
                }
            )
        );
    }

    Ok(parameters)
}

pub async fn get_string_list_items(db: &Db, parameter_id: i64) -> DbResult<Vec<String>> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    Ok(sqlx::query!("SELECT value FROM StringListItems WHERE parameter_id = ?", parameter_id)
        .fetch(&mut connection)
        .map_ok(|item| item.value)
        .try_collect::<Vec<String>>()
        .await?)
}

#[derive(Clone)]
pub struct Instance {
    pub part_id: i64,
    pub path: String,
    pub usage: Option<i64>,
    pub age: Option<i64>
}

pub async fn create_instance(db: &Db, new_instance: Instance) -> DbResult<()> {
    sqlx::query!("INSERT INTO Instances (part_id, path) VALUES (?, ?)", new_instance.part_id, new_instance.path)
        .execute(&mut db.0.acquire().await?)
        .await?;

    Ok(())
}

pub async fn find_least_valuable_instance(db: &Db) -> DbResult<Instance> {
    let mut instances: Vec<Instance> = sqlx::query!("SELECT part_id, path, usage, age FROM Instances")
        .fetch(&mut db.0.acquire().await?)
        .map_ok(|instance| {
            Instance {
                part_id: instance.part_id,
                path: instance.path,
                usage: Some(instance.usage),
                age: instance.age
            }
        })
        .try_collect::<Vec<Instance>>()
        .await?;

    instances.sort_by_key(|instance| instance.usage.unwrap());

    let last = instances[instances.len() - 1].usage.unwrap();
    instances.retain(|instance| instance.usage.unwrap() == last);

    instances.sort_by_key(|instance| instance.age.unwrap());

    Ok(instances[0].clone())
}

pub async fn remove_instance(db: &Db, path: &str) -> DbResult<()> {
    sqlx::query!("DELETE FROM Instances WHERE path = ?", path)
        .execute(&mut db.0.acquire().await?)
        .await?;

    Ok(())
}

pub async fn increment_instance_usage(db: &Db, path: String) -> DbResult<()> {
    let mut connection: PoolConnection<Sqlite> = db.0.acquire().await?;

    let usage: i64 = sqlx::query!("SELECT usage FROM Instances WHERE path = ?", path)
        .fetch_one(&mut connection)
        .map_ok(|record| record.usage)
        .await?
        + 1;

    sqlx::query!("UPDATE Instances SET usage = ? WHERE path = ?", usage, path)
        .execute(&mut connection)
        .await?;

    Ok(())
}
