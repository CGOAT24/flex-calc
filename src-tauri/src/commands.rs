use crate::AppState;
use std::collections::HashMap;
use tauri::State;
use crate::models::requests::CalcRequest;
use crate::models::responses::{CalcResponse, PlateCount};
use crate::models::settings::Settings;
use crate::models::units::WeightUnit;

#[tauri::command]
pub fn calc_weights(request: CalcRequest, state: State<AppState>) -> Result<CalcResponse, String> {
    let plates = [45.0, 35.0, 25.0, 10.0, 5.0, 2.5];

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
pub fn save_settings(settings: Settings, state: State<AppState>) {
    todo!()
}
