<template>
    <div>
      <div class="p-mb-sm">
        {{ hasCompleted ? 'Group Ranking': 'Drag to reorder ranking' }}
      </div>
    <div ref="el">
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
import { useCountryStore } from 'src/stores/country-store';
import { shallowRef, useTemplateRef, watch } from 'vue'
const countries = useCountryStore().countriesList
const [model, _] = defineModel<{ "kind": "group_ranking"; "entry": string[] }>({required: true })
const props = defineProps<{tipStrategy: TipStrategy}>()
const hasCompleted = props.tipStrategy?.completed // or end date has passed


const el = useTemplateRef('el')
const list = shallowRef(countries.map((entry) => JSON.parse(JSON.stringify(entry))))

watch(list, (_, newValues) => {
  model.value.entry = newValues.map((entry)=> entry.id)
})

const { start, stop, option } = useSortable(el, list);
option('animation', 120);


if (!hasCompleted) {
   start()
} else {
  stop()
}

</script>
