use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum WeightUnit {
    Kg,
    Lb,
}

#[derive(Deserialize, Serialize)]
pub struct CalcRequest {
    pub total_weight: f64,
    pub bar_weight: f64,
    pub unit: WeightUnit,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CalcResponse {
    pub plates: Vec<PlateCount>,
    pub unit: WeightUnit,
}

impl CalcResponse {
    pub fn new(plates: Vec<PlateCount>, unit: WeightUnit) -> Self {
        CalcResponse { plates, unit }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlateCount {
    pub weight: String,
    pub count: usize,
}

impl PlateCount {
    pub fn new(weight: String, count: usize) -> Self {
        PlateCount { weight, count }
    }
}
