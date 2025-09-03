<template>
    <div class="item">
        <InkSpot :size=30 positionType="absolute" bgColor="#aa99fb" posX="1%" posY="10%" :zIndex=1 />
        <InkSpot :size=40 positionType="absolute" bgColor="#cabffd" :posX=r_number_x :posY=r_number_y :zIndex=1 />
        <div class="item-left">
            <span class="tag">{{ item.type }}</span>
            {{ item.name }}
            <span class="date">{{ dateConvert(item.created_at) }}</span>
        </div>
        <div class="item-right">
            <span style="float: right;">{{ item.value.toFixed(2) }}</span>
            <button class="edit-btn" @click="$emit('edit', item)">修改</button>
        </div>
    </div>

</template>

<script setup lang="ts">
import { type Item } from '@/types/account'
import InkSpot from './InkSpot.vue'
import { dateConvert } from '@/utils/dateUtil'

// 接收父组件传递的记账项数据
const props = defineProps<{
    item: Item
}>()

const r_number_x = (Math.random()*100)+10 + '%';
const r_number_y = Math.random()*100 + '%';
console.log(r_number_x, r_number_y);

</script>

<style scoped>
.item {
    z-index: 2;
    position: relative;
    overflow: hidden;
    display: flex;
    align-items: center;
    /* 关键：让所有子元素垂直居中 */
    justify-content: space-between;
    /* 让左侧内容（tag+name）和右侧内容（value+按钮）分开 */
    vertical-align: middle;
    border-radius: 10px;
    width: 100%;
    min-width: 300px;
    padding: 10px;
    margin: 10px 0;
    background: white;
    background-size: 40px 40px;
    color: #353535;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    transition: all 0.2s cubic-bezier(0.075, 0.82, 0.165, 1);
}

.item:hover {
    transform: translate(0, -2px);
    box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
    cursor: pointer;
}

.item-left {
    z-index: 2;
    display: flex;
    align-items: center;
    gap: 10px;
}

.item-right {
    z-index: 2;
    display: flex;
    align-items: center;
    gap: 10px;
}

.tag {
    background-color: #5F3FF6;
    font-size: 10px;
    color: white;
    border-radius: 5px;
    padding: 8px;
}

.date {
    font-size: 12px;
    color: gray;
}

.edit-btn {
    padding: 6px 12px;
    border: none;
    border-radius: 5px;
    margin-left: 8px;
    margin-right: 8px;
    cursor: pointer;
}

.edit-btn {
    background: #5F3FF6;
    color: white;
}



.edit-btn:hover,
.del-btn:hover {
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    transform: translate(0, -2px);
}
</style>