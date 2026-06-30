<template>
  <select-countries v-model="model" v-if="countries" :countries="countries" :tip-strategy="props.tipStrategy" :max="1"
    :is-closed="props.isClosed" :for-result="props.forResult" label="Country to win the tournament"></select-countries>
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
const [model, _] = defineModel<Strategy>({ required: true })
const countries = ref<Country[]>([])
const userStore = useUserStore()
const countryGroupClient = CountryGroupClient(userStore.authHeader(), props.tipStrategy.tournamentId)


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
        countries.value.push(country)
      }
    })

    if (props.forResult) {
      const resultEntry = props.tipStrategy.result?.strategyResults?.find((entry) => entry.kind === 'cup_winner')
      if (resultEntry) {
        model.value.entry = resultEntry.entry
      }
    } else if (!model.value.entry && countries.value.length > 0) {
      model.value.entry = countries.value[Math.floor(Math.random() * countries.value.length)]?.id || ''
    }
  }
})


</script>
