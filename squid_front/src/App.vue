<!-- src/App.vue -->
<script setup lang="ts">
import { RouterView } from 'vue-router';
import FooterNav from './components/FooterNav.vue';

const goHome = () => {
  window.location.href = '/'; // 使用 window.location.href 实现页面跳转
};
</script>

<template>
  <div id="app">
    <div @click="goHome" class="header">
      <h1>墨鱼记账</h1>
    </div>
    <RouterView />
    <FooterNav />
  </div>
  
</template>

<style>
/* 保留全局样式（如 html、body、#app 的样式） */
html,
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow-x: hidden;
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
  color: rgb(0, 0, 0);
  text-shadow: 0px 0px 10px rgb(255, 255, 255);
  font-family: var(--font-title);
  font-size: 20px;
  height: 60px;
  background-color: var(--splat-pink);
  text-align: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

@media (max-width: 768px) {
  .header {
    padding-top: 20px;
    padding-bottom: 10px;
    height: auto;
  }
}
#app {
  width: 100%;
  height: 100%;
}
</style>
