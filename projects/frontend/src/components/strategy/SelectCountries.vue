<template>
  <div v-if="hasCompleted">
    {{ model.entry }}
  </div>

   <div v-if="max == 1">
    <q-select
    outlined
    v-model="model.entry"
    :options="countries"
    option-value="id"
    option-label="name" :label="props.label" />
   </div>
   <div v-else>
    <q-list bordered padding>
      <q-item-label header>{{  props.label  }}</q-item-label>
      <div v-for="aCountry in countries" :key="aCountry.id">
      <q-item clickable v-ripple >
        <q-item-section avatar>
          <q-avatar square>
            <span :class='["fi", "fi-" + aCountry.alpha2.toLowerCase()]'></span>
          </q-avatar>
        </q-item-section>

        <q-item-section>
          <q-item-label lines="1">{{  aCountry.name  }}</q-item-label>
        </q-item-section>
        <q-item-section side>
          <q-toggle
            color="blue"
            v-model="selectedValues[aCountry.id]"
            val="aCountry.id" checked-icon="check" unchecked-icon="clear" @update:model-value="() => onSelectToggle()" />
        </q-item-section>
      </q-item>
      <q-separator inset="item" />
      </div>

    </q-list>
   </div>

</template>

<script setup lang="ts">
import type { Country } from 'src/api/Country';
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import { reactive } from 'vue';

const props = defineProps<{ tipStrategy: TipStrategy, max: number, countries: Country[], label: string}>()
const [model, _] = defineModel<Strategy>({required: true })
const selectedValues = reactive<Record<string, boolean>>(Object.fromEntries(props.countries.map((entry) => [entry.id, false])))

const hasCompleted = props.tipStrategy?.completed // or end date has passed



if ((!model.value.entry || !Array.isArray(model.value.entry)) && props.max > 1) {
  model.value.entry = []
}

const onSelectToggle = () => {
  model.value.entry = [];
  for (const k in selectedValues) {
    if (selectedValues[k]) {
         model.value.entry.push(k)
      }
  }
}

</script>
