import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    // 配置代理
    proxy: {
      // 1. 匹配所有以 "/api" 开头的请求
      '/api': {
        target: 'http://localhost:8080', // 后端接口的实际地址
        changeOrigin: true, // 允许跨域（代理服务器会伪装请求来源为后端域名）
        rewrite: (path) => path.replace(/^\/api/, '') // 转发时去掉路径中的 "/api" 前缀
      }
    }

  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'), // 配置@为src目录别名
    },
  },
})
    