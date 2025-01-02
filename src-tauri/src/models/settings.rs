use serde::{Deserialize, Serialize};
use crate::models::units::WeightUnit;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub preferred_weight_unit: WeightUnit,
    pub available_weights: Vec<AvailablePlate>,
}

#[derive(Serialize, Deserialize)]
pub struct AvailablePlate {
    pub weight: f64,
    pub weight_unit: WeightUnit,
    pub available: bool
}