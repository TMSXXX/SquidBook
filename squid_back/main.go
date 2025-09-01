package main

import (
	"database/sql"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	_ "github.com/mattn/go-sqlite3"
)

type Item struct {
	ID        int     `json:"id"`
	Name      string  `json:"name"`
	Value     float32 `json:"value"`
	Type      string  `json:"type"`
	CreatedAt string  `json:"created_at"`
}

func initDB() *sql.DB {
	// 连接数据库
	db, err := sql.Open("sqlite3", ".account_book.db")
	if err != nil {
		panic("无法连接数据库: " + err.Error())
	}

	createTableSQL := `CREATE TABLE IF NOT EXISTS items (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		name TEXT NOT NULL,
		value REAL NOT NULL,
		type TEXT NOT NULL,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	);`

	_, err = db.Exec(createTableSQL)
	if err != nil {
		panic("无法创建表: " + err.Error())
	}

	return db
}

var db *sql.DB

func main() {
	// 修复：去掉 := 中的冒号，改为给全局变量 db 赋值
	db = initDB() // 全局变量 db 被初始化
	defer db.Close()
	r := gin.Default()

	r.GET("/items", getItems)
	r.GET("/items/:id", getItem)
	r.POST("/items", createItem)
	r.PUT("/items/:id", updateItem)
	r.DELETE("/items/:id", deleteItem)

	r.Run(":8080")

}

func getItems(c *gin.Context) {
	rows, error := db.Query("SELECT id, name, value, type, created_at FROM items")
	if error != nil {
		c.JSON(500, gin.H{"error": "无法获取数据: " + error.Error()})
		return
	}
	defer rows.Close()
	var items []Item
	for rows.Next() {
		var item Item
		if err := rows.Scan(&item.ID, &item.Name, &item.Value, &item.Type, &item.CreatedAt); err != nil {
			c.JSON(500, gin.H{"error": "无法解析数据: " + err.Error()})
			return
		}
		items = append(items, item)
	}
	c.JSON(http.StatusOK, items)
}

func getItem(c *gin.Context) {
	// id := c.Param("id")
}

func createItem(c *gin.Context) {
	newItem := Item{}
	if error := c.ShouldBindJSON(&newItem); error != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Wrong Arguments: " + error.Error()})
		return
	}
	result, err := db.Exec("INSERT INTO items (name, value, type, created_at) VALUES (?, ?, ?, ?)", newItem.Name, newItem.Value, newItem.Type, newItem.CreatedAt)
	if err != nil {
		c.JSON(500, gin.H{"error": "无法插入数据: " + err.Error()})
		return
	}

	id, _ := result.LastInsertId()
	newItem.ID = int(id)

	c.JSON(http.StatusOK, newItem)
}

func updateItem(c *gin.Context) {
	itemID := c.Param("id")
	if itemID == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "缺少项目 ID"})
		return
	}
	var updateData Item
	if err := c.ShouldBindJSON(&updateData); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "参数错误: " + err.Error()})
		return
	}
	result, err := db.Exec("UPDATE items SET name = ?, value = ?, type = ?, created_at = ? WHERE id = ?", updateData.Name, updateData.Value, updateData.Type, updateData.CreatedAt, itemID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "无法更新数据: " + err.Error()})
		return
	}
	rowsAffected, err := result.RowsAffected()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "获取修改结果失败: " + err.Error()})
		return
	}
	if rowsAffected == 0 {
		c.JSON(http.StatusNotFound, gin.H{"error": "未找到 ID 为 " + itemID + " 的项目"})
		return
	}

	updateData.ID, _ = strconv.Atoi(itemID)
	c.JSON(http.StatusOK, gin.H{
		"message": "修改成功",
		"data":    updateData,
	})
}

func deleteItem(c *gin.Context) {
	id := c.Param("id")
	_, err := db.Exec("DELETE FROM items WHERE id = ?", id)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "无法删除数据" + err.Error()})
		return
	}
	c.JSON(http.StatusOK, gin.H{"message": "删除成功", "id": id})
}
