// src/router/index.ts

import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import ExpensePage from '../views/ExpensePage.vue'
import Settings from '../views/Settings.vue'
import Monthly from '../views/Monthly.vue' // 1. 导入新组件

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/expense',
      name: 'expense',
      component: ExpensePage
    },
    {
      path: '/settings',
      name: 'settings',
      component: Settings
    },
    // 2. 添加新路由
    {
      path: '/monthly/:month', // 使用动态参数 :month
      name: 'monthly',
      component: Monthly,
      props: true // 自动将 :month 参数作为 prop 传递给组件
    }
  ]
})

export default router