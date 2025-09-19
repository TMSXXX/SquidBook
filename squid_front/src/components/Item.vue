<template>
    <div class="item-wrapper">
        <div class="item-background"></div>

        <div class="item-content-wrapper">
            <InkSpot :size=80 positionType="absolute" :bgColor="splatColor1" :posX="r_x1" :posY="r_y1" :zIndex=1 :rotation="r_rot1" />
            <InkSpot :size=60 positionType="absolute" :bgColor="splatColor2" :posX="r_x2" :posY="r_y2" :zIndex=1 :rotation="r_rot2" />
            
            <div class="item-content-inner">
                <div class="item-left">
                    <span class="tag">{{ item.type }}</span>
                    <span class="name">{{ item.name }}</span>
                </div>
                <div class="item-right">
                    <span class="value">{{ item.value.toFixed(2) }}</span>
                    <button class="edit-btn" @click.stop="$emit('edit', item)">修改</button>
                </div>
            </div>
            <span class="date">{{ dateConvert(item.created_at) }}</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { type Item } from '@/types/account'
import InkSpot from './InkSpot.vue'
import { dateConvert } from '@/utils/dateUtil'

const props = defineProps<{
    item: Item
}>()

// 随机数部分保持不变
const r_x1 = Math.random() * 100 + '%';
const r_y1 = 20 + Math.random() * 80 + '%';
const r_rot1 = Math.random() * 360;

const r_x2 = Math.random() * 100 + '%';
const r_y2 = 20 + Math.random() * 80 + '%';
const r_rot2 = Math.random() * 360;

const splatColor1 = "rgba(254, 90, 152, 0.5)"; // 粉色，50%透明度
const splatColor2 = "rgba(153, 243, 36, 0.5)"; // 绿色，50%透明度
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Rock+Salt&display=swap');

/* 1. 外层包装纸：负责定位和间距 */
.item-wrapper {
    position: relative;
    width: 100%;
    min-width: 300px;
    margin: 25px 0;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.075, 0.82, 0.165, 1);
}

/* 2. 背景描边：它就是一个简单的div */
.item-background {
    position: absolute;
    top: -6px;
    left: -6px;
    right: -6px;
    bottom: -6px;
    background-color: rgb(58, 58, 58);
    border-radius: 18px; /* 圆角比内容稍大一点，更好看 */
    transform: rotate(-3deg);
    transition: all 0.3s ease;
}

/* 3. 内容层：您的白色卡片 */
.item-content-wrapper {
    font-family: var(--font-normal);
    position: relative; /* 为内部的InkSpot提供定位 */
    background: #ffffff; /* 白色背景在这里 */
    border-radius: 15px;
    padding: 12px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
    display: flex;
    flex-direction: column;
    gap: 10px;
    overflow: hidden; /* 可以裁剪内部溢出的墨迹 */
    transition: all 0.2s cubic-bezier(0.075, 0.82, 0.165, 1);
}

/* --- 悬浮效果 --- */
.item-wrapper:hover {
    transform: translateY(-3px);
}

.item-wrapper:hover .item-background {
    transform: rotate(1deg) scale(1.02);
    background-color: var(--splat-pink);
}

.item-wrapper:hover .item-content-wrapper {
    transform: rotate(-1deg);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
}


/* --- 内部元素样式，基本保持不变 --- */
.item-content-inner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    z-index: 2;
}

.item-left, .item-right {
    display: flex;
    align-items: center;
    gap: 12px;
}

.name {
    font-size: 1.1em;
}

.value {
    font-weight: bold;
    font-size: 1.2em;
}

.tag {
    background-color: var(--splat-pink);
    font-size: 12px;
    font-weight: bold;
    color: white;
    border-radius: 5px;
    padding: 8px 10px;
    transform: rotate(-2.5deg);
    box-shadow: 2px 2px 5px rgba(0,0,0,0.1);
    border: 3px solid var(--text-color, #353535); /* 添加一个3px的深色描边 */
    filter: drop-shadow(3px 3px 0px rgba(0, 0, 0, 0.7));
}

.date {
    font-size: 12px;
    color: gray;
    align-self: flex-end;
    z-index: 2;
}

.edit-btn {
    padding: 8px 14px;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    background: var(--splat-green);
    color: var(--text-color);
    font-weight: bold;
    transition: all 0.2s;
    transform: rotate(1.5deg);
    border: 3px solid var(--text-color, #353535); /* 同样添加描边 */
    filter: drop-shadow(3px 3px 0px rgba(0, 0, 0, 0.7)); /* 同样添加硬阴影 */
}

.edit-btn:hover {
    transform: translate(0, -2px) rotate(1.5deg) scale(1.05);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    filter: drop-shadow(4px 4px 0px var(--splat-pink));
}
</style>