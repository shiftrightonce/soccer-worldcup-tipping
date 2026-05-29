<template>
  <SelectCountries :tip-strategy="props.tipStrategy" :max="1" :countries="countries" :is-closed="props.isClosed"
    label="Winner" v-model="model">
  </SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import SelectCountries from './SelectCountries.vue';
import { ref, onMounted } from 'vue';
import type { Country } from 'src/api/Country';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean }>()
const countries = ref<Country[]>([])
const [model, _] = defineModel<Strategy>({ required: true })

onMounted(() => {
  countries.value.push(props.tipStrategy.game?.countryA as Country)
  countries.value.push(props.tipStrategy.game?.countryB as Country)

  if (props.tipStrategy.result?.strategyResults) {
    const resultEntry = props.tipStrategy.result.strategyResults.find((entry) => entry.kind === 'winner')
    if (resultEntry) {
      model.value.entry = resultEntry.entry
    }
  } else if (!model.value.entry) {
    model.value.entry = countries.value[Math.floor(Math.random() * countries.value.length)]?.id as string
  }
})

</script>
