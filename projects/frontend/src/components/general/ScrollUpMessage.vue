<template>
  <div class="col cursor-pointer q-pa-sm bg-accent text-white text-center">
    {{ message }}
  </div>
</template>

<script setup lang="ts">
const { message = 'Scroll back up' } = defineProps<{ message?: string }>()
</script>
