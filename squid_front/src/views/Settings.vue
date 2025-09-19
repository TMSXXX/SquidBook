<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import InkSpot from '../components/InkSpot.vue';
import { invoke } from '@tauri-apps/api/core';
import { save, open } from "@tauri-apps/plugin-dialog";
import { writeTextFile, readTextFile, BaseDirectory } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from '@tauri-apps/api/path';
// 导入社区分享插件
import { shareFile } from 'tauri-plugin-share';
import type { Item } from '@/types/account';

// --- 状态管理 ---
const selectedMonth = ref(new Date().toISOString().slice(0, 7));
const budgetAmount = ref<number | null>(null);
const feedbackMessage = ref('');

// --- 预算逻辑 ---
const fetchBudgetForMonth = async (month: string) => {
    try {
        const amount = await invoke<number | null>('get_monthly_budget', { month });
        budgetAmount.value = amount;
    } catch (error) {
        console.error(`获取 ${month} 预算失败:`, error);
        feedbackMessage.value = `获取 ${month} 预算失败`;
    }
};

const saveBudget = async () => {
    if (budgetAmount.value === null || budgetAmount.value < 0) {
        alert('请输入一个有效的预算金额！');
        return;
    }
    try {
        feedbackMessage.value = '正在保存...';
        await invoke('set_monthly_budget', {
            month: selectedMonth.value,
            amount: budgetAmount.value
        });
        feedbackMessage.value = `${selectedMonth.value} 的预算已成功保存为 ${budgetAmount.value} 元！`;
        alert('保存成功！');
    } catch (error) {
        console.error('保存预算失败:', error);
        feedbackMessage.value = '保存失败';
        alert('保存失败！');
    }
};


// --- 导入/导出/分享逻辑 ---
const handleExport = async (format: 'json' | 'csv', share: boolean = false) => {
    try {
        feedbackMessage.value = '正在准备数据...';
        const items = await invoke<Item[]>('get_items');
        if (!Array.isArray(items) || items.length === 0) {
            alert('没有数据可以导出！');
            feedbackMessage.value = '';
            return;
        }

        let fileContent = '';
        let fileName = `鱿型记账导出_${new Date().toISOString().split('T')[0]}`;
        let filters = [];
        let mimeType = ''; // 新增：定义 MIME 类型变量

        if (format === 'json') {
            fileContent = JSON.stringify(items, null, 2);
            fileName += '.json';
            filters.push({ name: 'JSON Files', extensions: ['json'] });
            mimeType = 'application/json'; // JSON 的 MIME 类型
        } else {
            const headers = "id,name,value,type,created_at";
            const rows = items.map((item: any) =>
                [item.id, `"${item.name.replace(/"/g, '""')}"`, item.value, item.type, item.created_at].join(',')
            );
            fileContent = [headers, ...rows].join('\n');
            fileName += '.csv';
            filters.push({ name: 'CSV Files', extensions: ['csv'] });
            mimeType = 'text/csv'; // CSV 的 MIME 类型
        }

        // 判断是分享还是保存
        if (share) {
            feedbackMessage.value = '正在准备分享...';
            const tempDir = await appDataDir();
            // 使用 join 函数安全拼接路径
            const tempFilePath = await join(tempDir, fileName);

            await writeTextFile(tempFilePath, fileContent);

            // 关键修正：为 shareFile 添加第二个参数 mimeType
            await shareFile(tempFilePath, mimeType);

            feedbackMessage.value = '已调起分享菜单。';
        } else {
            feedbackMessage.value = '请选择保存位置...';
            const savePath = await save({ defaultPath: fileName, filters });
            if (savePath) {
                await writeTextFile(savePath, fileContent);
                feedbackMessage.value = `成功导出到 ${savePath}`;
                alert('导出成功！');
            } else {
                feedbackMessage.value = '导出已取消。';
            }
        }
    } catch (error) {
        console.error('操作失败:', error);
        feedbackMessage.value = `操作失败: ${error}`;
        if (!share) {
            alert('导出失败！');
        } else {
            alert('此设备不支持分享，或分享已取消。');
        }
    }
};

const handleImport = async () => {
    try {
        feedbackMessage.value = '请选择备份文件...';
        const selectedPath = await open({ multiple: false, filters: [{ name: 'JSON Backup', extensions: ['json'] }] });
        if (typeof selectedPath === 'string' && selectedPath) {
            feedbackMessage.value = '正在读取文件...';
            const content = await readTextFile(selectedPath);
            feedbackMessage.value = '正在导入数据...';
            await invoke('import_data', { content });
            feedbackMessage.value = '导入成功！';
            alert('导入成功！数据已刷新。');
            window.location.reload();
        } else {
            feedbackMessage.value = '导入已取消。';
        }
    } catch (error) {
        console.error('导入失败:', error);
        feedbackMessage.value = `导入失败: ${error}`;
        alert(`导入失败！\n错误: ${error}`);
    }
};

// --- 生命周期与侦听器 ---
onMounted(() => {
    fetchBudgetForMonth(selectedMonth.value);
});

watch(selectedMonth, (newMonth) => {
    feedbackMessage.value = `正在查询 ${newMonth} 的预算...`;
    fetchBudgetForMonth(newMonth);
});
</script>



<template>
    <div class="settings-container">
        <div class="summary-content">
            <div class="item-wrapper">
                <div class="item-background" style="transform: rotate(-1.5deg);"></div>
                <div class="item-content-wrapper">
                    <InkSpot :size="80" positionType='absolute' bgColor="#5fc3e4" posX="5%" posY="10%" />
                    <h2 class="card-title">月度预算设置</h2>
                    <p class="card-description">选择月份，设定该月生活费总额。</p>
                    <div class="form-group">
                        <input type="month" v-model="selectedMonth" class="month-input" />
                        <input v-model.number="budgetAmount" type="number" placeholder="输入预算金额" class="amount-input" />
                    </div>
                    <div class="btn-group">
                        <button @click="saveBudget" class="btn-save">保存设置</button>
                    </div>
                </div>
            </div>

            <div class="item-wrapper">
                <div class="item-background" style="transform: rotate(1.5deg);"></div>
                <div class="item-content-wrapper">
                    <InkSpot :size="90" positionType='absolute' bgColor="#f9ec55" posX="95%" posY="85%" />
                    <h2 class="card-title">数据管理</h2>
                    <div class="options-grid">
                        <div class="option-item">
                            <p class="option-title">导入备份</p>
                            <button @click="handleImport" class="btn-option">选择 .json 文件</button>
                        </div>
                        <div class="option-item">
                            <p class="option-title">导出备份</p>
                            <button @click="handleExport('json')" class="btn-option">导出 .json</button>
                        </div>
                        <div class="option-item">
                            <p class="option-title">导出表格</p>
                            <button @click="handleExport('csv')" class="btn-option">导出 .csv</button>
                        </div>
                        <div class="option-item">
                            <p class="option-title">手机分享</p>
                            <button @click="handleExport('json', true)" class="btn-option">分享备份</button>
                        </div>
                    </div>
                </div>
            </div>

            <div v-if="feedbackMessage" class="feedback-panel">
                <p>{{ feedbackMessage }}</p>
            </div>
        </div>
    </div>
</template>

<style scoped>
.settings-container {
    position: relative;
    margin-top: 60px;
    height: 100vh;
    box-sizing: border-box;
    overflow-y: auto;
    padding: 80px 20px 150px;
    font-family: var(--font-normal);
}

.summary-content {
    position: relative;
    z-index: 1;
    max-width: 700px;
    margin: 0 auto;
}

.item-wrapper {
    position: relative;
    width: 100%;
    margin-bottom: 50px;
    transition: transform 0.2s ease-out;
    will-change: transform;
}

.item-wrapper:hover {
    transform: translateY(-8px) scale(1.02);
}

.item-background {
    position: absolute;
    top: -10px;
    left: -10px;
    right: -10px;
    bottom: -10px;
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
    background: #fff;
    border-radius: 15px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
    transition: all 0.2s ease-out;
    padding: 30px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
}

.item-wrapper:hover .item-content-wrapper {
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
    transform: rotate(1deg);
}

.card-title {
    font-family: var(--font-title);
    font-size: 2.2em;
    margin: 0 0 10px;
    color: var(--text-color);
}

.card-description {
    font-family: var(--font-normal);
    font-size: 1em;
    color: #666;
    max-width: 80%;
    margin-bottom: 25px;
}

.form-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    width: 100%;
}

.month-input,
.amount-input {
    font-family: var(--font-title);
    font-size: 1.2em;
    padding: 10px 15px;
    border: 3px solid var(--text-color, #353535);
    border-radius: 10px;
    background-color: #fff;
    cursor: pointer;
    text-align: center;
    width: 80%;
}

.month-input:focus,
.amount-input:focus {
    outline: 3px solid var(--splat-green);
}

.btn-group {
    margin-top: 25px;
}

.btn-save {
    font-family: var(--font-title);
    border: 3px solid var(--text-color, #353535);
    padding: 12px 30px;
    border-radius: 10px;
    cursor: pointer;
    font-size: 1.2em;
    transition: all 0.2s ease;
    background-color: var(--splat-green);
    color: var(--text-color);
    transform: rotate(2deg);
}

.btn-save:hover {
    transform: translateY(-3px) rotate(-1deg) scale(1.05);
    background-color: var(--splat-pink);
    color: #fff;
}

.feedback-panel {
    margin-top: 20px;
    padding: 15px;
    border-radius: 8px;
    background-color: rgba(0, 0, 0, 0.1);
    text-align: center;
    font-family: var(--font-normal);
    color: #333;
}

.options-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    width: 100%;
}

.option-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
}

.option-title {
    font-family: var(--font-normal);
    font-weight: 700;
    margin: 0;
    color: #555;
}

.btn-option {
    font-family: var(--font-title);
    border: 3px solid var(--text-color, #353535);
    padding: 10px 20px;
    border-radius: 10px;
    cursor: pointer;
    font-size: 1em;
    transition: all 0.2s ease;
    background-color: #eee;
    color: var(--text-color);
}

.btn-option:hover {
    transform: translateY(-3px) scale(1.05);
    background-color: var(--splat-green);
    border-color: var(--text-color);
}
</style>