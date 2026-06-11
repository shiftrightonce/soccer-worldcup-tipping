<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-md-4 col-xs-12 q-pa-sm" v-for="strategy in openTipStrategies" :key="strategy.id">
        <TipCardComponent :tip-strategy="strategy" :tip="tips[strategy.id]"
          @tip-saved="(tip) => tips[strategy.id] = tip"></TipCardComponent>
      </div>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import type { Tip } from 'src/api/Tip';
import type { TipStrategy } from 'src/api/TipStrategy';
import TipStrategyClient from 'src/api/v1/clients/TipStrategyClient';
import TournamentClient from 'src/api/v1/clients/TournamentClient';
import TipCardComponent from 'src/components/tip/TipCardComponent.vue';
import { useLayoutStore } from 'src/stores/layout-store';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';


const layoutStore = useLayoutStore();
const route = useRoute();
const tournamentId = (route.params.tournamentId || '') as string
const userStore = useUserStore()
const tournamentClient = TournamentClient(userStore.authHeader())
const openTipStrategies = ref<Array<TipStrategy>>([])
const tips = ref<Record<string, Tip>>({})
const router = useRouter()

layoutStore.setTitle('Active')

const fetchActiveTips = async (tournamentId: string) => {
  const client = TipStrategyClient(userStore.authHeader(), tournamentId)
  const response = await client.open(new URLSearchParams({ with: 'my_tips' }))
  if (response.data) {
    openTipStrategies.value = response.data
    response.data.forEach(strategy => {
      if (strategy.tips && strategy.tips.length > 0) {
        tips.value[strategy.id] = strategy.tips[0] as Tip
      }
    })
  }
}

onMounted(async () => {
  if (tournamentId) {
    await fetchActiveTips(tournamentId)
  } else {
    const params = new URLSearchParams({ filter: 'status=active' });
    const response = await tournamentClient.paginate(params)
    if (response.data) {
      if (response.data.length == 1) {
        const tournamentId = response.data[0]?.id || ''
        await router.push({
          name: 'active-tips',
          params: {
            tournamentId
          }
        })
        await fetchActiveTips(tournamentId)
      } else {
        // Show the list for the user to select one
      }
    }
  }
})
</script>
