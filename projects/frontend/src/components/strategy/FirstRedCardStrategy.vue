<template>
  <div v-if="hasCompleted && isReady">
    <q-tabs v-model="completedViewTabs" dense class="text-grey" active-color="primary" indicator-color="primary"
      align="justify" narrow-indicator>
      <q-tab name="result" label="Result" />
      <q-tab name="yours" label="Yours" />
    </q-tabs>
    <q-separator />

    <q-tab-panels keep-alive v-model="completedViewTabs" animated>
      <q-tab-panel name="result">
        <div v-if="!scoreEntered">
          <span class="text-h6">Result pending</span>
        </div>
        <SelectCountries v-if="scoreEntered && model.entry" v-model="model" :countries="countries"
          :tip-strategy="props.tipStrategy" :is-closed="hasCompleted" :max="1" label="">
        </SelectCountries>
        <div v-if="scoreEntered && !model.entry">
          <span class="text-h6">None</span>
        </div>
      </q-tab-panel>
      <q-tab-panel name="yours">
        <div v-if="!userEntered">
          <span class="text-h6">
            You didn't enter
          </span>
        </div>
        <SelectCountries v-if="userEntered && dummyModel.entry" v-model="dummyModel" :countries="countries"
          :tip-strategy="props.tipStrategy" :is-closed="hasCompleted" :max="1" label="">
        </SelectCountries>
      </q-tab-panel>
    </q-tab-panels>
  </div>
  <SelectCountries v-model="model" v-if="!hasCompleted && isReady" :countries="countries"
    :tip-strategy="props.tipStrategy" :max="1" :is-closed="props.isClosed" :for-result="props.forResult"
    label="First to get a red card">
  </SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import SelectCountries from './SelectCountries.vue';
import type { Country } from 'src/api/Country';
import { onMounted, ref } from 'vue';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean, forResult?: boolean }>()
const [model, _] = defineModel<Strategy>({ required: true })
const countries = ref<Country[]>([])
const hasCompleted = (props.isClosed || props.tipStrategy?.completed) && !props.forResult
const completedViewTabs = ref('result')
const dummyModel = ref<{ kind: "first_red_card", "entry": string }>({ kind: 'first_red_card', entry: '' });
const userEntered = ref(false)
const scoreEntered = ref(false);
const isReady = ref(false);


onMounted(() => {
  countries.value.push({
    id: '',
    name: 'None',
    alpha2: 'none',
    alpha3: 'none',
    tournaments: null,
    groups: null,
    coutryGroup: null,
    updatedAt: null,
    deletedAt: null,
    createdAt: null
  });

  countries.value.push(props.tipStrategy.game?.countryA as Country)
  countries.value.push(props.tipStrategy.game?.countryB as Country)

  if (hasCompleted || props.tipStrategy.result?.strategyResults) {
    const resultEntry = props.tipStrategy.result?.strategyResults.find((entry) => entry.kind === 'first_red_card')
    if (resultEntry) {
      model.value.entry = resultEntry.entry
    }
    scoreEntered.value = hasCompleted;
  }

  if (hasCompleted && props.tipStrategy.tips) {
    const userEntry = props.tipStrategy.tips[0]?.strategies.find((entry) => entry.kind == 'first_red_card');
    if (userEntry) {
      dummyModel.value = userEntry
      userEntered.value = true;
    }
  }
  isReady.value = true;
})

</script>
