<template>
    <div class="circle"></div>
</template>

<script setup lang="ts">
import { defineProps, withDefaults, ref } from 'vue';

const props = withDefaults(defineProps<{
    size: number;
    positionType: 'static' | 'relative' | 'absolute' | 'fixed' | 'sticky';
    bgColor: string;
    posX: string;
    posY: string;
    zIndex?: number;
}>(), {
    size: 10,
    positionType: 'absolute',
    bgColor: '#42b983',
    posX: '0px',
    posY: '0px',
    zIndex: 0,
});



// 颜色格式验证
if (!/^#([0-9A-F]{3}){1,2}$/i.test(props.bgColor) && 
    !/^rgb\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*\)$/.test(props.bgColor) &&
    !/^rgba\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*[0-1](\.\d+)?\s*\)$/.test(props.bgColor)) {
    console.warn(`InkSpot 组件：无效的颜色格式 ${props.bgColor}，已使用默认色 #42b983`);
}
</script>

<style scoped>
.circle {
    z-index: v-bind(zIndex);
    width: v-bind(size + 'px');
    height: v-bind(size + 'px');
    border-radius: 50%;
    background-color: v-bind(bgColor);
    position: v-bind(positionType);
    left: v-bind(posX);
    top: v-bind(posY);
    transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    transform: translate(-50%, -50%);
    
    /* 关键：确保元素可以接收鼠标事件 */
    pointer-events: auto;
    /* 确保没有透明区域导致鼠标检测问题 */
    opacity: 1;
}

/* 提高hover样式优先级 */
.circle:hover {
    transform: translate(-50%, -50%) scale(1.05) !important;
    cursor: pointer !important;
}

/* 可选：添加激活状态样式，便于测试 */
.circle:active {
    transform: translate(-50%, -50%) scale(1) !important;
}
</style>
