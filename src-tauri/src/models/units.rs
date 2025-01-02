use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum WeightUnit {
    Kg,
    Lb,
}