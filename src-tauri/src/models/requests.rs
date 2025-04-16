use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CalcRequest {
    pub total_weight: f64,
    pub bar_weight: f64,
    pub unit: WeightUnit,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum WeightUnit {
    Kg,
    Lb,
}

impl Display for WeightUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self == &WeightUnit::Kg {
            return write!(f, "Kg");
        }
        write!(f, "Lb")
    }
}

impl FromStr for WeightUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kg" => Ok(WeightUnit::Kg),
            "lb" => Ok(WeightUnit::Lb),
            _ => Err(()),
        }
    }
}