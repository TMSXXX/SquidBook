<script setup lang="ts">
import { ref } from 'vue'
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
  type: ItemType;
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
  if (!confirm('确定要删除这条记账记录吗？')) return;

  try {
    await deleteItem(id); // 1. 先调用删除接口，确保删除成功
    const newList = await getItems(); // 2. 获取删除后的最新数据
    list.value = newList; // 3. 赋值给 list，触发页面更新
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
  <div>
    <div class="header">
      <h1>鱿型记账</h1>
    </div>

    <InkSpot :size=300 positionType='fixed' bgColor="#f9ec55" posX="0%" posY="90%" />
    <InkSpot :size=100 positionType='fixed' bgColor="#f9ec55" posX="15%" posY="75%" />
    <InkSpot :size=100 positionType='fixed' bgColor="#f9ec55" posX="90%" posY="20%" :zIndex="0" />

    <div class="content">
      <div class="feature">
        <button @click="isAddItem = !isAddItem" style="margin: 0px 10px">添加记账</button>
        <button @click="isViewChart = !isViewChart" style="margin: 0px 10px">查看汇总</button>
        <ExpenseChart v-if="isViewChart"
          :expenses="list.map(item => ({ date: item.created_at, amount: item.value }))" />
        <div class="edit-mask" v-if="isAddItem || isEditItem || isViewChart"
          @click.self="isAddItem = false; isEditItem = false; isViewChart = false">
        </div>
        <!-- 遮罩层，点击遮罩层关闭窗口 -->
        <addWindow v-if="isAddItem" :visible="isAddItem" :types="Types" @submit="handleAddItem"
          @cancel="isAddItem = false" />

      </div>

    </div>
    <div class="book">
      <ItemComponent v-for="item in list" :key="item.id" :item="item" @edit="handleEdit" @delete="handleDeleteItem" />
      <EditWindow v-if="isEditItem" :types="Types" :editingItem="editItem" @submit="handleUpdateItem"
        @cancel="isEditItem = false" />
    </div>
  </div>
</template>

<style scoped>
.header {
  left: 0;
  top: 0;
  z-index: 100;
  position: fixed;
  color: white;
  text-shadow: 0px 0px 10px rgba(0, 0, 0, 1);
  text-decoration: underline solid black 0.4vw;
  width: 100%;
  height: 60px;
  background-color: #5F3FF6;
  text-align: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  box-decoration-break: slice;
}

.content {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  margin-top: 100px;
  font-size: 20px;
  color: rgb(35, 35, 35);
}

.feature {
  position: relative;
  text-align: center;
  width: 80%;
}



.edit-mask {
  position: fixed;
  /* 固定定位，覆盖整个屏幕 */
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.3);
  /* 半透明黑色背景 */
  z-index: 6;
  /* 层级低于窗口（99 < 100），确保窗口在遮罩上方 */
  display: flex;
  align-items: center;
  justify-content: center;
  /* 让内部的 addWindow 居中 */
}

.book {
  position: relative;
  width: 60%;
  margin: 20px auto;
  justify-content: center;
}

button:hover {
  transform: translate(0, -2px);
  box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
  cursor: pointer;
}

input {
  margin: 8px 0 5px 0;
  padding: 5px;
  border: 1px solid #ccc;
  border-radius: 5px;
  width: 80%;
}

.del-btn {
  vertical-align: middle;
  background-color: #FF4D4F;
  font-size: 14px;
  border: none;
  color: white;
  margin-left: 10px;
  cursor: pointer;
}

.date {
  font-size: 12px;
  color: gray;
}

button:active {
  transform: translate(0, 1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

@media (max-width: 768px) {

  /* 1. 功能按钮（添加记账/查看汇总）纵向排列 */
  .feature {
    display: flex;
    flex-direction: column;
    /* 纵向排列 */
    gap: 3vw;
    /* 按钮之间的间距 */
  }

  /* 2. 记账项组件：手机端全屏显示，避免左右留白过多 */
  .book {
    width: 95%;
    /* 更贴近屏幕边缘 */
  }

  /* 3. 图表组件：手机端高度自适应，避免过高 */
  ExpenseChart {
    height: 50vw;
    /* 约250-300px，适合手机屏幕 */
  }

  /* 4. 弹窗组件（添加/编辑窗口）：手机端宽度占满屏幕 */
  addWindow,
  EditWindow {
    width: 100%;
    /* 替换电脑端的固定宽度 */
    max-width: 500px;
    /* 防止大屏手机上弹窗过宽 */
  }

  /* 5. 删除按钮：手机端加大点击区域，避免误触 */
  .del-btn {
    padding: 1.5vw 3vw;
    /* 增大内边距 */
    font-size: 3vw;
  }
}

/* 适配电脑端（屏幕宽度 >= 768px）：保持原有布局 */
@media (min-width: 769px) {
  .feature {
    flex-direction: row;
    /* 横向排列按钮 */
    gap: 20px;
  }

  .book {
    width: 60%;
    /* 固定列表宽度，避免电脑端过宽 */
  }

  ExpenseChart {
    height: 400px;
    /* 电脑端图表更高 */
  }
}
</style>
