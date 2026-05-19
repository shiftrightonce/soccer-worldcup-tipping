<template>
  <span :class="code" :style="{ width, height }"></span>
</template>

<script setup lang="ts">
const { countryCode, width, height } = defineProps({
  countryCode: { type: String, required: true },
  width: { type: String, default: '200px' },
  height: { type: String, default: '200px' },
});

const code = `fi fi-${countryCode.toLowerCase()}`;
</script>
