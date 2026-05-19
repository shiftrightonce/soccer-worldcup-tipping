<template>
  <SelectCountries
    :tipStrategy="props.tipStrategy"
    :max="2"
    :countries="countries"
    label="Select 2 Countries"
    v-model="model"
  ></SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import { useCountryStore } from 'src/stores/country-store';
import SelectCountries from './SelectCountries.vue';

const props = defineProps<{tipStrategy: TipStrategy}>()
const countries = useCountryStore().countriesList
const [model, _] = defineModel<Strategy>({required: true })

</script>
