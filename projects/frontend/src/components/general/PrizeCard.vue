<template>
  <q-card flat bordered class="my-card">
    <q-card-section>
      <div class="text-h6">{{ prize.title }}</div>
    </q-card-section>

    <q-card-section class="q-pt-none">
      <q-img :src="prize.image" />
    </q-card-section>

    <q-separator inset v-if="prize.description" />

    <q-card-section v-if="prize.description">
      {{ prize.description }}
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
const { prize } = defineProps<{ prize: { image: string, title: string, position: number, description: string } }>()

</script>

<style scoped></style>
