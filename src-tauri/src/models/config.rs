use serde::{Deserialize, Serialize};
use crate::models::requests::WeightUnit;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub store: String,
    pub commands: CommandsConfig,
    pub data: DataConfig,
    pub default: DefaultConfig
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CommandsConfig {
    pub calc_weights: String,
    pub one_rm: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DataConfig {
    pub total_weight: String,
    pub bar_weight: String,
    pub weight_unit: String,
    pub one_rm_weight: String,
    pub one_rm_reps: String
}

#[derive(Deserialize, Serialize)]
pub struct DefaultConfig {
    pub total_weight: f64,
    pub bar_weight: f64,
    pub weight_unit: WeightUnit
}

#[derive(Deserialize, Serialize)]
pub struct PlateConfig {
    pub weight_unit: WeightUnit,
    pub weight: f64,
    pub disabled: bool
}