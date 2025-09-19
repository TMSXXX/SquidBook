import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router';
import type { Component } from 'vue';
const HomePage = () => import('@/views/Home.vue');
const Expense = () => import('@/views/ExpensePage.vue');
const Settings = () => import('@/views/Settings.vue');

// 使用类型断言
const routes = [
  {
    path: '/',
    name: 'Home',
    component: HomePage as Component,
    meta: { title: '鱿型记账' }
  },
  {
    path: '/expense',
    name: 'Expense',
    component: Expense as Component,
    meta: { title: '支出汇总' }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: Settings as Component,
    meta: { title: '设置' }
  }
] as RouteRecordRaw[]; // 对整个数组进行类型断言

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

router.beforeEach((to) => {
  if (to.meta.title && typeof to.meta.title === 'string') {
    document.title = to.meta.title;
  }
});

export default router;