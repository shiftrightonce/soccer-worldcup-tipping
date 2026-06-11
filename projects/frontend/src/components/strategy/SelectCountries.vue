<template>
  <div v-if="hasCompleted">
    <q-list bordered padding v-if="selectedCountries.length > 0">
      <div v-for="aCountry in selectedCountries" :key="aCountry.id">
        <q-item>
          <q-item-section avatar>
            <q-avatar square>
              <flag-component :country-code="aCountry.alpha2" width="25px" height="18px" />
            </q-avatar>
          </q-item-section>

          <q-item-section>
            <q-item-label lines="1">
              {{ aCountry.name }}
            </q-item-label>
          </q-item-section>
        </q-item>
      </div>
    </q-list>
  </div>
  <div v-else>
    <div v-if="max == 1">
      <q-select outlined v-model="model.entry" :options="countries" map-options emit-value option-value="id"
        option-label="name" :label="props.label">
        <template v-slot:option="scope">
          <q-item v-bind="scope.itemProps">
            <q-item-section avatar>
              <flag-component :country-code="scope.opt.alpha2" width="25px" height="18px" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ scope.opt.name }}</q-item-label>
            </q-item-section>
          </q-item>
        </template>
      </q-select>
    </div>
    <div v-else>
      <div class="text-subtitle1">{{ props.label }}</div>
      <q-list bordered padding>
        <div v-for="aCountry in countries" :key="aCountry.id">
          <q-item clickable v-ripple>
            <q-item-section avatar>
              <q-avatar square>
                <flag-component :country-code="aCountry.alpha2" width="25px" height="18px" />
              </q-avatar>
            </q-item-section>

            <q-item-section>
              <q-item-label lines="1">
                {{ aCountry.name }}
              </q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-toggle color="blue" v-show="!isDisabled[aCountry.id]" v-model="selectedValues[aCountry.id]"
                val="aCountry.id" checked-icon="check" unchecked-icon="clear"
                @update:model-value="() => onSelectToggle()" />
            </q-item-section>
          </q-item>
          <q-separator inset="item" />
        </div>

      </q-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Country } from 'src/api/Country';
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import { onMounted, reactive } from 'vue';
import FlagComponent from '../FlagComponent.vue';


const props = defineProps<{ tipStrategy: TipStrategy, max: number, countries: Country[], label: string, isClosed: boolean, forResult?: boolean }>()
const [model, _] = defineModel<Strategy>({ required: true })
const selectedValues = reactive<Record<string, boolean>>(Object.fromEntries(props.countries.map((entry) => [entry.id, false])))
const isDisabled = reactive<Record<string, boolean>>(Object.fromEntries(props.countries.map((entry) => [entry.id, false])))

const hasCompleted = (props.isClosed || props.tipStrategy?.completed) && !props.forResult
const selectedCountries = reactive<Country[]>([])

onMounted(() => {
  if (Array.isArray(model.value.entry)) {
    (model.value.entry.map((id) => props.countries.find((c) => c.id === id)) as Country[]).forEach((entry) => {
      selectedCountries.push(entry)
    });
    selectedCountries.forEach((entry) => {
      selectedValues[entry.id] = true
    })
  }


  if ((!model.value.entry || !Array.isArray(model.value.entry)) && props.max > 1) {
    model.value.entry = []
  }
})

const onSelectToggle = () => {
  model.value.entry = [];
  for (const k in selectedValues) {
    if (selectedValues[k]) {
      model.value.entry.push(k)
      if (model.value.entry.length >= props.max) {
        for (const aCountry of props.countries) {
          if (!selectedValues[aCountry.id]) {
            isDisabled[aCountry.id] = true
          }
        }
      } else {
        for (const key in isDisabled) {
          isDisabled[key] = false
        }
      }
    }
  }
}

</script>
