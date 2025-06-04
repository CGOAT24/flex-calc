use crate::commands::{calc_one_rm, calc_weights, change_weight_unit, get_settings, update_settings};
use std::sync::Arc;
use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};

mod commands;
mod models;

pub struct AppState {
    store: Arc<Store<Wry>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store("store.json")?;

            app.manage(AppState {
                store
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            calc_weights,
            calc_one_rm,
            change_weight_unit,
            get_settings,
            update_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
