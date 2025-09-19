<template>
    <div class="item-wrapper">
        <div class="item-background"></div>

        <div class="item-content-wrapper">
            <ink-spot :size=80 positionType='absolute' bgColor="#aa99fb" posX="98%" posY="10%" :zIndex=1 />
            <ink-spot :size=30 positionType='absolute' bgColor="#aa99fb" posX="90%" posY="75%" />
            <p>修改记账窗口</p>
            <input v-model="name" type="text" placeholder="记账内容" />
            <input v-model="value" type="number" placeholder="金额" />
            <input v-model="date" type="date" class="date" />
            <select v-model="type">
                <option v-for="(label, key) in Types" :key="key" :value="label">{{ label }}</option>
            </select>
            <div class="btn-group">
                <button
                    @click="handleDelete"
                    class="del-btn"
                    :class="{ 'confirm-delete': isDeleteConfirming }"
                >
                    {{ isDeleteConfirming ? '真?' : '删除' }}
                </button>
                <button @click="handleCancel" class="cancel-btn">取消</button>
                <button @click="handleSubmit" class="submit-btn">修改</button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
// --- 您的 Script 内容，100% 保持原样 ---
import { ref } from 'vue'
import { Types, type Item, type ItemType } from '@/types/account';
import { getToday } from '@/utils/dateUtil';
import InkSpot from './InkSpot.vue';

const props = defineProps<{
    editingItem: Item;
}>();

const emit = defineEmits<{
    (e: 'submit', data: { id: number, name: string; value: number; type: ItemType; created_at: string }): void;
    (e: 'cancel'): void;
    (e: 'delete', id: number): void;
}>();

const name = ref(props.editingItem.name || '');
const value = ref(props.editingItem.value || 0);
const type = ref<ItemType>(props.editingItem.type as ItemType || Types.Other);
const date = ref(props.editingItem.created_at || getToday());
const isDeleteConfirming = ref(false);

const handleSubmit = () => {
    if (!name.value || value.value <= 0) {
        alert('请填写完整且有效的数据！');
        return;
    }
    emit('submit', {
        id: props.editingItem.id,
        name: name.value,
        value: value.value,
        type: type.value,
        created_at: date.value,
    });
    resetForm();
};

const handleDelete = () => {
    if (isDeleteConfirming.value) {
        emit('delete', props.editingItem.id);
        resetForm();
        isDeleteConfirming.value = false;
    } else {
        isDeleteConfirming.value = true;
        setTimeout(() => {
            if (isDeleteConfirming.value) {
                isDeleteConfirming.value = false;
            }
        }, 5000);
    }
}

const handleCancel = () => {
    emit('cancel');
    resetForm();
    isDeleteConfirming.value = false;
};

const resetForm = () => {
    name.value = '';
    value.value = 0;
    type.value = Types.Other;
    date.value = getToday();
    isDeleteConfirming.value = false;
};
</script>

<style scoped>
/* --- 全新的、基于三明治结构的样式 --- */
.item-wrapper {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 90%;
    max-width: 400px;
    z-index: 10;
    transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    will-change: transform;
}
.item-wrapper:hover {
    transform: translate(-50%, -50%) scale(1.02);
}

.item-background {
    position: absolute;
    top: -10px; left: -10px; right: -10px; bottom: -10px;
    background-color: var(--text-color, #353535);
    border-radius: 22px;
    transform: rotate(-2deg);
    transition: all 0.3s ease;
}
.item-wrapper:hover .item-background {
    transform: rotate(1deg) scale(1.01);
    background-color: var(--splat-pink);
}

.item-content-wrapper {
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
}

/* 标题样式 */
p {
    font-family: var(--font-title);
    font-size: 2em;
    margin: 0 0 15px;
}

/* 输入框样式 */
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

/* 按钮组 */
.btn-group {
    display: flex;
    justify-content: center;
    gap: 10px;
    margin-top: 10px;
    flex-wrap: wrap; /* 增加换行，防止移动端挤压 */
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

/* 各按钮具体样式 */
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
.del-btn {
    background-color: #ff7675;
    color: white;
    transform: rotate(1deg);
    border-color: #d63031;
}
.confirm-delete {
    background-color: #ff4d4f;
    animation: shake 0.5s ease-in-out infinite;
}

@keyframes shake {
    0%, 100% { transform: rotate(1deg); }
    10%, 30%, 50%, 70%, 90% { transform: rotate(0deg); }
    20%, 40%, 60%, 80% { transform: rotate(2deg); }
}

@media screen and (max-width: 768px) {
    .item-wrapper { width: 80%; }
    input, select { width: 80%; }
}
</style>