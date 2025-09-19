<script setup lang="ts">
import { computed } from 'vue';

// 关键修正：动态计算当前月份的路由路径
const currentMonthPath = computed(() => {
    const now = new Date();
    const year = now.getFullYear();
    const month = (now.getMonth() + 1).toString().padStart(2, '0'); // getMonth() 返回 0-11，所以要+1
    return `/monthly/${year}-${month}`;
});
</script>

<template>
    <div class="footer-nav">
        <router-link to="/" class="nav-item">首页</router-link>
        <router-link to="/expense" class="nav-item">汇总</router-link>
        <router-link :to="currentMonthPath" class="nav-item">月度</router-link>
        <router-link to="/settings" class="nav-item">设置</router-link>
    </div>
</template>

<style scoped>
.footer-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    height: 70px;
    background-color: var(--card-bg, #ffffff);
    display: flex;
    justify-content: space-around;
    align-items: center;
    box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.1);
    z-index: 100;
    border-top: 3px solid var(--text-color, #353535);
    padding-bottom: env(safe-area-inset-bottom); /* 适配iPhone底部安全区域 */
}

.nav-item {
    font-family: var(--font-title);
    text-decoration: none;
    color: #888;
    font-size: 1.1em;
    padding: 10px;
    border-radius: 8px;
    transition: all 0.2s ease-out;
}

/* vue-router 会为当前激活的链接自动添加 .router-link-active class */
.nav-item.router-link-active {
    color: white;
    background-color: var(--splat-pink);
    transform: translateY(-5px) rotate(-2deg);
    box-shadow: 0 4px 8px rgba(0,0,0,0.2);
}
</style>