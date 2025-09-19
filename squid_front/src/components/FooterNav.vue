<template>
  <nav class="footer-nav">
    <router-link to="/" class="nav-item">
      <i class="icon">📝</i>
      <span>记账</span>
    </router-link>
    <router-link to="/expense" class="nav-item">
      <i class="icon">📊</i>
      <span>汇总</span>
    </router-link>
    <router-link to="/settings" class="nav-item">
      <i class="icon">⚙️</i>
      <span>设置</span>
    </router-link>
  </nav>
</template>

<style scoped>
.footer-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  justify-content: space-around;
  align-items: center;
  padding: 10px 0;
  background-color: #313131;
  border-top: 1px solid #444;
  z-index: 10;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-decoration: none;
  color: #aaa;
  font-size: 12px;
  transition: color 0.3s;
}

.nav-item.router-link-exact-active {
  color: var(--splat-green); /* 选中状态的颜色 */
}

.icon {
  font-size: 24px;
  margin-bottom: 4px;
}
</style>