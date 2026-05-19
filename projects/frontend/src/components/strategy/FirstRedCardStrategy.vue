<template>
  <SelectCountries
    v-model="model"
    :countries="countries"
    :tip-strategy="props.tipStrategy"
    :max="1"
    label="First to get a red card"
  >

  </SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import SelectCountries from './SelectCountries.vue';
import { useCountryStore } from 'src/stores/country-store';

const props = defineProps<{tipStrategy: TipStrategy}>()
const [model, _] = defineModel<Strategy>({required: true })
const countries = useCountryStore().countries

</script>
