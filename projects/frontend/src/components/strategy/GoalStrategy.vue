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

        <q-list bordered dense v-else>
          <q-item>
            <q-item-section avatar>
              <q-avatar square>
                <flag-component :country-code="props.tipStrategy.game?.countryA?.alpha2 || ''" width="25px"
                  height="18px" />
              </q-avatar>
            </q-item-section>
            <q-item-section>
              <q-item-label lines="1">
                {{ countryNames.a }}
              </q-item-label>
            </q-item-section>

            <q-item-section side>
              {{ model.entry.country_a_goals }}
            </q-item-section>
          </q-item>

          <q-item>
            <q-item-section avatar>
              <q-avatar square>
                <flag-component :country-code="props.tipStrategy.game?.countryB?.alpha2 || ''" width="25px"
                  height="18px" />
              </q-avatar>
            </q-item-section>
            <q-item-section>
              <q-item-label lines="1">
                {{ countryNames.b }}
              </q-item-label>
            </q-item-section>

            <q-item-section side>
              {{ model.entry.country_b_goals }}
            </q-item-section>
          </q-item>
        </q-list>
      </q-tab-panel>

      <q-tab-panel name="yours">
        <div v-if="!dummyModel.entry">
          <span class="text-h6">
            You didn't enter
          </span>
        </div>

        <q-list bordered dense v-else>
          <q-item>
            <q-item-section avatar>
              <q-avatar square>
                <flag-component :country-code="props.tipStrategy.game?.countryA?.alpha2 || ''" width="25px"
                  height="18px" />
              </q-avatar>
            </q-item-section>
            <q-item-section>
              <q-item-label lines="1">
                {{ countryNames.a }}
              </q-item-label>
            </q-item-section>

            <q-item-section side>
              {{ dummyModel.entry.country_a_goals }}
            </q-item-section>
          </q-item>

          <q-item>
            <q-item-section avatar>
              <q-avatar square>
                <flag-component :country-code="props.tipStrategy.game?.countryB?.alpha2 || ''" width="25px"
                  height="18px" />
              </q-avatar>
            </q-item-section>
            <q-item-section>
              <q-item-label lines="1">
                {{ countryNames.b }}
              </q-item-label>
            </q-item-section>

            <q-item-section side>
              {{ dummyModel.entry.country_b_goals }}
            </q-item-section>
          </q-item>
        </q-list>
      </q-tab-panel>
    </q-tab-panels>

  </div>
  <div v-else class="row">
    <div class="col q-pr-sm">
      <q-input outlined type="number" v-model="model.entry.country_a_goals"
        @update:model-value="(v) => model.entry.country_a_goals = Number(v)" :label="countryNames.a" />
    </div>
    <div class="col q-pl-sm">
      <q-input outlined type="number" v-model="model.entry.country_b_goals"
        @update:model-value="(v) => model.entry.country_b_goals = Number(v)" :label="countryNames.b" />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TipStrategy } from 'src/api/TipStrategy';
import FlagComponent from '../FlagComponent.vue';
import { ref } from 'vue';

const props = defineProps<{ tipStrategy: TipStrategy, isClosed: boolean, forResult?: boolean }>()
const [model, _] = defineModel<{ kind: "goals", entry: { country_a_goals: number, country_b_goals: number } }>({ required: true })
const hasCompleted = (props.isClosed || props.tipStrategy?.completed) && !props.forResult
const countryNames = { a: props.tipStrategy.game?.countryA?.name || 'Country A', b: props.tipStrategy.game?.countryB?.name || 'Country B' }
const completedViewTabs = ref('result')
const dummyModel = ref<{ kind: "goals", "entry": { country_a_goals: number, country_b_goals: number } }>({ kind: 'goals', entry: { country_b_goals: 0, country_a_goals: 0 } });
const scoreEntered = ref(false);


if (hasCompleted || props.forResult) {
  const resultEntry = props.tipStrategy.result?.strategyResults?.find((entry) => entry.kind === 'goals')
  if (resultEntry) {
    model.value.entry = resultEntry.entry
    scoreEntered.value = hasCompleted;
  }
} else {
  if (!model.value.entry.country_a_goals) {
    model.value.entry.country_a_goals = 0
  }

  if (!model.value.entry.country_b_goals) {
    model.value.entry.country_b_goals = 0
  }
}

if (hasCompleted && props.tipStrategy.tips) {
  const goals = props.tipStrategy.tips[0]?.strategies.find((entry) => entry.kind == 'goals');
  if (goals) {
    dummyModel.value = goals
  }
}


</script>
