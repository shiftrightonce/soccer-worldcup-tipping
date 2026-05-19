<template>
  <q-avatar class="q-mr-md" :size="size">
    <q-img :src="imgSrc" />
  </q-avatar>
</template>

<script setup lang="ts">
const { imgSrc = 'icons/favicon-128x128.png', size = '38px' } = defineProps<{
  imgSrc?: string;
  size?: string;
}>();
</script>
