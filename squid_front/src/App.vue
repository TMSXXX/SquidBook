<script setup lang="ts">
import { RouterView, useRouter } from 'vue-router'; // 1. 导入 useRouter
import FooterNav from './components/FooterNav.vue';
import InkSpot from './components/InkSpot.vue'; // 2. 导入 InkSpot 组件

const router = useRouter(); // 3. 获取 router 实例

const goHome = () => {
  // 4. 使用 router.push 实现无刷新跳转，体验更佳
  router.push('/');
};
</script>

<template>
  <div id="app">
    <div @click="goHome" class="header">
      <InkSpot :size="80" positionType='absolute' bgColor="#f9ec55" posX="5%" posY="50%" :zIndex="-1" />
      <InkSpot :size="60" positionType='absolute' bgColor="#5fc3e4" posX="95%" posY="60%" :zIndex="-1" />
      
      <h1>墨鱼记账</h1>
    </div>
    <RouterView />
    <FooterNav />
  </div>
</template>

<style>
/* --- 全局样式保持不变 --- */
html,
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow-x: hidden;
}

#app {
  width: 100%;
  height: 100%;
}

/* --- Header 样式 (已修正) --- */
.header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  height: 60px;
  background-color: var(--splat-pink);
  
  display: flex;
  justify-content: center;
  align-items: center;

  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.15);
  border-bottom: 4px solid var(--text-color, #353535);
  cursor: pointer;
  transition: all 0.3s ease;
  overflow: hidden;
}

.header::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: repeating-linear-gradient(45deg, rgba(255,255,255,0.05) 0, rgba(255,255,255,0.05) 10px, transparent 10px, transparent 20px);
  z-index: 0;
}

.header h1 {
  position: relative;
  z-index: 1;
  font-family: var(--font-title);
  
  /* 关键修正：将字号加大 */
  font-size: 36px;
  
  color: white;
  margin: 0;
  padding: 0;
  
  -webkit-text-stroke: 2px var(--text-color, #353535);
  text-shadow: 4px 4px 0px rgba(0, 0, 0, 0.2);
  
  transition: all 0.2s ease;
}

/* 悬浮交互效果 */
.header:hover {
  filter: brightness(1.1);
}
.header:hover h1 {
  transform: rotate(-2deg) scale(1.05);
}
.header:active h1 {
  transform: rotate(0deg) scale(1);
}


@media (max-width: 768px) {
  .header {
    height: 80px;
    padding: 15px 0;
  }
  .header h1 {
    /* 在移动端也相应加大 */
    font-size: 35px;
    padding-top: 10px;
    -webkit-text-stroke: 1.5px var(--text-color, #353535);
    text-shadow: 3px 3px 0px rgba(0, 0, 0, 0.2);
  }
}
</style>