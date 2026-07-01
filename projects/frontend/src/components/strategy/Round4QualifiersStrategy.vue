<template>
  <div v-if="hasCompleted">
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
        <SelectCountries v-if="scoreEntered && model.entry && countries.length" v-model="model" :countries="countries"
          :tip-strategy="props.tipStrategy" :is-closed="hasCompleted" :max="16" label="">
        </SelectCountries>
        <div v-else-if="scoreEntered">
          <span class="text-h6">None</span>
        </div>
      </q-tab-panel>
      <q-tab-panel name="yours">
        <div v-if="!userEntered">
          <span class="text-h6">
            You didn't enter
          </span>
        </div>
        <SelectCountries v-if="userEntered && dummyModel.entry && countries.length" v-model="dummyModel"
          :countries="countries" :tip-strategy="props.tipStrategy" :is-closed="hasCompleted" :max="16" label="">
        </SelectCountries>
      </q-tab-panel>

    </q-tab-panels>
  </div>

  <SelectCountries v-if="countries.length && !hasCompleted" :tipStrategy="props.tipStrategy" :max="4"
    :countries="countries" :is-closed="hasCompleted" label="Select 4 countries" v-model="model"
    :for-result="props.forResult">
  </SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import SelectCountries from './SelectCountries.vue';
import { useUserStore } from 'src/stores/user-store';
import CountryGroupClient from 'src/api/v1/clients/CountryGroupClient';
import { onMounted, ref } from 'vue';
import type { Country } from 'src/api/Country';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean, forResult?: boolean }>()
const countries = ref<Country[]>([])
const [model, _] = defineModel<Strategy>({ required: true })
const userStore = useUserStore()
const countryGroupClient = CountryGroupClient(userStore.authHeader(), props.tipStrategy.tournamentId)
const hasCompleted = (props.isClosed || props.tipStrategy?.completed) && !props.forResult

const completedViewTabs = ref('result')
const dummyModel = ref<{ kind: "round_4_qualifiers", "entry": string[] }>({ kind: 'round_4_qualifiers', entry: [] });
const userEntered = ref(false)
const scoreEntered = ref(false);


onMounted(async () => {
  const params = new URLSearchParams({ filter: 'still-in', with: 'country' })
  // if (props.forResult) {
  //   params = new URLSearchParams({ with: 'country' })
  // }
  const response = await countryGroupClient.all(params)
  if (response.data) {
    response.data.forEach((entry) => {
      const country = entry.country as Country
      if (country) {
        countries.value.push(country)
      }
    })
  }

  if (props.tipStrategy.result?.strategyResults) {
    const resultEntry = props.tipStrategy.result.strategyResults.find((entry) => entry.kind === 'round_4_qualifiers')
    if (resultEntry) {
      model.value.entry = resultEntry.entry
      scoreEntered.value = hasCompleted
    }
  }
})
</script>
