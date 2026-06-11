<template>
  <q-card flat bordered>
    <q-card-section>
      <div class="row items-center no-wrap">
        <div class="col">
          <div class="text-h6">{{ props.tipStrategy?.label }}</div>
          <div class="text-subtitle2">{{ props.tipStrategy?.description }}</div>
        </div>
        <div class="col-auto">
          <q-badge v-if="props.forResult" rounded :color="props.tipStrategy.result ? 'negative' : 'grey'">
          </q-badge>
          <q-badge v-else rounded :color="tip?.id ? 'green-6' : 'orange'">
          </q-badge>
        </div>
      </div>
    </q-card-section>
    <q-card-section class="flex flex-center items-center" v-if="game">
      <FlagComponent v-if="game.countryA" :country-code="game.countryA.alpha2"
        :width="$q.platform.is.mobile ? '8em' : '8.33em'" height="6em" />
      <span class="q-ml-md q-mr-md">vs</span>
      <FlagComponent v-if="game.countryB" :country-code="game.countryB?.alpha2"
        :width="$q.platform.is.mobile ? '8em' : '8.33em'" height="6em" />
    </q-card-section>
    <q-card-section v-else class="flex flex-center items-center">
      <q-img src="/img/sort.svg" height="6em" width="6em" />
    </q-card-section>
    <q-separator />
    <q-card-section v-show="!props.forResult">
      <div class="text-body2 text-muted-foreground" v-if="isClosed">Ended at {{ endsAt.toLocaleString() }}</div>
      <div class="text-body2 text-muted-foreground" v-else>Ends at {{ endsAt.toLocaleString() }}</div>
    </q-card-section>
    <q-separator />
    <q-card-actions align="right">
      <div class="text-h6" v-show="isClosed && !props.forResult">Points: <span class="text-red-4">{{ props.tip?.points
        || 0 }}</span></div>
      <q-space />
      <q-btn flat round color="primary" :icon="icon" @click="onManageBtnClick"></q-btn>
    </q-card-actions>
  </q-card>
</template>

<script setup lang="ts">
import type { Tip } from 'src/api/Tip';
import type { TipStrategy } from 'src/api/TipStrategy';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import type { Game } from 'src/api/Game';
import FlagComponent from 'src/components/FlagComponent.vue';
import { useQuasar } from 'quasar'
import TipStrategyComponent from './TipStrategyComponent.vue';
import { useGameStore } from 'src/stores/game-store';

const $q = useQuasar()
const props = defineProps<{ tip?: Tip | undefined, tipStrategy: TipStrategy, forResult?: boolean }>()
const icon = ref('tips_and_updates')
const router = useRouter()
const game = ref<Game | null>(null)
const endsAt = new Date(props.tipStrategy.endsAt || props.tip?.tipStrategy?.endsAt || '')
const useDialog = ref(true)
const gameStore = useGameStore()
const isClosed = props.tipStrategy.completed || (new Date(props.tipStrategy.endsAt || '')) < new Date()

const emits = defineEmits<{ tipSaved: [Tip] }>()

const onManageBtnClick = async () => {
  if (props.tipStrategy.strategyTypes.includes('round_32_qualifiers')) {
    useDialog.value = false
  }

  if (useDialog.value) {
    // open dialog with TipStrategyComponent
    $q.dialog({
      component: TipStrategyComponent,
      componentProps: {
        tournamentId: props.tipStrategy.tournamentId || props.tip?.tipStrategy?.tournamentId || '',
        tipStrategyId: props.tipStrategy.id || props.tip?.tipStrategy?.id || '',
        id: props.tip?.id,
        forResult: props.forResult
      }
    }).onOk((tip: Tip) => {
      if (tip.id) {
        icon.value = 'check';
      }
      emits('tipSaved', tip)
    })
  } else {
    // navigate to TipManagePage
    await router.push({
      name: 'manage-tip',
      params: {
        tournamentId: props.tipStrategy.tournamentId || props.tip?.tipStrategy?.tournamentId || '',
        tipStrategyId: props.tipStrategy.id || props.tip?.tipStrategy?.id || '',
        id: props.tip?.id
      }
    })
  }
}

if (props.tipStrategy?.completed || props.tip?.tipStrategy?.completed) {
  icon.value = 'query_stats'
} else if (props.tip) {
  icon.value = 'check'
}

if (props.forResult) {
  icon.value = 'input'
}

onMounted(async () => {
  if (props.tipStrategy?.gameId) {
    game.value = await gameStore.fetchById(props.tipStrategy.gameId, props.tipStrategy.tournamentId || '')
  }
})

</script>
