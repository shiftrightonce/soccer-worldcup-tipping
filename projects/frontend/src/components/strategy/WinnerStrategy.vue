<template>
  <SelectCountries
    :tip-strategy="props.tipStrategy"
    :max="1"
    :countries="countries"
    label="Winner"
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
