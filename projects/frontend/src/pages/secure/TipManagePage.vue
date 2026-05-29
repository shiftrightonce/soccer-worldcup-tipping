<template>
  <q-page class="q-pa-md q-mb-md">
    <div class="row">
      <div class="col-10 col-xs-12">
        <div class="text-h6">{{ tipStrategy?.label }}</div>
      </div>
    </div>
    <div class="row">
      <div class="col-12">
        <q-list bordered>
          <q-expansion-item v-for="(type, index) in tipStrategy?.strategyTypes" :default-opened="index == 0" :key="type"
            group="somegroup" icon="explore" :label=strategyTypeKeyValue[type]>
            <q-card>
              <q-card-section class="q-pa-md">
                <div class="text-body2 text-muted-foreground">{{ tipStrategy?.description }}</div>
                <component :is="strategyComponents[type]" v-model="strategyData[type]" :tip-strategy="tipStrategy"
                  :is-closed="isClosed">
                </component>
              </q-card-section>
            </q-card>
          </q-expansion-item>
        </q-list>
      </div>
    </div>
    <div class="row q-mt-md">
      <div class="col-6">
        <q-btn flat color="secondary" label="Back" @click="() => router.go(-1)"></q-btn>
      </div>
      <div class="col-6">
        <q-btn color="primary" flat label="Save" @click="saveTip" style="float: right;" :disabled="isClosed"
          v-show="!isClosed"></q-btn>
      </div>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { strategyTypeKeyValue } from 'src/general/lists';
import { strategyComponents } from 'src/components/strategy/.'
import { ref, onMounted } from 'vue';
import type { TipStrategy } from 'src/api/TipStrategy';
import type { Tip } from 'src/api/Tip';
import type { TipPayload } from 'src/api/v1/TipPayload'
import TipStrategyClient from 'src/api/v1/clients/TipStrategyClient';
import { useUserStore } from 'src/stores/user-store';
import type { Strategy } from 'src/api/Strategy';
import { strategyFromType, validateStrategy } from 'src/general/strategy_helper';
import { useQuasar } from 'quasar';
import { useGameStore } from 'src/stores/game-store';
import TipClient, { makeNewPayload } from 'src/api/v1/clients/TipClient';
import { useRouter } from 'vue-router';

const props = defineProps<{ tournamentId: string, tipStrategyId: string, id?: string }>()
const userStore = useUserStore()
const tipStrategyClient = TipStrategyClient(userStore.authHeader(), props.tournamentId)
const tipClient = TipClient(userStore.authHeader(), props.tournamentId)
const strategyData = ref<Record<string, Strategy>>({})
const isClosed = ref(false)
const q = useQuasar()
const gameStore = useGameStore()
const router = useRouter()

// test data for now
const tipStrategy = ref<TipStrategy | null>(null)

// test data for now
const tip = ref<TipPayload>(makeNewPayload(props.tournamentId, props.tipStrategyId))

const saveTip = async () => {
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
    console.log('saved tip response', response.data)
    tip.value = response.data
  }
}

onMounted(async () => {
  q.loading.show()

  // 1. Fetch the Tipstrategy by ID
  const response = await tipStrategyClient.byId(props.tipStrategyId, new URLSearchParams({ with: 'my_tips' }))
  if (response.data) {
    response.data.strategyTypes = response.data.strategyTypes.sort((a, b) => a.toString().localeCompare(b.toString()))
    if (response.data.tips && response.data.tips.length > 0) {
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
