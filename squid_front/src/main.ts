import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import router from './router/index'; // 引入路由配置

const app = createApp(App)
app.use(router) // 直接使用，不需要类型断言
app.mount('#app')