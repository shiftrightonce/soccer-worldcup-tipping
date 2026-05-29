<template>
  <SelectCountries :tipStrategy="props.tipStrategy" :max="8" :countries="countries" :is-closed="props.isClosed"
    label="Select 8 countries" v-model="model"></SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import SelectCountries from './SelectCountries.vue';
import { useUserStore } from 'src/stores/user-store';
import CountryGroupClient from 'src/api/v1/clients/CountryGroupClient';
import type { Country } from 'src/api/Country';
import { onMounted, ref } from 'vue';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean }>()
const countries = ref<Country[]>([])
const [model, _] = defineModel<Strategy>({ required: true })
const userStore = useUserStore()
const countryGroupClient = CountryGroupClient(userStore.authHeader(), props.tipStrategy.tournamentId)

onMounted(async () => {
  let params = new URLSearchParams({ filter: 'still-in', with: 'country' })
  if (props.tipStrategy.result?.strategyResults) {
    params = new URLSearchParams({ with: 'country' })
  }
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
    const resultEntry = props.tipStrategy.result.strategyResults.find((entry) => entry.kind === 'round_8_qualifiers')
    if (resultEntry) {
      model.value.entry = resultEntry.entry
    }
  }
})
</script>
