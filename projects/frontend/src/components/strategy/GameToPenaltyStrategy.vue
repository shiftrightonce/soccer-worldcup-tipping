<template>
  <div v-if="hasCompleted">Game will go into penalty kickoff: {{ model.entry ? 'Yes' : 'No'  }}</div>
    <q-checkbox v-else v-model="model.entry" left-label label="Game will go into penalty kickoff"  />
</template>

<script setup lang="ts">

import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';

const props = defineProps<{tipStrategy: TipStrategy}>()
const [model, _] = defineModel<Strategy>({required: true })
const hasCompleted = props.tipStrategy?.completed // or end date has passed

</script>
