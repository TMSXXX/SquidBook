mod backend;

use backend::Database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 初始化数据库
            let db = Database::new().expect("Failed to initialize database");
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
            backend::update_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

