<script setup lang="ts">
import { computed } from 'vue';
import InkSpot from './InkSpot.vue';

// 定义 Props（和原代码一致，用 TS 增强类型）
const props = defineProps<{
  expenses: Array<{ date: string; amount: number }>;
}>();

// 1. 随机墨点属性生成函数（和 App.vue 中逻辑一致，可复用）
const getRandomInkProps = () => {
  // 随机大小：8~20px（比 item 墨点小，避免遮挡文字）
  const size = Math.floor(Math.random() * 12) + 8;
  // 随机颜色：匹配项目色系（柔和的紫、黄、蓝）
  const colorPool = ['#aa99fb', '#f9ec55', '#5fc3e4', '#48c9b0'];
  const bgColor = colorPool[Math.floor(Math.random() * colorPool.length)];
  // 随机位置：在 <li> 内部（水平10%~90%，垂直20%~80%）
  const posX = 10 + '%';
  const posY = `${Math.floor(Math.random() * 60) + 20}%`;
  return { size, bgColor, posX, posY, positionType: 'absolute' as const };
};

// 2. 处理每月汇总：为每个月份添加墨点属性
const monthlyTotalsWithInk = computed(() => {
  const result: Array<{ month: string; total: number; inkProps: ReturnType<typeof getRandomInkProps> }> = [];
  const rawTotals = {} as Record<string, number>;

  // 先计算每月总金额（原逻辑）
  props.expenses.forEach(exp => {
    const month = exp.date.slice(0, 7); // YYYY-MM
    rawTotals[month] = (rawTotals[month] || 0) + exp.amount;
  });

  // 为每个月份添加墨点属性
  Object.entries(rawTotals).forEach(([month, total]) => {
    result.push({ month, total, inkProps: getRandomInkProps() });
  });
  return result;
});

// 3. 处理每日汇总：为每个日期添加墨点属性
const dailyTotalsWithInk = computed(() => {
  const result: Array<{ day: string; total: number; inkProps: ReturnType<typeof getRandomInkProps> }> = [];
  const rawTotals = {} as Record<string, number>;

  // 先计算每日总金额（原逻辑）
  props.expenses.forEach(exp => {
    rawTotals[exp.date] = (rawTotals[exp.date] || 0) + exp.amount;
  });

  // 为每个日期添加墨点属性
  Object.entries(rawTotals).forEach(([day, total]) => {
    result.push({ day, total, inkProps: getRandomInkProps() });
  });
  return result;
});
</script>

<template>
  <div class="Window">
    <!-- 移除原有的固定位置 InkSpot（那个是页面底部的墨点，不属于汇总列表） -->
    <p>每月汇总</p>
    <ul class="total-list">
      <!-- 每月汇总项：带墨点 -->
      <li v-for="item in monthlyTotalsWithInk" :key="item.month" class="total-item">
        <!-- 每个汇总项内部的随机墨点 -->
        <InkSpot
          :size="item.inkProps.size"
          :position-type="item.inkProps.positionType"
          :bg-color="item.inkProps.bgColor"
          :pos-x="item.inkProps.posX"
          :pos-y="item.inkProps.posY"
        />
        <!-- 汇总文字（层级高于墨点，避免被遮挡） -->
        <span class="total-text">
          {{ item.month }}: {{ item.total.toFixed(2) }} 元 <br>
          月结余：{{ (2000 - item.total).toFixed(2) }} 元
        </span>
      </li>
    </ul>

    <p>每日汇总</p>
    <ul class="total-list">
      <!-- 每日汇总项：带墨点 -->
      <li v-for="item in dailyTotalsWithInk" :key="item.day" class="total-item">
        <InkSpot
          :size="item.inkProps.size"
          :position-type="item.inkProps.positionType"
          :bg-color="item.inkProps.bgColor"
          :pos-x="item.inkProps.posX"
          :pos-y="item.inkProps.posY"
        />
        <span class="total-text">
          {{ item.day }}: {{ item.total.toFixed(2) }} 元
        </span>
      </li>
    </ul>
  </div>
</template>

<style scoped>
/* 1. 修复 Window 样式：移除 display: contents，恢复正常 flex 布局 */
.Window {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 100;
  width: 50%;
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 6px 24px rgba(0, 0, 0, 0.15);
  display: flex; /* 恢复 flex 布局，确保内部元素正常排列 */
  justify-content: center;
  align-items: center;
  flex-direction: column;
  gap: 12px;
  max-height: 80%;
  overflow-y: auto;
}

/* 2. 汇总列表容器：去掉默认 padding */
.total-list {
  height: auto;
  overflow: scroll;
  padding: 0;
  margin: 0;
}

/* 3. 汇总项：添加相对定位，作为墨点的参考容器 */
.total-item {
  position: relative; /* 关键：让墨点相对于 <li> 定位 */
  font-size: 14px;
  list-style: none;
  padding: 8px 0; /* 上下留白，避免墨点贴边 */
  padding-left: 8px; /* 左侧留白，避免文字和墨点挤在一起 */
}

/* 4. 汇总文字：提高层级，确保在墨点之上 */
.total-text {
  position: relative;
  z-index: 2; /* 高于墨点的 z-index:1 */
}

/* 5. 适配手机端：调整宽度和内边距 */
@media screen and (max-width: 768px) {
  .Window {
    width: 80%;
    padding: 16px;
  }
  .total-item {
    padding: 10px 0;
    font-size: 13px;
  }
}
</style>
