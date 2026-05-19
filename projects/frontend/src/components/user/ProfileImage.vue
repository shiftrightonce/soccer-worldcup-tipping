<template>
  <q-img v-if="userStore.user" :src="userStore.avatar" :width="width" :height="height"> </q-img>
</template>

<script setup lang="ts">
import { useUserStore } from 'src/stores/user-store';
const { width = '38px', height = '38px' } = defineProps<{
  width?: string;
  height?: string;
}>();

const userStore = useUserStore();
</script>
