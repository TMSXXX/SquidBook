// src-tauri/src/lib.rs (已注册预算接口)

mod backend;

use backend::Database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_share::init())
        .plugin(tauri_plugin_fs::init())
        
        .setup(|app| {
            let db = Database::new(app.handle()).expect("Failed to initialize database");
            app.manage(db);
            
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            backend::get_items,
            backend::add_item,
            backend::delete_item,
            backend::update_item,
            backend::import_data,
            // --- 在这里注册新的预算命令 ---
            backend::get_monthly_budget,
            backend::set_monthly_budget
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}