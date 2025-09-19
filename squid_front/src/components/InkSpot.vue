<template>
    <div class="splat-container" :style="containerStyle">
        <svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
            <path :fill="bgColor" :d="selectedPath" transform="translate(100 100)"  :style="{ transform: randomTransform }" />
        </svg>
    </div>
</template>

<script setup lang="ts">
import { defineProps, withDefaults, computed, ref, onMounted } from 'vue';

type PositionType = 'static' | 'relative' | 'absolute' | 'fixed' | 'sticky';


// 1. 定义一个包含多个 SVG 路径数据的数组
const splatPaths = [
    "M25,-43.2C36.4,-36.8,52.1,-38,55.2,-32.1C58.2,-26.1,48.6,-13.1,44.9,-2.1C41.2,8.8,43.4,17.6,37.7,18.9C31.9,20.2,18.2,14,10.6,15.6C3,17.2,1.5,26.5,-4.3,34C-10.1,41.4,-20.3,47,-24.4,43.5C-28.6,39.9,-26.7,27.1,-37.7,18.4C-48.7,9.6,-72.4,4.8,-78.8,-3.7C-85.2,-12.2,-74.2,-24.3,-66.2,-38.3C-58.3,-52.2,-53.3,-68,-42.7,-74.7C-32.2,-81.5,-16.1,-79.3,-4.6,-71.3C6.9,-63.3,13.7,-49.5,25,-43.2Z",
    "M38.5,-59.3C46.7,-62,48,-45.2,44.8,-32.1C41.6,-19,33.9,-9.5,39.9,3.4C45.8,16.4,65.5,32.8,64.5,38.6C63.5,44.4,41.9,39.8,27.7,45.1C13.5,50.4,6.8,65.8,1.7,62.8C-3.3,59.8,-6.5,38.4,-15.9,30.2C-25.2,22.1,-40.7,27.2,-46.4,24.4C-52.1,21.6,-48.1,10.8,-50,-1.1C-51.8,-12.9,-59.5,-25.9,-58.4,-36.7C-57.3,-47.5,-47.3,-56.1,-36.1,-51.7C-24.9,-47.4,-12.5,-29.9,1.3,-32.3C15.2,-34.6,30.3,-56.7,38.5,-59.3Z",
    "M29,-41.4C42.9,-42.3,63,-45.1,62.6,-38.6C62.2,-32,41.3,-16,35.7,-3.2C30.1,9.5,39.7,19,41,27.2C42.4,35.4,35.4,42.3,27.2,49C18.9,55.7,9.5,62.2,-3,67.3C-15.4,72.4,-30.8,76.2,-43,71.8C-55.2,67.4,-64.1,54.8,-68.7,41.5C-73.2,28.1,-73.4,14.1,-66.5,4C-59.5,-6.1,-45.5,-12.1,-33.8,-13.1C-22.1,-14.1,-12.8,-10,-7.5,-14.1C-2.3,-18.2,-1.1,-30.5,3.2,-36.1C7.6,-41.7,15.2,-40.5,29,-41.4Z",
];

// 2. 创建一个 ref 来存储当前组件实例选择的路径
const selectedPath = ref(splatPaths[0]); // 默认给一个值

const randomTransform = ref('');
// 3. 使用 onMounted 生命周期钩子，在组件挂载时执行随机选择
onMounted(() => {
    const randomIndex = Math.floor(Math.random() * splatPaths.length);
    let transforms = [];
    if (Math.random() > 0.5) {
        transforms.push('scaleX(-1)');
    }
    // 50% 的概率垂直翻转
    if (Math.random() > 0.5) {
        transforms.push('scaleY(-1)');
    }
    randomTransform.value = transforms.join(' ');
    selectedPath.value = splatPaths[randomIndex];
});


const props = withDefaults(defineProps<{
    size?: number;
    bgColor?: string;
    posX?: string;
    positionType?: PositionType;
    posY?: string;
    zIndex?: number;
    rotation?: number;
}>(), {
    size: 100,
    positionType: 'absolute',
    bgColor: '#42d392',
    posX: '0px',
    posY: '0px',
    zIndex: 0,
    rotation: 0,
});

const containerStyle = computed(() => ({
    width: `${props.size}px`,
    height: `${props.size}px`,
    position: props.positionType,
    left: props.posX,
    top: props.posY,
    zIndex: props.zIndex,
    // 你仍然可以结合随机的 CSS 旋转，让形态更丰富
    transform: `translate(-50%, -50%) rotate(${props.rotation}deg)`,
}));
</script>

<style scoped>
/* 样式保持不变 */
.splat-container {
    transition: transform 0.2s ease-out;
}

.splat-container:hover {
    transform: translate(-50%, -50%) scale(1.1) rotate(calc(var(--rotation, 0deg) + 5deg));
    cursor: pointer;
}

svg {
    overflow: visible;
    width: 100%;
    height: 100%;
}
</style>