<script setup lang="ts">
import { onMounted, ref, computed, watch, nextTick } from 'vue';
import { type Item } from '../types/account';
import { getItems } from '../api/accountApi';
import InkSpot from '../components/InkSpot.vue';
import { useRouter } from 'vue-router';
import { Chart, registerables } from 'chart.js';
import { invoke } from '@tauri-apps/api/core';
Chart.register(...registerables);

const router = useRouter();

// --- 数据和状态管理 ---
const expenses = ref<Item[]>([]);
const budgets = ref<Record<string, number>>({});
const isLoading = ref(true);

// --- 数据获取与处理 ---
const loadData = async () => {
    try {
        // isLoading 已经在 watch 中处理，这里只负责重置
        if (!isLoading.value) {
            isLoading.value = true;
        }
        expenses.value = [];
        budgets.value = {};

        const data = await getItems();
        expenses.value = data;
        await loadAllBudgets();

    } catch (error) {
        console.error('加载数据失败：', error);
        alert('加载数据失败，请重试！');
    } finally {
        // 关键：让 watch 来处理后续
        isLoading.value = false;
    }
};

const loadAllBudgets = async () => {
    const monthSet = new Set(expenses.value.map(item => item.created_at.slice(0, 7)));
    if (monthSet.size === 0) return;

    const budgetPromises = Array.from(monthSet).map(async (month) => {
        const amount = await invoke<number | null>('get_monthly_budget', { month });
        return { month, amount: amount || 0 };
    });
    const budgetResults = await Promise.all(budgetPromises);
    const budgetMap: Record<string, number> = {};
    budgetResults.forEach(b => {
        budgetMap[b.month] = b.amount;
    });
    budgets.value = budgetMap;
};

// --- 计算属性 (保持不变) ---
const monthlySummary = computed(() => {
    const result: Record<string, { amount: number }> = {};
    expenses.value.forEach(item => {
        const month = item.created_at.slice(0, 7);
        if (!result[month]) {
            result[month] = { amount: 0 };
        }
        result[month].amount += item.value;
    });

    return Object.entries(result)
        .sort((a, b) => b[0].localeCompare(a[0]))
        .map(([month, { amount }]) => {
            const budget = budgets.value[month] || 0;
            const balance = budget - amount;
            return { month, amount, budget, balance };
        });
});

const totalExpense = computed(() => expenses.value.reduce((sum, item) => sum + item.value, 0));
const averageDailyExpense = computed(() => {
    if (expenses.value.length === 0) return 0;
    const dates = expenses.value.map(item => new Date(item.created_at));
    const timestamps = dates.map(date => date.getTime());
    const earliest = Math.min(...timestamps);
    const latest = Math.max(...timestamps);
    const days = Math.ceil((latest - earliest) / (1000 * 60 * 60 * 24)) || 1;
    return totalExpense.value / days;
});

// --- 图表逻辑 ---
const chartCanvas = ref<HTMLCanvasElement | null>(null);
let chartInstance: Chart | null = null;

const updateChart = () => {
    if (chartCanvas.value && monthlySummary.value.length > 0) {
        const chartData = [...monthlySummary.value].reverse();
        const labels = chartData.map(item => item.month);
        const data = chartData.map(item => item.amount);
        renderChart(labels, data);
    } else {
        if (chartInstance) {
            chartInstance.destroy();
            chartInstance = null;
        }
    }
}

const renderChart = (labels: string[], data: number[]) => {
    if (chartInstance) chartInstance.destroy();
    if (!chartCanvas.value) return;
    const ctx = chartCanvas.value.getContext('2d');
    if (!ctx) return;
    chartInstance = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: labels,
            datasets: [{
                label: '每月支出', data,
                backgroundColor: 'rgba(254, 90, 152, 0.7)',
                borderColor: '#353535', borderWidth: 3, borderRadius: 5,
            }]
        },
        options: {
            responsive: true, maintainAspectRatio: false,
            scales: {
                y: { beginAtZero: true, ticks: {font: {family: "'Smiley Sans', sans-serif"}} },
                x: { ticks: {font: {family: "'Smiley Sans', sans-serif"}} }
            },
            plugins: { legend: { display: false } }
        }
    });
};

// 关键修正：使用 watch 侦听 isLoading 的变化
watch(isLoading, async (newIsLoading) => {
    // 当加载结束时 (从 true -> false)
    if (!newIsLoading) {
        // 等待 Vue 将 v-else 的内容渲染到 DOM 中
        await nextTick();
        // 现在，chartCanvas.value 肯定存在，我们再调用图表更新
        updateChart();
    }
});

onMounted(() => {
    loadData();
});
</script>

<template>
    <div class="summary-container">
        <div class="loading" v-if="isLoading"><p>加载中...</p></div>
        <div class="summary-content" v-else>
            <div class="stats-cards">
                <div class="item-wrapper">
                    <div class="item-background" style="transform: rotate(2deg);"></div>
                    <div class="item-content-wrapper stat-card">
                        <h3>总支出</h3>
                        <p class="stat-value">{{ totalExpense.toFixed(2) }}</p>
                    </div>
                </div>
                <div class="item-wrapper">
                    <div class="item-background" style="transform: rotate(-2deg);"></div>
                    <div class="item-content-wrapper stat-card">
                        <h3>记录条数</h3>
                        <p class="stat-value">{{ expenses.length }}</p>
                    </div>
                </div>
                <div class="item-wrapper">
                    <div class="item-background" style="transform: rotate(1deg);"></div>
                    <div class="item-content-wrapper stat-card">
                        <h3>平均每日支出</h3>
                        <p class="stat-value">{{ averageDailyExpense.toFixed(2) }}</p>
                    </div>
                </div>
            </div>
            <div class="item-wrapper">
                <div class="item-background" style="transform: rotate(1deg);"></div>
                <div class="item-content-wrapper chart-wrapper">
                    <h2>月度支出图表</h2>
                    <div class="chart-container"><canvas ref="chartCanvas"></canvas></div>
                </div>
            </div>
            <div class="item-wrapper">
                <div class="item-background" style="transform: rotate(-1.5deg);"></div>
                <div class="item-content-wrapper monthly-summary">
                    <h2>月度支出汇总</h2>
                    <table>
                        <thead>
                            <tr>
                                <th>月份</th>
                                <th>预算</th>
                                <th>总支出</th>
                                <th>结余</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="item in monthlySummary" :key="item.month">
                                <td><router-link :to="`/monthly/${item.month}`">{{ item.month }}</router-link></td>
                                <td>{{ item.budget.toFixed(2) }}</td>
                                <td>{{ item.amount.toFixed(2) }}</td>
                                <td :class="{ 'positive-balance': item.balance >= 0, 'negative-balance': item.balance < 0 }">{{ item.balance.toFixed(2) }}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
            <div class="action-buttons">
                <button @click="loadData">刷新数据</button>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* --- 全局容器和背景 --- */
.summary-container {
    position: relative;
    margin-top: 60px;
    height: 100vh;
    box-sizing: border-box;
    overflow-y: auto;
    /* 关键修正2：增加底部内边距，为刷新按钮留出空间 */
    padding: 80px 20px 150px; /* 上 左右 下 */
    -webkit-overflow-scrolling: touch;
    font-family: var(--font-normal);
}
.loading {
    text-align: center;
    padding: 50px;
    font-family: var(--font-title);
    font-size: 2em;
    color: #666;
}
.summary-content {
    position: relative;
    z-index: 1;
    max-width: 1000px;
    margin: 0 auto;
}


/* --- 核心：三明治结构样式 --- */
.item-wrapper {
    position: relative;
    width: 100%;
    margin-bottom: 40px;
    transition: transform 0.2s ease-out;
    /* 关键修正1：添加 will-change 优化动画，防止闪烁 */
    will-change: transform;
}
.item-wrapper:hover {
    transform: translateY(-8px) scale(1.02);
}

.item-background {
    position: absolute;
    top: -10px; left: -10px; right: -10px; bottom: -10px;
    background-color: var(--text-color, #353535);
    border-radius: 22px;
    transition: all 0.3s ease;
}
.item-wrapper:hover .item-background {
    transform: rotate(0deg) scale(1.01);
    background-color: var(--splat-pink);
}

.item-content-wrapper {
    position: relative;
    background: #ffffff;
    border-radius: 15px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
    transition: all 0.2s ease-out;
    padding: 20px;
    overflow: hidden;
}
.item-wrapper:hover .item-content-wrapper {
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
    transform: rotate(1deg);
}


/* --- 统计卡片特定样式 --- */
.stats-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 40px;
    margin-bottom: 20px;
}
.stats-cards .item-wrapper {
    margin-bottom: 0;
}
.stat-card {
    text-align: center;
    padding: 20px;
}
.stat-card h3 {
    color: #666;
    margin-bottom: 10px;
    font-size: 16px;
    font-family: var(--font-normal);
}
.stat-value {
    font-size: 2.2em;
    font-weight: bold;
    color: var(--splat-pink);
    margin: 0;
    font-family: var(--font-title);
    line-height: 1.1;
}

/* --- 图表和表格标题 --- */
.chart-wrapper h2, .monthly-summary h2 {
    color: #333;
    margin-top: 0;
    margin-bottom: 20px;
    text-align: center;
    font-family: var(--font-title);
    font-size: 2em;
}

/* --- 图表卡片 --- */
.chart-wrapper { padding: 30px; }
.chart-container { position: relative; height: 300px; }

/* --- 表格卡片 --- */
.monthly-summary { padding: 30px; }
table { width: 100%; border-collapse: collapse; }
th, td { padding: 15px; text-align: left; border: none; border-bottom: 2px dashed rgba(0,0,0,0.1); }
th {
    font-family: var(--font-title);
    background-color: transparent;
    color: #333;
    font-size: 1.2em;
}
tr:last-child td { border-bottom: none; }
tr:hover { background-color: #fdf59a; }

/* --- 按钮样式 --- */
.action-buttons { text-align: center; margin-top: 20px; }
.action-buttons button {
    font-family: var(--font-title);
    background-color: var(--splat-pink);
    color: white;
    border: 3px solid var(--text-color, #353535);
    padding: 12px 25px;
    border-radius: 10px;
    cursor: pointer;
    font-size: 1.2em;
    transition: all 0.2s ease;
    transform: rotate(2deg);
}
.action-buttons button:hover {
    background-color: var(--splat-green);
    color: var(--text-color, #353535);
    transform: translateY(-3px) rotate(-1deg) scale(1.05);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

/* --- 响应式设计 --- */
@media (max-width: 768px) {
    .summary-container {
        margin-top: 80px;
        /* 在移动端也确保有足够的底部空间 */
        padding: 60px 15px 200px;
    }
    .stats-cards {
        gap: 30px;
    }
    .monthly-summary, .chart-wrapper, .stat-card {
        padding: 20px;
    }
    th, td {
        padding: 10px;
        font-size: 14px;
    }
}
</style>