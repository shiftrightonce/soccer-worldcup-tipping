<template>
  <div v-if="hasCompleted">
    <q-tabs v-model="completedViewTabs" dense class="text-grey" active-color="primary" indicator-color="primary"
      align="justify" narrow-indicator>
      <q-tab name="result" label="Result" />
      <q-tab name="yours" label="Yours" />
    </q-tabs>
    <q-separator />

    <q-tab-panels keep-alive v-model="completedViewTabs" animated>

      <q-tab-panel name="result">
        <div v-if="!scoreEntered">
          <span class="text-h6">Result pending</span>
        </div>
        <div v-if="!model.entry && scoreEntered">
          <span class="text-h6">Draw</span>
        </div>
        <SelectCountries v-if="model.entry && countries.length" v-model="model" :countries="countries"
          :tip-strategy="props.tipStrategy" :is-closed="hasCompleted" :max="1" label="">
        </SelectCountries>
      </q-tab-panel>

      <q-tab-panel name="yours">
        <div v-if="!userEntered">
          <span class="text-h6">
            You didn't enter
          </span>
        </div>
        <div v-if="!dummyModel.entry && userEntered">
          <span class="text-h6">
            Draw
          </span>
        </div>
        <SelectCountries v-if="dummyModel.entry && countries.length" v-model="dummyModel" :countries="countries"
          :tip-strategy="props.tipStrategy" :is-closed="hasCompleted" :max="1" label="">
        </SelectCountries>
      </q-tab-panel>
    </q-tab-panels>
  </div>
  <SelectCountries v-if="!hasCompleted && countries.length" :tip-strategy="props.tipStrategy"
    :for-result="props.forResult" :max="1" :countries="countries" :is-closed="props.isClosed" label="Winner"
    v-model="model">
  </SelectCountries>
</template>

<script setup lang="ts">
import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import SelectCountries from './SelectCountries.vue';
import { ref, onMounted } from 'vue';
import type { Country } from 'src/api/Country';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean, forResult?: boolean }>()
const countries = ref<Country[]>([])
const [model, _] = defineModel<Strategy>({ required: true })

const hasCompleted = (props.isClosed || props.tipStrategy?.completed) && !props.forResult
const completedViewTabs = ref('result')
const dummyModel = ref<{ kind: "winner", "entry": string }>({ kind: 'winner', entry: '' });
const userEntered = ref(false)
const scoreEntered = ref(false);

onMounted(() => {

  if (props.tipStrategy.game && props.tipStrategy.game.count <= 72) {
    countries.value.push({
      id: '',
      name: 'None',
      alpha2: 'none',
      alpha3: 'none',
      tournaments: null,
      groups: null,
      coutryGroup: null,
      updatedAt: null,
      deletedAt: null,
      createdAt: null
    });
  }
  countries.value.push(props.tipStrategy.game?.countryA as Country)
  countries.value.push(props.tipStrategy.game?.countryB as Country)

  if (hasCompleted || props.tipStrategy.result?.strategyResults) {
    const resultEntry = props.tipStrategy.result?.strategyResults.find((entry) => entry.kind === 'winner')
    if (resultEntry) {
      model.value.entry = resultEntry.entry
      scoreEntered.value = hasCompleted;
    }
  } else if (!model.value.entry && props.tipStrategy.game && props.tipStrategy.game.count > 72) {
    model.value.entry = countries.value[Math.floor(Math.random() * countries.value.length)]?.id as string
  }

  if (hasCompleted && props.tipStrategy.tips) {
    const winner = props.tipStrategy.tips[0]?.strategies.find((entry) => entry.kind == 'winner');
    if (winner) {
      dummyModel.value = winner
      userEntered.value = true;
    }
  }

})

</script>
