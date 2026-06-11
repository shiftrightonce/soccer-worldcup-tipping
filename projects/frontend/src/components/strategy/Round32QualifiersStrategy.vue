<template>
  <SelectCountries v-if="isReady" :tipStrategy="props.tipStrategy" :max="32" :countries="countries"
    :is-closed="props.isClosed" label="Select 32 countries" v-model="model"></SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import SelectCountries from './SelectCountries.vue';
import { useUserStore } from 'src/stores/user-store';
import CountryGroupClient from 'src/api/v1/clients/CountryGroupClient';
import { ref, onMounted } from 'vue';
import type { Country } from 'src/api/Country';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean, forResult?: boolean }>()

const countries = ref<Country[]>([])
const [model, _] = defineModel<Strategy>({ required: true })
const userStore = useUserStore()
const countryGroupClient = CountryGroupClient(userStore.authHeader(), props.tipStrategy.tournamentId)
const isReady = ref(false)

onMounted(async () => {
  let params = new URLSearchParams({ filter: 'still-in', with: 'country' })
  if (props.forResult) {
    params = new URLSearchParams({ with: 'country' })
  }

  const response = await countryGroupClient.all(params)
  if (response.data) {
    response.data.forEach((entry) => {
      const country = entry.country as Country
      if (country) {
        country.name = country.name + ' - ' + entry.group?.name;
        countries.value.push(country)
      }
    })
  }

  if (props.tipStrategy.result?.strategyResults) {
    const resultEntry = props.tipStrategy.result.strategyResults.find((entry) => entry.kind === 'round_32_qualifiers')
    if (resultEntry) {
      model.value.entry = resultEntry.entry
    }
  }
  isReady.value = true
})
</script>
