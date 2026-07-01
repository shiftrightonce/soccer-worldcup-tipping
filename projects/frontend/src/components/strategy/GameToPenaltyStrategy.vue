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
        <div else-if="scoreEntered">
          {{ model.entry ? 'Yes' : 'No' }}
        </div>
      </q-tab-panel>

      <q-tab-panel name="yours">
        <div v-if="!userEntered">
          <span class="text-h6">
            You didn't enter
          </span>
        </div>
        <div v-if="userEntered">
          {{ dummyModel.entry ? 'Yes' : 'No' }}
        </div>
      </q-tab-panel>
    </q-tab-panels>

  </div>
  <q-checkbox v-else v-model="model.entry" left-label label="Game will go into penalty kickoff" />
</template>

<script setup lang="ts">

import type { Strategy } from 'src/api/Strategy';
import type { TipStrategy } from 'src/api/TipStrategy';
import { onMounted, ref } from 'vue';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean, forResult?: boolean }>()
const [model, _] = defineModel<Strategy>({ required: true })
const hasCompleted = (props.isClosed || props.tipStrategy?.completed) && !props.forResult

const completedViewTabs = ref('result')
const dummyModel = ref<{ kind: "game_to_penalty", "entry": boolean }>({ kind: 'game_to_penalty', entry: false });
const userEntered = ref(false)
const scoreEntered = ref(false);

onMounted(() => {
  if (hasCompleted || props.tipStrategy.result?.strategyResults) {
    const resultEntry = props.tipStrategy.result?.strategyResults?.find((entry) => entry.kind === 'game_to_penalty')
    if (resultEntry) {
      model.value.entry = resultEntry.entry
      scoreEntered.value = hasCompleted
    }
  }

  if (hasCompleted && props.tipStrategy.tips) {
    const userEntry = props.tipStrategy.tips[0]?.strategies.find((entry) => entry.kind == 'game_to_penalty');
    if (userEntry) {
      dummyModel.value = userEntry
      userEntered.value = true;
    }
  }
})

</script>
