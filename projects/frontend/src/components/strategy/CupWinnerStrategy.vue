<template>
    <select-countries
    v-model="model"
    :countries="countries"
    :tip-strategy="props.tipStrategy"
    :max="1"
    label="Country to win the tournament"
    ></select-countries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import { useCountryStore } from 'src/stores/country-store';
import SelectCountries from './SelectCountries.vue';

const props = defineProps<{tipStrategy: TipStrategy}>()
const [model, _] = defineModel<Strategy>({required: true })
const countries = useCountryStore().countries


console.log('cup winner', {
  ...props
});

</script>
