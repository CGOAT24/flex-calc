use serde::{Deserialize, Serialize};
use crate::models::units::WeightUnit;

#[derive(Deserialize, Serialize)]
pub struct CalcRequest {
    pub total_weight: f64,
    pub bar_weight: f64,
    pub unit: WeightUnit,
}