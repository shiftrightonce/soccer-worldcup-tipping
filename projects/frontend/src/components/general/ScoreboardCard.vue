<template>

  <q-card class="q-mb-md q-mr-md-md" flat bordered :class="{ 'bg-positive': currentUser }">
    <q-item>
      <q-item-section>
        <q-item-label>{{ (currentUser) ? 'Your Score' : point.username }}</q-item-label>
        <q-item-label caption>
          Position
          <q-badge :label="point.position" color="red" />
        </q-item-label>
      </q-item-section>
      <q-space />
      <q-btn flat round icon="visibility" v-show="false" @click="viewClosedPlacedTips"></q-btn>
    </q-item>
    <q-separator />
    <q-card-section horizontal>
      <q-card-section class="col-3">
        <q-avatar square size="100px" font-size="82px">
          <q-img :src="'/public/avatar/' + point.avatar" />
        </q-avatar>
      </q-card-section>
      <!-- <q-separator vertical /> -->
      <q-card-section class="col-8">
        <q-list dense>
          <q-item>
            <q-item-section>
              <q-item-label>Total Points</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-item-label>{{ point.totalPoints }}</q-item-label>
            </q-item-section>
          </q-item>
          <q-item>
            <q-item-section>
              <q-item-label>Total Tips</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-item-label>{{ point.totalTips }}</q-item-label>
            </q-item-section>
          </q-item>
        </q-list>
      </q-card-section>
    </q-card-section>
  </q-card>

</template>

<script setup lang="ts">
import type { Point } from 'src/api/Point';

const { point, currentUser } = defineProps<{ point: Point, currentUser?: boolean }>()

function viewClosedPlacedTips () {
}

</script>
