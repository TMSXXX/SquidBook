<script setup lang="ts">
import { onUnmounted, ref } from 'vue'
import { onMounted } from 'vue'
import { Types as ImportedTypes } from './types/account'
import type { Item, ItemType } from './types/account'
import { dateConvert } from './utils/dateUtil'
import { type UpdateItemParams, getItems, updateItem, addItem, deleteItem } from './api/accountApi'
import addWindow from './components/AddWindow.vue'
import EditWindow from './components/EditWindow.vue'
import ExpenseChart from './components/ExpenseChart.vue'
import InkSpot from './components/InkSpot.vue'
import ItemComponent from './components/Item.vue'





// 使用导入的 Types
const Types = ImportedTypes


const isAddItem = ref(false)
const isEditItem = ref(false)
const isViewChart = ref(false)


const list = ref<Item[]>([])
const editItem = ref<Item>({ id: 0, name: '', value: 0, type: Types.Other, created_at: '' })

// 处理添加记账项：接收 addWindow 传递的表单数据，调用 api 函数
const handleAddItem = async (formData: {
  name: string;
  value: number;
  type: ItemType;
  created_at: string
}) => {
  try {
    // 调用 api 中的 addItem 函数（参数格式完全匹配 AddItemParams）
    await addItem(formData);
    // 添加成功后：重新加载列表、关闭添加窗口
    const newList = await getItems();
    list.value = newList;
    isAddItem.value = false;
  } catch (error) {
    console.error('添加记账项失败：', error);
    alert('添加失败，请重试！');
  }
};


const handleEdit = (item: Item) => {
  if (isEditItem.value) {
    // 如果编辑窗口已经打开，关闭它
    isEditItem.value = false;
    return; // 直接返回，避免继续执行下面的代码
  }
  console.log("编辑请求：", item);
  const formattedDate = dateConvert(item.created_at);
  // 将当前项目的数据赋值给 editItem（实现回显）
  editItem.value = { ...item, created_at: formattedDate } // 深拷贝，避免直接修改原数据
  console.log("日期：", editItem.value.created_at);
  isEditItem.value = true // 打开修改窗口
}
// 处理修改记账项：接收 EditWindow 传递的修改数据（含 id）
const handleUpdateItem = async (updatedData: {
  id: number;
  name: string;
  value: number;
  type: string;
  created_at: string
}) => {
  try {
    // 调用 api 中的 updateItem 函数（参数格式匹配 UpdateItemParams）
    await updateItem(updatedData);
    // 修改成功后：重新加载列表、关闭编辑窗口
    const newList = await getItems();
    list.value = newList;
    isEditItem.value = false;
  } catch (error) {
    console.error('修改记账项失败：', error);
    alert('修改失败，请重试！');
  }
};

// 处理删除记账项
const handleDeleteItem = async (id: number) => {
  // 先提示确认，避免误删
  try {
    await deleteItem(id); // 1. 先调用删除接口，确保删除成功
    const newList = await getItems(); // 2. 获取删除后的最新数据
    list.value = newList; // 3. 赋值给 list，触发页面更新
    isEditItem.value = false;
  } catch (error) {
    console.error('删除记账项失败：', error);
    alert('删除失败，请重试！');
  }
};




onMounted(async () => {

  const data = await getItems();
  list.value = data;
});


</script>

<template>
  <div class="app-container">
    <div class="header">
      <h1>鱿型记账</h1>
    </div>

    <InkSpot :size=300 positionType='fixed' bgColor="#f9ec55" posX="0%" posY="90%" />
    <InkSpot :size=100 positionType='fixed' bgColor="#f9ec55" posX="15%" posY="75%" />
    <InkSpot :size=100 positionType='fixed' bgColor="#f9ec55" posX="90%" posY="20%" :zIndex="0" />

    <!-- 遮罩层 -->
    <div class="edit-mask" v-if="isAddItem || isEditItem || isViewChart"
      @click.self="isAddItem = false; isEditItem = false; isViewChart = false">
    </div>

    <!-- 弹窗组件 -->
    <addWindow v-if="isAddItem" :visible="isAddItem" :types="Types" @submit="handleAddItem"
      @cancel="isAddItem = false" />
    <EditWindow v-if="isEditItem" :types="Types" :editingItem="editItem" @submit="handleUpdateItem"
      @delete="handleDeleteItem" @cancel="isEditItem = false" />
    <ExpenseChart v-if="isViewChart" :expenses="list.map(item => ({ date: item.created_at, amount: item.value }))" />

    <!-- 主要内容区域（可滚动） -->
    <div class="main-content">
      <div class="content">
        <div class="feature">
          <button @click="isAddItem = !isAddItem" style="margin: 0px 10px">添加记账</button>
          <button @click="isViewChart = !isViewChart" style="margin: 0px 10px">查看汇总</button>
        </div>
      </div>

      <!-- 记账项列表（可滚动） -->
      <div class="book-container">
        <div class="book">
          <ItemComponent v-for="item in list" :key="item.id" :item="item" @edit="handleEdit" />
        </div>
      </div>
    </div>
  </div>
</template>


<style>
/* 修正全局样式 */
html,
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  position: fixed;
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  user-select: none;
  overflow: hidden;
  /* 只在外层禁用滚动 */
}

#app {
  width: 100%;
  height: 100%;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  overflow: hidden;
}
</style>


<style scoped>
.app-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  overscroll-behavior: none;
  padding-top: 5px;
  padding-bottom: 5px;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  color: white;
  text-shadow: 0px 0px 10px rgba(0, 0, 0, 1);
  text-decoration: underline solid black 0.4vw;
  height: 60px;
  background-color: #5F3FF6;
  text-align: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  margin-top: 60px;
  /* 头部高度 */
  overflow: hidden;
  overscroll-behavior: auto;
}

.content {
  padding: 20px 0;
  display: flex;
  justify-content: center;
  align-items: center;
}

.feature {
  text-align: center;
}

.book-container {
  overscroll-behavior: contain;
  overflow-y: auto;
  /* 允许垂直滚动 */
  padding: 0 20px;
  margin: 0 auto;
  width: 100%;
}

.book {
  max-width: 800px;
  margin: 0 auto;
  padding-bottom: 100px;
  /* 为底部按钮留出空间 */
}

/* 移动端样式 */
@media (max-width: 768px) {
  .header {
    padding-top: 20px;
    padding-bottom: 10px;
    height: auto;
  }



  .main-content {
    margin-top: 15vw;
  }

  .feature {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 5;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #313131;
    padding-top: 10px;
    padding-bottom: 30px;
    gap: 20px;
    overflow-block: none;
  }

  .feature button {
    width: 40%;
    padding: 12px 0;
    background-color: #5F3FF6;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 16px;
  }

  .book-container {
    overscroll-behavior: contain;
    overflow-y: auto;

    margin: 0 auto;
    padding-left: 0px;
    padding-right: 0px;
    width: 100%;
    padding-bottom: 80px;
    /* 为底部固定按钮留出更多空间 */
    overflow-x: hidden;
  }

  .book {
    width: 100%;
    margin-right: 10px;
  }
}

/* 桌面端样式 */
@media (min-width: 769px) {
  .feature {
    flex-direction: row;
    gap: 20px;
  }

  .book {
    width: 60%;
  }
}

/* 其他样式保持不变 */
.edit-mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.3);
  z-index: 6;
  display: flex;
  align-items: center;
  justify-content: center;
}

button:hover {
  transform: translate(0, -2px);
  box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
  cursor: pointer;
}

button:active {
  transform: translate(0, 1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}
</style>

