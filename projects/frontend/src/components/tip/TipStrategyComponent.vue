<template>
  <q-dialog ref="dialogRef" @hide="onDialogHide" v-show="tipStrategy" persistent transition-show="scale"
    transition-hide="scale">
    <q-card class="q-dialog-plugin q-pt-md q-pb-md">
      <q-card-section>
        <div class="text-h6">
          {{ tipStrategy?.label }}
        </div>
      </q-card-section>
      <div class="row">
        <div class="col-12">
          <q-list bordered>
            <q-expansion-item v-for="(type, index) in tipStrategy?.strategyTypes" :default-opened="index == 0"
              :key="type" group="somegroup" icon="explore" :label=strategyTypeKeyValue[type]>
              <q-card>
                <q-card-section class="q-pa-md">
                  <div class="text-body2 text-muted-foreground">{{ tipStrategy?.description }}</div>
                  <component :is="strategyComponents[type]" v-model="strategyData[type]" :tip-strategy="tipStrategy"
                    :is-closed="isClosed" :forResult="props.forResult">
                  </component>
                </q-card-section>
              </q-card>
            </q-expansion-item>
          </q-list>
        </div>
      </div>


      <!-- buttons example -->
      <q-card-actions align="right">
        <q-btn color="secondary" flat label="Close" @click="onDialogCancel" />
        <q-btn color="primary" flat label="Save" @click="saveTip" v-if="!isClosed && !props.forResult" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { useDialogPluginComponent, useQuasar } from 'quasar'
import { strategyTypeKeyValue } from 'src/general/lists';
import { strategyComponents } from 'src/components/strategy/.'
import { ref, onMounted } from 'vue';
import type { TipStrategy } from 'src/api/TipStrategy';
import type { Tip } from 'src/api/Tip';
import type { TipPayload } from 'src/api/v1/TipPayload';
import TipStrategyClient, { makeNewResultPayload } from 'src/api/v1/clients/TipStrategyClient';
import { useUserStore } from 'src/stores/user-store';
import type { Strategy } from 'src/api/Strategy';
import { strategyFromType, validateStrategy } from 'src/general/strategy_helper';
import { useGameStore } from 'src/stores/game-store';
import TipClient, { makeNewPayload } from 'src/api/v1/clients/TipClient';

const props = defineProps<{ tournamentId: string, tipStrategyId: string, id?: string, forResult?: boolean }>()
const userStore = useUserStore()
const tipStrategyClient = TipStrategyClient(userStore.authHeader(), props.tournamentId)
const tipClient = TipClient(userStore.authHeader(), props.tournamentId)
const strategyData = ref<Record<string, Strategy>>({})
const isClosed = ref(false)
const q = useQuasar()
const gameStore = useGameStore()

// test data for now
const tipStrategy = ref<TipStrategy | null>(null)

// test data for now
const tip = ref<TipPayload>(makeNewPayload(props.tournamentId, props.tipStrategyId))



defineEmits([
  // REQUIRED; need to specify some events that your
  // component will emit through useDialogPluginComponent()
  ...useDialogPluginComponent.emits
])

const { dialogRef, onDialogHide, onDialogOK, onDialogCancel } =
  useDialogPluginComponent()
// dialogRef      - Vue ref to be applied to QDialog
// onDialogHide   - Function to be used as handler for @hide on QDialog
// onDialogOK     - Function to call to settle dialog with "ok" outcome
//                    example: onDialogOK() - no payload
//                    example: onDialogOK({ /*...*/ }) - with payload
// onDialogCancel - Function to call to settle dialog with "cancel" outcome

// this is part of our example (so not required)
function _onOKClick () {
  // on OK, it is REQUIRED to
  // call onDialogOK (with optional payload)
  onDialogOK()
  // or with payload: onDialogOK({ ... })
  // ...and it will also hide the dialog automatically
}

const saveTip = async () => {
  if (props.forResult) {
    const result = makeNewResultPayload()
    result.strategyResults = Object.values(strategyData.value)
    const response = await tipStrategyClient.saveResults(props.tipStrategyId, result)
    if (response.data) {
      onDialogOK(response.data)
    }
    return;
  }

  tip.value.strategies = Object.values(strategyData.value);
  for (const entry of tip.value.strategies) {
    const result = validateStrategy(entry)
    if (result !== true) {
      q.dialog({
        title: 'Wrong tip entry',
        message: result
      });
      return;
    }
  }
  const response = await tipClient.saveMyTip(tip.value, tip.value.id || '')
  if (response.data) {
    tip.value = response.data
    onDialogOK(response.data)
  }
}

onMounted(async () => {
  q.loading.show()
  // 1. Fetch the Tipstrategy by ID
  const response = await tipStrategyClient.byId(props.tipStrategyId, (!props.forResult) ? new URLSearchParams({ with: 'my_tips' }) : undefined)
  if (response.data) {
    response.data.strategyTypes = response.data.strategyTypes.sort((a, b) => a.toString().localeCompare(b.toString()))
    if (!props.forResult && response.data.tips && response.data.tips.length > 0) {
      tip.value = response.data.tips[0] as Tip
      strategyData.value = Object.fromEntries(tip.value.strategies.map((entry) => [entry.kind, entry]))
    } else if (response.data.strategyTypes) {
      strategyData.value = Object.fromEntries(response.data.strategyTypes.map((entry) => {
        return JSON.parse(JSON.stringify([entry, strategyFromType(entry)]))
      }))
    }

    if (response.data.gameId && (!response.data.game || !response.data.game.countryA)) {
      const gameResponse = await gameStore.fetchById(response.data.gameId || '', props.tournamentId)
      if (gameResponse) {
        response.data.game = gameResponse
      }
    }

    const endDate = new Date(response.data.endsAt || '');
    isClosed.value = response.data.completed || (endDate && endDate < new Date())

    tipStrategy.value = response.data
  }

  // 2. Sort the tip strategies
  q.loading.hide()
})
</script>
