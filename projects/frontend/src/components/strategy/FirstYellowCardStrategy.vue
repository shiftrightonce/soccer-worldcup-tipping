<template>
  <SelectCountries v-model="model" :countries="countries" :tip-strategy="props.tipStrategy" :max="1"
    :is-closed="props.isClosed" label="First to get a yellow card">
  </SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import SelectCountries from './SelectCountries.vue';
import type { Country } from 'src/api/Country';
import { ref } from 'vue';
import { onMounted } from 'vue';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean }>()
const [model, _] = defineModel<Strategy>({ required: true })
const countries = ref<Country[]>([])

onMounted(() => {

  countries.value.push(props.tipStrategy.game?.countryA as Country)
  countries.value.push(props.tipStrategy.game?.countryB as Country)

  if (props.tipStrategy.result?.strategyResults) {
    const resultEntry = props.tipStrategy.result.strategyResults.find((entry) => entry.kind === 'first_yellow_card')
    if (resultEntry) {
      model.value.entry = resultEntry.entry
    }
  } else if (!model.value.entry) {
    model.value.entry = countries.value[Math.floor(Math.random() * countries.value.length)]?.id as string
  }
})

</script>
