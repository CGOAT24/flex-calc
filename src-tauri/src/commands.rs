use crate::models::{CalcRequest, CalcResponse, PlateCount, WeightUnit};
use crate::AppState;
use std::collections::HashMap;
use tauri::State;
use tauri_plugin_store::JsonValue;

#[tauri::command]
pub fn calc_weights(request: CalcRequest, state: State<AppState>) -> Result<CalcResponse, String> {
    let mut remaining_weight = request.total_weight - request.bar_weight;
    let plates = vec![45.0, 35.0, 25.0, 10.0, 5.0, 2.5];
    let mut plates_needed: HashMap<String, usize> = HashMap::new();

    state.store.set("total_weight", request.total_weight);
    state.store.set("bar_weight", request.bar_weight);

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
    state.store.set("1rm_weight", weight);
    state.store.set("1rm_reps", reps);
    weight * (1.0 + (reps as f64 / 30.0))
}
