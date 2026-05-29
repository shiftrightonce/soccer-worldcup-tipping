<template>
  <div>
    <div class="text-body2 text-muted-foreground center" v-if="!hasCompleted">
      Drag to reorder ranking
    </div>
    <div v-if="hasCompleted">
      <SelectCountries v-model="model" :countries="list" :tip-strategy="props.tipStrategy" :is-closed="hasCompleted"
        :max="4" label="">
      </SelectCountries>
    </div>
    <div v-else ref="el">
      <div v-for="aCountry in list" :key="aCountry.id">
        <q-card class="q-mb-sm" bordered>
          <q-item>
            <q-item-section avatar>
              <q-avatar square>
                <span :class='["fi", "fi-" + aCountry.alpha2.toLowerCase()]'></span>
              </q-avatar>
            </q-item-section>
            <q-item-section>
              <q-item-label>
                {{ aCountry.name }}
              </q-item-label>
            </q-item-section>
          </q-item>
        </q-card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSortable } from '@vueuse/integrations/useSortable';
// import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import { onMounted, shallowRef, useTemplateRef, watch } from 'vue'
import CountryGroupClient from 'src/api/v1/clients/CountryGroupClient';
import type { Country } from 'src/api/Country';
import { useUserStore } from 'src/stores/user-store';
import SelectCountries from './SelectCountries.vue';

const userStore = useUserStore()
const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean, forResult?: boolean }>()
const [model, _] = defineModel<{ "kind": "group_ranking"; "entry": string[] }>({ required: true, default: () => ({ kind: 'group_ranking', entry: [] }) })
const hasCompleted = (props.isClosed || props.tipStrategy?.completed) && !props.forResult
const countryGroupClient = CountryGroupClient(userStore.authHeader(), props.tipStrategy.tournamentId)

const el = useTemplateRef('el')
const list = shallowRef<Country[]>([])


watch(list, (newValues, _) => {
  model.value.entry = newValues.map((entry) => entry.id)
})


const { start, stop, option } = useSortable(el, list);
option('animation', 120);


if (!hasCompleted) {
  start()
} else {
  stop()
}

onMounted(async () => {
  const groupCountries = ((await countryGroupClient.byGroupId(props.tipStrategy.groupId || '')).data || [])
  const countries = groupCountries.map((entry) => entry.country as Country);

  if (props.forResult && props.tipStrategy.result?.strategyResults) {
    list.value = (props.tipStrategy.result?.strategyResults.find((entry) => entry.kind === 'group_ranking')?.entry || []).map((id) => countries.find((entry) => entry.id == id) as Country)
  } else {

    if (props.tipStrategy.groupId && model.value.entry.length === 0) {
      list.value = countries
    } else if (props.tipStrategy.groupId) {

      list.value = model.value.entry.map((id) => countries.find((entry) => entry.id == id) as Country)
    }
  }
})

</script>
