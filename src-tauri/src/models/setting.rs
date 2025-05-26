use serde::{Deserialize, Serialize};
use crate::models::requests::WeightUnit;

#[derive(Deserialize, Serialize, Clone)]
pub struct Setting {
    pub weight_unit: WeightUnit,
    pub plates: Vec<PlateSetting>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PlateSetting {
    pub weight_unit: WeightUnit,
    pub weight: f64,
    pub enabled: bool
}

impl Default for Setting {
    fn default() -> Self {
        Setting {
            weight_unit: WeightUnit::Lb,
            plates: vec![
                PlateSetting {
                    weight_unit: WeightUnit::Lb,
                    weight: 45.0,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Lb,
                    weight: 35.0,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Lb,
                    weight: 25.0,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Lb,
                    weight: 10.0,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Lb,
                    weight: 5.0,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Lb,
                    weight: 2.5,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Kg,
                    weight: 20.0,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Kg,
                    weight: 15.0,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Kg,
                    weight: 10.0,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Kg,
                    weight: 5.0,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Kg,
                    weight: 2.5,
                    enabled: true,
                },
                PlateSetting {
                    weight_unit: WeightUnit::Kg,
                    weight: 1.25,
                    enabled: true,
                }
            ]
        }
    }
}