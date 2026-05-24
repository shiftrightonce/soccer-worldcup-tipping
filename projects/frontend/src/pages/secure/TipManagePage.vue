<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10 col-xs-12">
        <div class="text-h6">Strategy Label goes here</div>
        <div class="text-body2 text-muted-foreground">Strategy Description goes here</div>
      </div>
    </div>
    <q-list bordered>
      <q-expansion-item v-for="type in tipStrategy.strategyTypes" :key="type" group="somegroup" icon="explore"
        :label=strategyTypeKeyValue[type] default-opened>
        <q-card>
          <q-card-section class="q-pa-md">
            {{ strategyData[type] }}
            <component :is="strategyComponents[type]" v-model="strategyData[type]" :tip-strategy="tipStrategy">
            </component>
          </q-card-section>
        </q-card>
      </q-expansion-item>
    </q-list>
  </q-page>
</template>

<script setup lang="ts">
import { strategyTypeKeyValue, strategyTypeList } from 'src/general/lists';
import { strategyComponents } from 'src/components/strategy/.'
import { strategiesToKeyValue } from 'src/general/strategy_helper'
import { ref, watch } from 'vue';
import type { TipStrategy } from 'src/api/TipStrategy';
import type { Tip } from 'src/api/Tip';
import { useCountryStore } from 'src/stores/country-store';

const props = defineProps<{ tournamentId: string, tipStrategyId: string, id?: string }>()
// test data for now
const countries = useCountryStore().countriesList
const tipStrategy = ref<TipStrategy>({
  id: props.tipStrategyId,
  tournamentId: props.tournamentId,
  gameId: 'game1',
  groupId: null,
  label: 'Dummy Tip Strategy',
  description: 'Dummy Tip Strategy',
  group: null,
  game: {
    id: 'game1',
    count: 1,
    countryA: countries[0] || null,
    countryB: countries[1] || null,
    countryAGoals: 0,
    countryBGoals: 0,
    countryAId: countries[0]?.id || '',
    countryBId: countries[1]?.id || '',
    stage: 'group',
    label: `${countries[0]?.name} vs ${countries[1]?.name}`,
    countryAPenaltyGoals: 0,
    countryBPenaltyGoals: 0,
    createdAt: null,
    deletedAt: null,
    updatedAt: null,
    penalty: false,
    status: 'open',
    toConfigureOn: null,
    tournament: null,
    tournamentId: props.tournamentId,
    winner: null,
    winnerId: '',
  },
  tournament: null,
  opensAt: null,
  endsAt: null,
  calculatePointsOn: null,
  completed: false,
  strategyTypes: strategyTypeList.sort((a, b) => a.toString().localeCompare(b.toString())),
  createdAt: null,
  updatedAt: null,
  deletedAt: null,
})

// test data for now
const tip = ref<Tip>({
  id: props.id || 'dummy-tip',
  tournamentId: tipStrategy.value.tournamentId,
  tipStrategyId: tipStrategy.value.id,
  userId: 'user123',
  points: 0,
  tipStrategyPts: [],
  tipStrategy: null,
  tournament: null,
  strategies: tipStrategy.value.strategyTypes.map((type) => {
    return JSON.parse(JSON.stringify(strategiesToKeyValue[type]))
  }),
  createdAt: null,
  updatedAt: null,
  deletedAt: null
})

// built from existing data or new
const strategyData = ref(Object.fromEntries(tipStrategy.value.strategyTypes.map((type) => {
  return JSON.parse(JSON.stringify([type, strategiesToKeyValue[type]]))
})))

if (props.id) {
  strategyData.value = Object.fromEntries(tip.value.strategies.map((entry) => [entry.kind, entry]))
}


watch(strategyData, (_, newData) => {
  console.log('new data', { ...newData })
})


// TO DO

// 1. Fetch the Tipstrategy by ID
// 2. Sort the tip strategies

</script>
