<template>
    <div class="addWindow">
        <ink-spot :size=80 positionType='absolute' bgColor="#aa99fb" posX="98%" posY="10%" :zIndex=100 />
        <ink-spot :size=30 positionType='absolute' bgColor="#aa99fb" posX="90%" posY="75%" />
        <p>修改记账窗口</p>
        <input v-model="name" type="text" placeholder="记账内容" />
        <input v-model="value" type="number" placeholder="金额" style="margin-bottom: 20px;" />
        <input v-model="date" type="date" class="date" />
        <select v-model="type"
            style="margin-bottom: 20px; padding: 5px; border-radius: 5px; border: 1px solid #ccc; width: 30%;">
            <option v-for="(label, key) in Types" :key="key" :value="label">{{ label }}</option>
        </select>
        <div style="display: flex; justify-content: center; gap: 10px;">
            <!-- 添加了动态class来控制按钮状态和动画 -->
            <button 
                @click="handleDelete" 
                class="del-btn"
                :class="{ 'confirm-delete': isDeleteConfirming }"
            >
                {{ isDeleteConfirming ? '真?' : '删除' }}
            </button>
            <button @click="handleCancel">取消</button>
            <button @click="handleSubmit">修改</button>
        </div>
    </div>
</template>

<script setup lang="ts">
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
// 添加删除确认状态变量
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
        // 二次确认后执行删除
        emit('delete', props.editingItem.id);
        resetForm();
        isDeleteConfirming.value = false;
    } else {
        // 第一次点击，进入确认状态
        isDeleteConfirming.value = true;
        
        // 5秒后自动取消确认状态
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
    // 取消时重置删除确认状态
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
.addWindow {
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    z-index: 10;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    position: fixed;
    width: 40%;
    display: flex;
    flex-direction: column;
    align-items: center;
    border-radius: 10px;
    padding: 20px;
    background: white;
    color: #353535;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

input,
select {
    width: 40%;
    padding: 8px;
    margin-top: 10px;
    border: 1px solid #ccc;
    border-radius: 5px;
}

.btn-group {
    display: flex;
    justify-content: center;
    margin-top: 10px;
}

.del-btn {
    background-color: #5F3FF6;
    transition: all 0.2s cubic-bezier(0.075, 0.82, 0.165, 1);
}

/* 添加删除确认状态的样式 */
.confirm-delete {
    background-color: #ff4d4f;
    animation: shake 0.5s ease-in-out infinite;
}

/* 定义颤抖动画 */
@keyframes shake {
    0%, 100% { transform: translate(0, 0) rotate(0deg); }
    10% { transform: translate(-1px, -1px) rotate(-0.5deg); }
    20% { transform: translate(1px, 1px) rotate(0.5deg); }
    30% { transform: translate(-1px, 1px) rotate(-0.5deg); }
    40% { transform: translate(1px, -1px) rotate(0.5deg); }
    50% { transform: translate(-1px, 0) rotate(-0.5deg); }
    60% { transform: translate(1px, 0) rotate(0.5deg); }
    70% { transform: translate(-1px, 1px) rotate(-0.5deg); }
    80% { transform: translate(1px, -1px) rotate(0.5deg); }
    90% { transform: translate(-1px, -1px) rotate(-0.5deg); }
}
    

button {
    padding: 8px 16px;
    border: none;
    border-radius: 5px;
    background: #5F3FF6;
    color: white;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.075, 0.82, 0.165, 1);
}

button:hover {
    transform: translate(0, -2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.date {
    padding: 5px;
    border: 1px solid #ccc;
    border-radius: 5px;
    width: 40%;
}

select {
    margin-top: 20px;
    margin-bottom: 20px;
    padding: 5px;
    border-radius: 5px;
    border: 1px solid #ccc;
    width: 30%;
}

@media screen and (max-width: 768px) {
    .addWindow {
        width: 80%;
    }

    input,
    select {
        width: 80%;
    }
}
</style>
