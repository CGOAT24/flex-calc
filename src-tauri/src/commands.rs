use crate::AppState;
use std::collections::HashMap;
use std::str::FromStr;
use serde_json::Value;
use tauri::State;
use crate::models::requests::{CalcRequest, WeightUnit};
use crate::models::responses::{CalcResponse, PlateCount};

#[tauri::command]
pub fn calc_weights(request: CalcRequest, state: State<AppState>) -> Result<CalcResponse, String> {
    let setting = state.store.get(&state.constants.data.weight_unit);

    let weight_unit: WeightUnit = match setting {
        None => state.constants.default.weight_unit.clone(),
        Some(value) => {
            let def = state.constants.default.weight_unit.clone();
            if let Value::String(v) = value {
                WeightUnit::from_str(&v).unwrap_or(def)
            } else {
                def
            }
        }
    };
    
    let plates = if weight_unit == WeightUnit::Lb {
        [45.0, 35.0, 25.0, 10.0, 5.0, 2.5]
    }
    else {
        [20.0, 15.0, 10.0, 5.0, 2.5, 1.25]
    };

    let mut remaining_weight = request.total_weight - request.bar_weight;
    let mut plates_needed: HashMap<String, usize> = HashMap::new();

    state.store.set(state.constants.data.total_weight.to_string(), request.total_weight);
    state.store.set(state.constants.data.bar_weight.to_string(), request.bar_weight);

    let mut plate_index = 0;
    while remaining_weight > 0.0 {
        if remaining_weight - (plates[plate_index] * 2.0) >= 0.0 {
            if let Some(value) = plates_needed.get(&format!("{:.2}", plates[plate_index])) {
                plates_needed.insert(format!("{:.2}", plates[plate_index]), value + 2);
            } else {
                plates_needed.insert(format!("{:.2}", plates[plate_index]), 2);
            }
            remaining_weight -= plates[plate_index] * 2.0;
        } else {
            plate_index += 1;
        }
        if plate_index == plates.len() {
            return Err("No possible combination of plates".to_string());
        }
    }
    if remaining_weight < 0.0 {
        return Err("No possible combination of plates".to_string());
    }
    let plates_count = plates_needed
        .into_iter()
        .map(|x| PlateCount::new(x.0, x.1))
        .collect();
    Ok(CalcResponse::new(plates_count, WeightUnit::Lb))
}

#[tauri::command]
pub fn calc_one_rm(weight: f64, reps: usize, state: State<AppState>) -> f64 {
    state.store.set(state.constants.data.one_rm_weight.to_string(), weight);
    state.store.set(state.constants.data.one_rm_reps.to_string(), reps);
    weight * (1.0 + (reps as f64 / 30.0))
}

#[tauri::command]
pub fn change_weight_unit(unit: WeightUnit, state: State<AppState>) -> WeightUnit {
    state.store.set(state.constants.data.weight_unit.to_string(), unit.to_string());
    unit
}