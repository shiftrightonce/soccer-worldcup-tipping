<template>
  <transition appear enter-active-class="animated fadeInUpBig" leave-active-class="animated fadeInDownBig">
    <!-- <q-chat-message :name="(data.from.id === userStore.user?.id) ? 'me' : data.from.username" :avatar="data.from.avatar"
      :text="[data.message]" :stamp="data.createdAt" :sent="data.from.id === userStore.user?.id" /> -->
  </transition>
</template>

<script setup lang="ts">
// import { ChatMessage } from 'src/stores/chat-store'
// import { useUserStore } from 'src/stores/user-store'

// const props = defineProps<{ data: string, from: boolean }>()

</script>
