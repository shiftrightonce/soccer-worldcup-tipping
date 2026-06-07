<template>
  <q-page class="row items-center justify-evenly"></q-page>
</template>

<script setup lang="ts">
import { useUserStore } from 'src/stores/user-store';
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';

const userStore = useUserStore();
const router = useRouter();

onMounted(async () => {
  userStore.clearLoginData()
  await router.push({
    name: 'home'
  })
})

</script>
