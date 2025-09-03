use chrono::{DateTime, ParseError, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("数据库错误: {0}")]
    SqlError(#[from] rusqlite::Error),
    #[error("序列化错误: {0}")]
    SerializeError(#[from] serde_json::Error),
    #[error("时间格式错误: {0}")]
    DateTimeError(#[from] ParseError),
    #[error("路径错误: {0}")]
    PathError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub value: f64,
    #[serde(rename = "type")]
    pub item_type: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddItemRequest {
    pub name: String,
    pub value: f64,
    pub item_type: String,
    pub created_at: Option<String>,
}

// 简单的数据库连接包装
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self, DbError> {
        // 获取正确的数据库路径
        let db_path = get_database_path()?;
        
        println!("数据库路径: {}", db_path.display());
        
        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                DbError::PathError(format!("创建目录失败: {}", e))
            })?;
        }

        let conn = Connection::open(&db_path).map_err(|e| {
            DbError::PathError(format!("无法打开数据库文件 {}: {}", db_path.display(), e))
        })?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            value REAL NOT NULL,
            type TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
            [],
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

// 获取正确的数据库路径
fn get_database_path() -> Result<std::path::PathBuf, DbError> {
    #[cfg(target_os = "android")]
    {
        // 在 Android 上使用应用数据目录
        let mut path = std::path::PathBuf::from("/data/data/com.squidbook.octopus");
        path.push("databases");
        path.push("account_book.db");
        Ok(path)
    }
    
    #[cfg(not(target_os = "android"))]
    {
        // 对于非Android平台，使用当前目录
        Ok(std::path::PathBuf::from(".account_book.db"))
    }
}

// Tauri命令 - 获取所有项目
#[tauri::command]
pub async fn get_items(db: State<'_, Database>) -> Result<Vec<Item>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, value, type, created_at FROM items ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;
    
    let item_iter = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            value: row.get(2)?,
            item_type: row.get(3)?,
            created_at: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for item in item_iter {
        items.push(item.map_err(|e| e.to_string())?);
    }
    Ok(items)
}

// Tauri命令 - 添加项目
#[tauri::command]
pub async fn add_item(
    db: State<'_, Database>,
    request: AddItemRequest,
) -> Result<i64, String> {
    let mut created_at = request.created_at.unwrap_or_default();
    if created_at.is_empty() {
        let now: DateTime<Utc> = Utc::now();
        created_at = now.to_rfc3339();
    }
    
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO items (name, value, type, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![&request.name, request.value, &request.item_type, &created_at],
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid())
}

// Tauri命令 - 删除项目
#[tauri::command]
pub async fn delete_item(db: State<'_, Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    let affected = conn.execute("DELETE FROM items WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    
    if affected == 0 {
        return Err("记录不存在".to_string());
    }
    Ok(())
}

// Tauri命令 - 更新项目（直接实现）
#[tauri::command]
pub async fn update_item(
    db: State<'_, Database>,
    id: i64,
    name: String,
    value: f64,
    item_type: String,
    created_at: String,
) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    let affected = conn.execute(
        "UPDATE items SET name = ?1, value = ?2, type = ?3, created_at = ?4 WHERE id = ?5",
        params![name, value, item_type, created_at, id],
    ).map_err(|e| e.to_string())?;
    
    if affected == 0 {
        return Err("记录不存在".to_string());
    }
    Ok(())
}