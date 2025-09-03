<template>
    <div class="outerWindow">
        <div class="addWindow">
            <ink-spot :size=80 positionType='absolute' bgColor="#aa99fb" posX="2%" posY="4%" :zIndex=100 />
            <ink-spot :size=30 positionType='absolute' bgColor="#aa99fb" posX="10%" posY="25%" />
            <p>添加记账窗口</p>
            <input v-model="name" type="text" placeholder="记账内容" />
            <input v-model="value" type="number" placeholder="金额" style="margin-bottom: 20px;" @keydown.enter.prevent="handleSubmit"/>
            <input v-model="date" type="date" class="date" />
            <select v-model="type"
                style="margin-bottom: 20px; padding: 5px; border-radius: 5px; border: 1px solid #ccc; width: 30%;">
                <option v-for="(label, key) in Types" :key="key" :value="label">{{ label }}</option>
            </select>
            <div style="display: flex; justify-content: center; gap: 10px;">
                <button @click="handleCancel">取消</button>
                <button @click="handleSubmit">添加</button>
            </div>
        </div>
    </div>



</template>


<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { Types, type ItemType } from '@/types/account';
import { getToday } from '@/utils/dateUtil';
import InkSpot from './InkSpot.vue';



// Emits：向父组件传递“提交”“取消”事件
const emit = defineEmits<{
    (e: 'submit', data: { name: string; value: number; type: ItemType; created_at: string }): void;
    (e: 'cancel'): void;
}>();

// 表单数据
const name = ref('');
const value = ref(0);
const type = ref<ItemType>(Types.Food);
const date = ref(getToday()); // 默认今日日期

// 提交表单
const handleSubmit = () => {
    if (!name.value || value.value <= 0) {
        alert('请填写完整且有效的数据！');
        return;
    }
    // 向父组件传递表单数据
    emit('submit', {
        name: name.value,
        value: value.value,
        type: type.value,
        created_at: date.value,
    });
    // 重置表单
    resetForm();
};

// 取消（关闭窗口）
const handleCancel = () => {
    emit('cancel');
    resetForm();
};

// 重置表单
const resetForm = () => {
    name.value = '';
    value.value = 0;
    type.value = Types.Food;
    date.value = getToday();
};

// 组件显示时重置表单
onMounted(() => {
    resetForm();
});
</script>


<style scoped>

.outerWindow {
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    z-index: 10;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);

    position: fixed;
    width: 40%;
}
.addWindow {
    z-index: 10;
    transform: rotate(0deg);
    overflow: hidden;
    width: 100%;
    height: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    border-radius: 10px;
    padding: 20px;
    background: white;
    color: #353535;
    /* 文字颜色改为白色，与深色背景对比 */
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    /* 添加轻微阴影，提升层次感 */
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

button {
    padding: 8px 16px;
    border: none;
    border-radius: 5px;
    background: #5F3FF6;
    color: white;
    cursor: pointer;
}

button:hover {
    transform: translate(0, -2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

date {
    margin: 4px 0 5px 0;
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
    .outerWindow {
        width: 80%;
    }
    input,
    select {
        width: 80%;
    }
    
}
</style>