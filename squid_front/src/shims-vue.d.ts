// src/shims-vue.d.ts
declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  // 更严格的类型定义（包含 props/emits 等）
  const component: DefineComponent<Record<string, unknown>, Record<string, unknown>, unknown>
  export default component
}
