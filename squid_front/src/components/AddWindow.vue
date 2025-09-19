<template>
    <div class="outerWindow">
        <div class="addWindow">
            <ink-spot :size=80 positionType='absolute' bgColor="#aa99fb" posX="2%" posY="4%" :zIndex=1 />
            <ink-spot :size=30 positionType='absolute' bgColor="#aa99fb" posX="10%" posY="25%" />
            <p>添加记账窗口</p>
            <input v-model="name" type="text" placeholder="记账内容" />
            <input v-model="value" type="number" placeholder="金额" @keydown.enter.prevent="handleSubmit"/>
            <input v-model="date" type="date" class="date" />
            <select v-model="type">
                <option v-for="(label, key) in Types" :key="key" :value="label">{{ label }}</option>
            </select>
            <div class="btn-group">
                <button @click="handleCancel" class="cancel-btn">取消</button>
                <button @click="handleSubmit" class="submit-btn">添加</button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Types, type ItemType } from '@/types/account';
import { getToday } from '@/utils/dateUtil';
import InkSpot from './InkSpot.vue';
// 关键修正：不再需要从这里导入 addItem
// import { addItem } from '../api/accountApi'; 

const emit = defineEmits<{
    (e: 'submit', data: { name: string; value: number; type: ItemType; created_at: string }): void;
    (e: 'cancel'): void;
}>();

const name = ref('');
const value = ref(0);
const type = ref<ItemType>(Types.Food);
const date = ref(getToday());

// 关键修正：handleSubmit 现在只负责验证和发射事件
const handleSubmit = () => {
    if (!name.value || value.value <= 0) {
        alert('请填写完整且有效的数据！');
        return;
    }
    // 不再调用 addItem API，而是直接将数据发射出去
    emit('submit', {
        name: name.value,
        value: value.value,
        type: type.value,
        created_at: date.value,
    });
    resetForm();
};

const handleCancel = () => {
    emit('cancel');
    resetForm();
};

const resetForm = () => {
    name.value = '';
    value.value = 0;
    type.value = Types.Food;
    date.value = getToday();
};
</script>

<style scoped>
/* --- 关键：与 EditWindow 几乎完全一致的“鱿型”样式 --- */
.outerWindow {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 90%;
    max-width: 400px;
    z-index: 10;
    
    /* 增加 hover 效果 */
    transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    will-change: transform;
}
.outerWindow:hover {
    transform: translate(-50%, -50%) scale(1.02);
}

.addWindow {
    position: relative;
    background: white;
    color: #353535;
    border-radius: 15px;
    padding: 30px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow: hidden;
    isolation: isolate;
}

/* 贴纸描边效果 */
.outerWindow::before {
    content: '';
    position: absolute;
    top: -10px; left: -10px; right: -10px; bottom: -10px;
    background-color: var(--text-color, #353535);
    border-radius: 22px;
    transform: rotate(-2deg);
    transition: all 0.3s ease;
    z-index: -1;
}
.outerWindow:hover::before {
    transform: rotate(1deg) scale(1.01);
    background-color: var(--splat-pink);
}

p {
    font-family: var(--font-title);
    font-size: 2em;
    margin: 0 0 15px;
}
input, select {
    font-family: var(--font-normal);
    width: 80%;
    padding: 12px;
    margin-bottom: 15px;
    border: 3px solid var(--text-color, #353535);
    border-radius: 8px;
    font-size: 1em;
    text-align: center;
}
input:focus, select:focus {
    outline: 3px solid var(--splat-green);
    border-color: var(--splat-green);
}

.btn-group {
    display: flex;
    justify-content: center;
    gap: 10px;
    margin-top: 10px;
}
button {
    font-family: var(--font-title);
    padding: 10px 20px;
    border-radius: 10px;
    cursor: pointer;
    font-size: 1.1em;
    transition: all 0.2s ease;
    border: 3px solid var(--text-color, #353535);
}
button:hover {
    transform: translateY(-3px) scale(1.05);
}

.submit-btn {
    background: var(--splat-green);
    color: var(--text-color);
    transform: rotate(2deg);
}
.cancel-btn {
    background-color: #eee;
    color: #888;
    transform: rotate(-2deg);
    border-color: #ccc;
}

@media screen and (max-width: 768px) {
    .outerWindow { width: 80%; }
    input, select { width: 80%; }
}
</style>