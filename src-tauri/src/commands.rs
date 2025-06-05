use crate::models::requests::{CalcRequest, WeightUnit};
use crate::models::responses::{CalcResponse, PlateCount};
use crate::AppState;
use std::collections::HashMap;
use serde_json::{from_value, to_value};
use tauri::State;
use crate::models::setting::Setting;

#[tauri::command]
pub fn calc_weights(request: CalcRequest, state: State<AppState>) -> Result<CalcResponse, String> {
    let setting = get_settings(state.clone());
    let weight_unit: WeightUnit = setting.weight_unit;
    let mut plates: Vec<f64> = setting.plates.iter().filter(|x| x.weight_unit == weight_unit && x.enabled).map(|x| x.weight).collect();
    plates.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut remaining_weight = request.total_weight - request.bar_weight;
    let mut plates_needed: HashMap<String, usize> = HashMap::new();

    state.store.set(
        "total_weight",
        request.total_weight,
    );
    state.store.set(
        "bar_weight",
        request.bar_weight,
    );

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
    state
        .store
        .set("1rm_weight", weight);
    state
        .store
        .set("1rm_reps", reps);
    weight * (1.0 + (reps as f64 / 30.0))
}

#[tauri::command]
pub fn change_weight_unit(unit: WeightUnit, state: State<AppState>) -> WeightUnit {
    let mut setting: Setting = get_settings(state.clone());
    setting.weight_unit = unit;
    state.store.set(
        "setting",
        to_value(setting.clone()).unwrap(),
    );
    setting.weight_unit
}

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Setting {
    if let Some(val) = state.store.get("setting") {
        let mut setting: Setting = from_value(val).unwrap();
        if setting.plates.len() == 0 {
            setting = Setting::default();
            state.store.set("setting", to_value(setting.clone()).unwrap());
        }
        setting
    }
    else {
        let setting = Setting::default();
        state.store.set("setting", to_value(setting.clone()).unwrap());
        setting
    }
}

#[tauri::command]
pub fn update_settings(weight: f64, state: State<AppState>) -> Setting {
    let mut setting: Setting = get_settings(state.clone());
    if let Some(val) = setting.plates.iter_mut().find(|item| item.weight == weight && item.weight_unit == setting.weight_unit) {
        val.enabled = !val.enabled;
    }
    state.store.set("setting", to_value(setting.clone()).unwrap());
    setting
}