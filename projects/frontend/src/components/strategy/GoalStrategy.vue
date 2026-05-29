<template>
  <div v-if="hasCompleted" class="row">
    <div class="col">
      Country A : {{ model.entry.country_a_goals }}
    </div>
    <div class="col">
      Country B : {{ model.entry.country_b_goals }}
    </div>
  </div>
  <div v-else class="row">
    <div class="col q-pr-sm">
      <q-input outlined type="number" v-model="model.entry.country_a_goals"
        @update:model-value="(v) => model.entry.country_a_goals = Number(v)" :label="countryNames.a" />
    </div>
    <div class="col q-pl-sm">
      <q-input outlined type="number" v-model="model.entry.country_b_goals"
        @update:model-value="(v) => model.entry.country_b_goals = Number(v)" :label="countryNames.b" />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TipStrategy } from 'src/api/TipStrategy';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean, forResult?: boolean }>()
const [model, _] = defineModel<{ kind: "goals", entry: { country_a_goals: number, country_b_goals: number } }>({ required: true })
const hasCompleted = (props.isClosed || props.tipStrategy?.completed) && !props.forResult
const countryNames = { a: props.tipStrategy.game?.countryA?.name || 'Country A', b: props.tipStrategy.game?.countryB?.name || 'Country B' }

if (props.forResult) {
  const resultEntry = props.tipStrategy.result?.strategyResults?.find((entry) => entry.kind === 'goals')
  if (resultEntry) {
    model.value.entry = resultEntry.entry
  }
} else {

  if (!model.value.entry.country_a_goals) {
    model.value.entry.country_a_goals = 0
  }

  if (!model.value.entry.country_b_goals) {
    model.value.entry.country_b_goals = 0
  }
}


</script>
