<template>
  <SelectCountries :tipStrategy="props.tipStrategy" :max="2" :countries="countries" :is-closed="props.isClosed"
    label="Countries to make it to the final" v-model="model"></SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import SelectCountries from './SelectCountries.vue';
import { onMounted, ref } from 'vue';
import type { Country } from 'src/api/Country';
import { useUserStore } from 'src/stores/user-store';
import CountryGroupClient from 'src/api/v1/clients/CountryGroupClient';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean }>()
const countries = ref<Country[]>([])
const [model, _] = defineModel<Strategy>({ required: true })
const userStore = useUserStore()
const countryGroupClient = CountryGroupClient(userStore.authHeader(), props.tipStrategy.tournamentId)

onMounted(async () => {
  const params = new URLSearchParams({ filter: 'still-in', with: 'country' })
  const response = await countryGroupClient.all(params)
  if (response.data) {
    response.data.forEach((entry) => {
      const country = entry.country as Country
      if (country) {
        countries.value.push(country)
      }
    })

    if (!model.value.entry && countries.value.length > 0) {
      model.value.entry = countries.value[Math.floor(Math.random() * countries.value.length)]?.id || ''
    }
  }
})

</script>
