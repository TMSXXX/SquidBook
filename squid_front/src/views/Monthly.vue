<script setup lang="ts">
import { onMounted, ref, computed, watch, nextTick } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { type Item } from '../types/account';
import { getItems } from '../api/accountApi';
import { Chart, registerables } from 'chart.js';
Chart.register(...registerables);

const route = useRoute();
const router = useRouter();

// --- 数据和状态管理 ---
const items = ref<Item[]>([]);
const isLoading = ref(true);
const currentMonth = ref(route.params.month as string || new Date().toISOString().slice(0, 7));

const loadItems = async () => {
    try {
        isLoading.value = true;
        const data = await getItems();
        items.value = data;
    } catch (error) {
        console.error('加载数据失败：', error);
    } finally {
        isLoading.value = false;
    }
};

// --- 数据计算 ---
const availableMonths = computed(() => {
    const monthSet = new Set(items.value.map(item => item.created_at.slice(0, 7)));
    return Array.from(monthSet).sort().reverse();
});

const monthlyItems = computed(() => {
    return items.value.filter(item => item.created_at.startsWith(currentMonth.value));
});

const totalMonthlyExpense = computed(() => {
    return monthlyItems.value.reduce((sum, item) => sum + item.value, 0);
});

const categoryData = computed(() => {
    const categories: Record<string, number> = {};
    monthlyItems.value.forEach(item => {
        categories[item.type] = (categories[item.type] || 0) + item.value;
    });
    return {
        labels: Object.keys(categories),
        data: Object.values(categories)
    };
});

const dailyData = computed(() => {
    const daysInMonth = new Date(
        Number(currentMonth.value.split('-')[0]),
        Number(currentMonth.value.split('-')[1]),
        0
    ).getDate();
    const dailyTotals = Array(daysInMonth).fill(0);
    monthlyItems.value.forEach(item => {
        const day = new Date(item.created_at).getDate() - 1;
        dailyTotals[day] += item.value;
    });
    return {
        labels: Array.from({ length: daysInMonth }, (_, i) => `${i + 1}`),
        data: dailyTotals
    };
});

// --- 图表渲染逻辑 ---
const pieChartCanvas = ref<HTMLCanvasElement | null>(null);
const barChartCanvas = ref<HTMLCanvasElement | null>(null);
let pieChartInstance: Chart | null = null;
let barChartInstance: Chart | null = null;

const renderCharts = () => {
    if (pieChartInstance) pieChartInstance.destroy();
    if (barChartInstance) barChartInstance.destroy();
    if (pieChartCanvas.value) {
        const pieCtx = pieChartCanvas.value.getContext('2d');
        if (pieCtx) {
            pieChartInstance = new Chart(pieCtx, {
                type: 'doughnut',
                data: {
                    labels: categoryData.value.labels,
                    datasets: [{
                        data: categoryData.value.data,
                        backgroundColor: ['#FE5A98', '#99F324', '#5fc3e4', '#f9ec55', '#aa99fb', '#f39c12', '#34495e'],
                        borderColor: '#ffffff', borderWidth: 4,
                    }]
                },
                options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: 'bottom', labels: {font: {family: "'Smiley Sans', sans-serif"}} } } }
            });
        }
    }
    if (barChartCanvas.value) {
        const barCtx = barChartCanvas.value.getContext('2d');
        if (barCtx) {
            barChartInstance = new Chart(barCtx, {
                type: 'bar',
                data: {
                    labels: dailyData.value.labels,
                    datasets: [{
                        label: '每日支出', data: dailyData.value.data,
                        backgroundColor: 'rgba(153, 243, 36, 0.7)',
                        borderColor: '#353535', borderWidth: 3, borderRadius: 5,
                    }]
                },
                options: { responsive: true, maintainAspectRatio: false, scales: { y: { beginAtZero: true, ticks: {font: {family: "'Smiley Sans', sans-serif"}} }, x: { ticks: {font: {family: "'Smiley Sans', sans-serif"}} } }, plugins: { legend: { display: false } } }
            });
        }
    }
};

const onMonthChange = () => {
    router.push({ name: 'monthly', params: { month: currentMonth.value } });
};

onMounted(() => loadItems());

watch([items, currentMonth], async () => {
    if (monthlyItems.value.length > 0) {
        await nextTick();
        renderCharts();
    } else {
        if (pieChartInstance) pieChartInstance.destroy();
        if (barChartInstance) barChartInstance.destroy();
    }
});

watch(() => route.params.month, (newMonth) => {
    if (newMonth && typeof newMonth === 'string') {
        currentMonth.value = newMonth;
    }
});
</script>

<template>
    <div class="monthly-container">
        <div class="loading" v-if="isLoading">
            <p>加载中...</p>
        </div>

        <div class="summary-content" v-else>
            <div class="title-and-selector">
                <h1 class="page-title">月度分析</h1>
                <select v-model="currentMonth" @change="onMonthChange" class="month-selector">
                    <option v-for="month in availableMonths" :key="month" :value="month">
                        {{ month }}
                    </option>
                </select>
            </div>

            <div v-if="monthlyItems.length > 0">
                <div class="item-wrapper">
                    <div class="item-background" style="transform: rotate(1.5deg);"></div>
                    <div class="item-content-wrapper chart-wrapper">
                        <h2>消费分类</h2>
                        <div class="chart-with-total">
                            <div class="pie-chart-container">
                                <canvas ref="pieChartCanvas"></canvas>
                            </div>
                            <div class="chart-total">
                                <span class="total-label">本月总计</span>
                                <span class="total-value">{{ totalMonthlyExpense.toFixed(2) }}</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="item-wrapper">
                    <div class="item-background" style="transform: rotate(-1.5deg);"></div>
                    <div class="item-content-wrapper chart-wrapper">
                        <h2>每日流水</h2>
                        <div class="bar-chart-container">
                            <canvas ref="barChartCanvas"></canvas>
                        </div>
                    </div>
                </div>
            </div>
            
            <div v-else class="no-data-card">
                <p> Σ( ° △ °|||)︴</p>
                <p>这个月还没有任何记录哦！</p>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* --- 整体布局与标题 --- */
.monthly-container {
    position: relative; margin-top: 60px; height: 100vh;
    box-sizing: border-box; overflow-y: auto;
    padding: 60px 20px 150px; font-family: var(--font-normal);
}
.loading {
    text-align: center; padding: 50px; font-family: var(--font-title);
    font-size: 2em; color: #666;
}
.summary-content {
    position: relative; z-index: 1; max-width: 1000px; margin: 0 auto;
}
.title-and-selector {
    display: flex; justify-content: center; align-items: center;
    gap: 20px; flex-wrap: wrap; margin-bottom: 40px;
}
.page-title {
    font-family: var(--font-title); font-size: 2.5em;
    color: var(--text-color); margin: 0;
}

/* --- 月份选择下拉框 --- */
.month-selector {
    font-family: var(--font-title); font-size: 1.2em; padding: 10px 15px;
    border: 3px solid var(--text-color, #353535); border-radius: 10px;
    background-color: white; cursor: pointer; transform: rotate(-2deg);
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='%23353535' stroke-width='3' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat; background-position: right 15px center;
    padding-right: 50px;
}
.month-selector:focus { outline: 3px solid var(--splat-green); }

/* --- 核心：三明治结构样式 --- */
.item-wrapper {
    position: relative; width: 100%; margin-bottom: 50px;
    transition: transform 0.2s ease-out; will-change: transform;
}
.item-wrapper:hover { transform: translateY(-8px) scale(1.02); }
.item-background {
    position: absolute; top: -10px; left: -10px; right: -10px; bottom: -10px;
    background-color: var(--text-color, #353535); border-radius: 22px;
    transition: all 0.3s ease;
}
.item-wrapper:hover .item-background {
    transform: rotate(0deg) scale(1.01); background-color: var(--splat-pink);
}
.item-content-wrapper {
    position: relative; background: #ffffff; border-radius: 15px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
    transition: all 0.2s ease-out; padding: 20px; overflow: hidden;
}
.item-wrapper:hover .item-content-wrapper {
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2); transform: rotate(1deg);
}

/* --- 图表卡片特定样式 --- */
.chart-wrapper { padding: 30px; }
.chart-wrapper h2 {
    color: #333; margin-top: 0; margin-bottom: 20px;
    text-align: center; font-family: var(--font-title); font-size: 2em;
}

/* --- 饼图/圆环图特定样式 --- */
.chart-with-total { position: relative; width: 100%; height: 450px; }
.pie-chart-container { position: absolute; top: 0; left: 0; width: 100%; height: 100%; }
.chart-total {
    position: absolute; top: 45%; left: 50%;
    transform: translate(-50%, -50%); display: flex;
    flex-direction: column; align-items: center;
    justify-content: center; pointer-events: none;
}
.total-label { font-family: var(--font-normal); font-size: 1em; color: #666; }
.total-value {
    font-family: var(--font-title); font-size: 2.5em;
    font-weight: bold; color: var(--text-color); line-height: 1.1;
}

/* --- 关键修正：柱状图特定样式 --- */
.bar-chart-container {
    position: relative;
    height: 300px; /* 缩小高度 */
}


/* --- 无数据提示卡片 --- */
.no-data-card {
    background-color: #fff; border-radius: 15px; padding: 50px 20px;
    text-align: center; font-family: var(--font-title); font-size: 1.5em;
    color: #888; border: 3px dashed #ccc;
}
.no-data-card p { margin: 10px 0; }

/* --- 响应式设计 --- */
@media (max-width: 768px) {
    .monthly-container { margin-top: 80px; padding: 40px 15px 120px; }
    .page-title { font-size: 2em; }
    .chart-wrapper { padding: 20px; }
    .chart-with-total { height: 350px; }
    .bar-chart-container { height: 250px; } /* 移动端进一步缩小 */
    .total-value { font-size: 2em; }
}
</style>