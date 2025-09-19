// src-tauri/src/backend.rs (已添加预算功能)

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension}; // 导入 OptionalExtension
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::{Manager, State};

// 您的 DbError 定义，保持不变
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("数据库错误: {0}")]
    SqlError(#[from] rusqlite::Error),
    #[error("序列化错误: {0}")]
    SerializeError(#[from] serde_json::Error),
    #[error("路径错误: {0}")]
    PathError(String),
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
}

// Item 和 AddItemRequest 结构体保持不变
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

// --- 新增：月度预算的结构体 ---
#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlyBudget {
    pub id: i64,
    pub month: String, // 格式 "YYYY-MM"
    pub budget_amount: f64,
}

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, DbError> {
        let app_data_path = app_handle.path().app_data_dir()
            .map_err(|e| DbError::PathError(e.to_string()))?;
        
        fs::create_dir_all(&app_data_path)?;

        let db_path = app_data_path.join("account.db");
        
        let conn = Connection::open(&db_path)?;
        
        // 创建 items 表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY, name TEXT NOT NULL, value REAL NOT NULL,
                type TEXT NOT NULL, created_at TEXT NOT NULL
            )",
            [],
        )?;

        // --- 新增：创建 monthly_budgets 表 ---
        conn.execute(
            "CREATE TABLE IF NOT EXISTS monthly_budgets (
                id INTEGER PRIMARY KEY,
                month TEXT NOT NULL UNIQUE, -- 确保每个月份只有一条记录
                budget_amount REAL NOT NULL
            )",
            [],
        )?;

        Ok(Database { conn: Mutex::new(conn) })
    }
}

// --- 已有命令保持不变 ---
#[tauri::command]
pub async fn get_items(db: State<'_, Database>) -> Result<Vec<Item>, String> {
    // ... 代码不变 ...
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, value, type, created_at FROM items ORDER BY created_at DESC").map_err(|e| e.to_string())?;
    let items_iter = stmt.query_map([], |row| {
        Ok(Item { id: row.get(0)?, name: row.get(1)?, value: row.get(2)?, item_type: row.get(3)?, created_at: row.get(4)?, })
    }).map_err(|e| e.to_string())?;
    let mut items = Vec::new();
    for item in items_iter { items.push(item.map_err(|e| e.to_string())?); }
    Ok(items)
}
#[tauri::command]
pub async fn add_item(db: State<'_, Database>, request: AddItemRequest) -> Result<i64, String> {
    // ... 代码不变 ...
    let mut created_at = request.created_at.unwrap_or_default();
    if created_at.is_empty() { created_at = Utc::now().to_rfc3339(); }
    let conn = db.conn.lock().unwrap();
    conn.execute( "INSERT INTO items (name, value, type, created_at) VALUES (?1, ?2, ?3, ?4)", params![&request.name, request.value, &request.item_type, &created_at], ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}
#[tauri::command]
pub async fn delete_item(db: State<'_, Database>, id: i64) -> Result<(), String> {
    // ... 代码不变 ...
    let conn = db.conn.lock().unwrap();
    let affected = conn.execute("DELETE FROM items WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    if affected == 0 { return Err("记录不存在".to_string()); }
    Ok(())
}
#[tauri::command]
pub async fn update_item(db: State<'_, Database>, id: i64, request: AddItemRequest) -> Result<(), String> {
    // ... 代码不变 ...
    let conn = db.conn.lock().unwrap();
    let affected = conn.execute( "UPDATE items SET name = ?1, value = ?2, type = ?3, created_at = ?4 WHERE id = ?5", params![&request.name, request.value, &request.item_type, &request.created_at, id], ).map_err(|e| e.to_string())?;
    if affected == 0 { return Err("记录不存在".to_string()); }
    Ok(())
}
#[tauri::command]
pub async fn import_data(db: State<'_, Database>, content: String) -> Result<(), String> {
    // ... 代码不变 ...
    let items_to_import: Vec<Item> = serde_json::from_str(&content).map_err(|e| format!("JSON 解析失败: {}", e))?;
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        for item in items_to_import {
            tx.execute( "INSERT INTO items (name, value, type, created_at) VALUES (?1, ?2, ?3, ?4)", params![item.name, item.value, item.item_type, item.created_at], ).map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

// --- 新增：获取指定月份的预算 ---
#[tauri::command]
pub async fn get_monthly_budget(db: State<'_, Database>, month: String) -> Result<Option<f64>, String> {
    let conn = db.conn.lock().unwrap();
    let amount: Option<f64> = conn.query_row(
        "SELECT budget_amount FROM monthly_budgets WHERE month = ?1",
        params![month],
        |row| row.get(0),
    )
    .optional() // 关键：如果查询没有结果，返回 Ok(None) 而不是 Err
    .map_err(|e| e.to_string())?;
    
    Ok(amount)
}

// --- 新增：设定/更新指定月份的预算 (UPSERT) ---
#[tauri::command]
pub async fn set_monthly_budget(db: State<'_, Database>, month: String, amount: f64) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    // "INSERT OR REPLACE" 是 SQLite 的一个方便的 UPSERT 写法
    // 它会基于 UNIQUE 约束（我们设置在 month 字段上）来判断
    // 如果 month 已存在，就替换（UPDATE）整行；如果不存在，就插入（INSERT）
    conn.execute(
        "INSERT OR REPLACE INTO monthly_budgets (month, budget_amount) VALUES (?1, ?2)",
        params![month, amount],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}