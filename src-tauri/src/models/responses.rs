use serde::{Deserialize, Serialize};
use crate::models::units::WeightUnit;

#[derive(Deserialize, Serialize, Debug)]
pub struct CalcResponse {
    pub plates: Vec<PlateCount>,
    pub unit: WeightUnit,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlateCount {
    pub weight: String,
    pub count: usize,
}

impl CalcResponse {
    pub fn new(plates: Vec<PlateCount>, unit: WeightUnit) -> Self {
        CalcResponse { plates, unit }
    }
}

impl PlateCount {
    pub fn new(weight: String, count: usize) -> Self {
        PlateCount { weight, count }
    }
}