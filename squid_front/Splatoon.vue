<template>
  <div class="splatoon-app">
    <!-- 顶部标题栏 -->
    <header class="splatoon-header">
      <div class="ink-splash"></div>
      <h1 class="main-title">鱿型记账</h1>
      <div class="corner-decoration top-left"></div>
      <div class="corner-decoration top-right"></div>
    </header>

    <!-- 主内容区 -->
    <main class="content-container">
      <!-- 控制按钮 -->
      <button 
        class="action-button" 
        @click="isAddItem = !isAddItem"
        :class="{ active: isAddItem }"
      >
        <span class="button-text">添加记账</span>
        <div class="ink-drip"></div>
      </button>

      <!-- 添加窗口 - 带有液体感设计 -->
      <div 
        class="add-window" 
        v-if="isAddItem"
        :class="{ 'slide-in': isAddItem, 'slide-out': !isAddItem }"
      >
        <div class="window-corner top-left"></div>
        <div class="window-corner top-right"></div>
        <div class="window-corner bottom-left"></div>
        <div class="window-corner bottom-right"></div>
        
        <h3 class="window-title">新记账项</h3>
        
        <div class="input-group">
          <label class="input-label">内容</label>
          <input 
            v-model="inputName" 
            type="text" 
            placeholder="输入记账内容" 
            class="splatoon-input"
          >
        </div>
        
        <div class="input-group">
          <label class="input-label">金额</label>
          <input 
            v-model="inputValue" 
            type="number" 
            placeholder="0" 
            class="splatoon-input"
          >
        </div>
        
        <button 
          class="confirm-button" 
          @click="addItem"
        >
          确认
          <div class="button-splash"></div>
        </button>
      </div>

      <!-- 记账列表 - 带有墨水风格的项目 -->
      <div class="items-list">
        <h2 class="list-title">记账记录</h2>
        
        <div 
          v-for="(item, index) in list" 
          :key="item.id"
          class="list-item"
          :style="{ 
            '--ink-color': index % 3 === 0 ? '#FF5555' : index % 3 === 1 ? '#55FF55' : '#5555FF'
          }"
        >
          <div class="item-ink"></div>
          <span class="item-name">{{ item.name }}</span>
          <span class="item-value">{{ item.value }} 元</span>
          <div class="item-splat"></div>
        </div>
      </div>
    </main>

    <!-- 底部装饰 -->
    <footer class="page-footer">
      <div class="wave-pattern"></div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

// 控制添加窗口显示状态
const isAddItem = ref(false);

// 记账列表数据
const list = ref([
  { id: 1, name: '章鱼小丸子', value: 30 },
  { id: 2, name: '鱿鱼须', value: 45 },
  { id: 3, name: '墨水饮料', value: 15 },
  { id: 4, name: '战斗装备', value: 300 }
]);

// 输入框绑定值
const inputName = ref('');
const inputValue = ref(0);

// 添加新记账项
function addItem() {
  if (!inputName.value || inputValue.value === 0) return;
  
  list.value.push({
    id: Date.now(),
    name: inputName.value,
    value: inputValue.value
  });
  
  // 重置表单
  inputName.value = '';
  inputValue.value = 0;
  isAddItem.value = false;
}
</script>

<style scoped>
/* 基础样式与喷射战士风格变量 */
.splatoon-app {
  min-height: 100vh;
  background-color: #f8f9fa;
  position: relative;
  overflow-x: hidden;
}

/* 标题栏样式 - 带有墨水泼溅效果 */
.splatoon-header {
  position: relative;
  height: 120px;
  background: linear-gradient(135deg, #3a267a 0%, #5F3FF6 100%);
  overflow: hidden;
}

.main-title {
  position: relative;
  color: white;
  font-size: 2.5rem;
  font-weight: 900;
  text-align: center;
  line-height: 120px;
  margin: 0;
  text-shadow: 
    3px 3px 0px rgba(0,0,0,0.2),
    -2px -2px 0px rgba(255,255,255,0.1);
  letter-spacing: 2px;
  transform: skew(-5deg, 1deg);
}

.ink-splash {
  position: absolute;
  top: -20px;
  left: 50%;
  transform: translateX(-50%);
  width: 300px;
  height: 150px;
  background: radial-gradient(circle, rgba(255,255,255,0.2) 0%, rgba(255,255,255,0) 70%);
  border-radius: 50%;
}

.corner-decoration {
  position: absolute;
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, transparent 50%, rgba(255,255,255,0.1) 50%);
}

.top-left {
  top: 0;
  left: 0;
}

.top-right {
  top: 0;
  right: 0;
  transform: rotate(90deg);
}

/* 主内容区 */
.content-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem 1rem;
}

/* 按钮样式 - 带有液体感和动态效果 */
.action-button {
  position: relative;
  background: linear-gradient(135deg, #FF5555 0%, #CC0000 100%);
  border: none;
  color: white;
  padding: 0.8rem 2rem;
  font-size: 1.1rem;
  font-weight: bold;
  border-radius: 25px;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 4px 0 #AA0000, 0 6px 10px rgba(0,0,0,0.2);
  margin-bottom: 2rem;
}

.action-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 0 #AA0000, 0 8px 15px rgba(0,0,0,0.3);
}

.action-button.active {
  background: linear-gradient(135deg, #55FF55 0%, #00AA00 100%);
  box-shadow: 0 4px 0 #008800, 0 6px 10px rgba(0,0,0,0.2);
}

.action-button.active:hover {
  box-shadow: 0 6px 0 #008800, 0 8px 15px rgba(0,0,0,0.3);
}

.button-text {
  position: relative;
  z-index: 2;
  text-shadow: 1px 1px 2px rgba(0,0,0,0.3);
}

.ink-drip {
  position: absolute;
  bottom: -10px;
  left: 50%;
  transform: translateX(-50%);
  width: 15px;
  height: 15px;
  background-color: inherit;
  border-radius: 50% 50% 50% 0;
  transform: translateX(-50%) rotate(-45deg);
  z-index: 1;
}

/* 添加窗口样式 - 具有喷射战士的液体窗口风格 */
.add-window {
  position: relative;
  background: linear-gradient(145deg, #f0f0f0 0%, #d9d9d9 100%);
  border-radius: 15px;
  padding: 2rem;
  margin-bottom: 2rem;
  box-shadow: 0 8px 20px rgba(0,0,0,0.15);
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  overflow: hidden;
}

.add-window::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 5px;
  background: linear-gradient(90deg, #FF5555, #55FF55, #5555FF, #FF55FF);
  background-size: 400% 100%;
  animation: gradientShift 5s ease infinite;
}

@keyframes gradientShift {
  0% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}

.window-title {
  color: #333;
  font-size: 1.5rem;
  font-weight: 800;
  margin-top: 0;
  margin-bottom: 1.5rem;
  text-align: center;
  text-shadow: 1px 1px 0px rgba(255,255,255,0.5);
}

.window-corner {
  position: absolute;
  width: 25px;
  height: 25px;
  z-index: 1;
}

.window-corner.top-left {
  top: 10px;
  left: 10px;
  border-top: 3px solid rgba(0,0,0,0.1);
  border-left: 3px solid rgba(0,0,0,0.1);
}

.window-corner.top-right {
  top: 10px;
  right: 10px;
  border-top: 3px solid rgba(0,0,0,0.1);
  border-right: 3px solid rgba(0,0,0,0.1);
}

.window-corner.bottom-left {
  bottom: 10px;
  left: 10px;
  border-bottom: 3px solid rgba(0,0,0,0.1);
  border-left: 3px solid rgba(0,0,0,0.1);
}

.window-corner.bottom-right {
  bottom: 10px;
  right: 10px;
  border-bottom: 3px solid rgba(0,0,0,0.1);
  border-right: 3px solid rgba(0,0,0,0.1);
}

/* 输入框样式 */
.input-group {
  margin-bottom: 1.2rem;
  width: 100%;
}

.input-label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: #555;
}

.splatoon-input {
  width: 100%;
  padding: 0.8rem;
  border: 2px solid #ddd;
  border-radius: 8px;
  font-size: 1rem;
  transition: all 0.2s ease;
  background-color: white;
}

.splatoon-input:focus {
  outline: none;
  border-color: #5F3FF6;
  box-shadow: 0 0 0 3px rgba(95, 63, 246, 0.2);
}

/* 确认按钮 */
.confirm-button {
  position: relative;
  background: linear-gradient(135deg, #5555FF 0%, #3333AA 100%);
  border: none;
  color: white;
  padding: 0.7rem 2rem;
  font-size: 1rem;
  font-weight: bold;
  border-radius: 20px;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 3px 0 #222288;
  margin-top: 1rem;
}

.confirm-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 5px 0 #222288;
}

.button-splash {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  background-color: rgba(255,255,255,0.3);
  border-radius: 50%;
  transform: translate(-50%, -50%);
  transition: width 0.4s ease, height 0.4s ease, opacity 0.4s ease;
  opacity: 0;
}

.confirm-button:active .button-splash {
  width: 150px;
  height: 150px;
  opacity: 1;
}

/* 列表样式 */
.items-list {
  width: 100%;
}

.list-title {
  color: #3a267a;
  font-size: 1.8rem;
  font-weight: 800;
  margin-bottom: 1.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 3px solid #5F3FF6;
  text-shadow: 1px 1px 0px rgba(255,255,255,0.7);
}

/* 列表项 - 带有墨水效果 */
.list-item {
  position: relative;
  background-color: white;
  border-radius: 10px;
  padding: 1rem;
  margin-bottom: 1rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 4px 6px rgba(0,0,0,0.1);
  overflow: hidden;
  transition: transform 0.2s ease;
}

.list-item:hover {
  transform: translateY(-3px);
}

.item-ink {
  position: absolute;
  top: 0;
  left: 0;
  width: 100px;
  height: 100px;
  background-color: var(--ink-color);
  opacity: 0.1;
  border-radius: 50%;
  transform: translate(-50%, -50%);
}

.item-name {
  font-size: 1.1rem;
  font-weight: 600;
  position: relative;
  z-index: 2;
}

.item-value {
  font-size: 1.1rem;
  font-weight: bold;
  color: var(--ink-color);
  position: relative;
  z-index: 2;
}

.item-splat {
  position: absolute;
  bottom: -10px;
  right: -10px;
  width: 60px;
  height: 60px;
  background-color: var(--ink-color);
  opacity: 0.05;
  border-radius: 40% 60% 70% 30% / 40% 50% 60% 50%;
  transform: rotate(15deg);
}

/* 底部波浪装饰 */
.page-footer {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: 60px;
  overflow: hidden;
}

.wave-pattern {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 60px;
  background: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1440 320'%3E%3Cpath fill='%235F3FF6' fill-opacity='0.1' d='M0,192L48,181.3C96,171,192,149,288,149.3C384,149,480,171,576,181.3C672,192,768,192,864,181.3C960,171,1056,149,1152,149.3C1248,149,1344,171,1392,181.3L1440,192L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z'%3E%3C/path%3E%3C/svg%3E");
  background-size: 1440px 60px;
  animation: waveMove 15s linear infinite;
}

@keyframes waveMove {
  0% { background-position-x: 0; }
  100% { background-position-x: 1440px; }
}

/* 动画效果 */
.slide-in {
  animation: slideIn 0.4s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
}

.slide-out {
  animation: slideOut 0.3s ease forwards;
}

@keyframes slideIn {
  0% {
    transform: translateY(20px);
    opacity: 0;
  }
  100% {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes slideOut {
  0% {
    transform: translateY(0);
    opacity: 1;
  }
  100% {
    transform: translateY(20px);
    opacity: 0;
  }
}
</style>
    