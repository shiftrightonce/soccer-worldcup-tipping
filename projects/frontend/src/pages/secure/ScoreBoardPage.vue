<template>
  <q-page padding v-if="scores.length">
    <transition appear enter-active-class="animated slideInDown" leave-active-class="animated slideOutUp">
      <div class="row">
        <div class="col-md-4 col-xs-12 q-pa-sm" v-if="currentUserPoint">
          <ScoreboardCard current-user :point="currentUserPoint"></ScoreboardCard>
        </div>
        <div class="col-md-4 col-xs-12 q-pa-sm" v-for="point in scores" :key="point.userId">
          <ScoreboardCard :point="point"></ScoreboardCard>
        </div>
      </div>
    </transition>
    <q-page-scroller expand position="top" :scroll-offset="150" :offset="[0, 0]">
      <ScrollUpMessage></ScrollUpMessage>
    </q-page-scroller>
  </q-page>

  <q-page padding v-if="ready && scores.length == 0" class="row items-evenly items-center flex-center">
    <div class="col-12" style="text-align:center">
      <img src="/img/first.svg" style="width:200px" /><br />
      <span class="text-h6">Top position is still up for grabs!</span>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import type { Point } from 'src/api/Point';
import TipClient from 'src/api/v1/clients/TipClient';
import TournamentClient from 'src/api/v1/clients/TournamentClient';
import ScoreboardCard from 'src/components/general/ScoreboardCard.vue';
import ScrollUpMessage from 'src/components/general/ScrollUpMessage.vue';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, ref } from 'vue';


const userStore = useUserStore()
const tournamentClient = TournamentClient(userStore.authHeader())
const scores = ref<Array<Point>>([])
const currentUserPoint = ref<Point | null>(null);
const ready = ref(false);
const user = userStore.loginData?.user;


onMounted(async () => {

  const params = new URLSearchParams({ filter: 'status=active' });
  const response = await tournamentClient.paginate(params)

  if (response.data) {
    if (response.data.length == 1) {
      const tipClient = TipClient(userStore.authHeader(), response.data[0]?.id || '')
      const result = await tipClient.leaderBoard();
      if (result.data) {
        scores.value = result.data
        scores.value.map((entry, index) => {
          entry.position = index + 1;
          if (entry.userId == user?.id) {
            currentUserPoint.value = entry;
          }
          return entry
        })
      }
    } else if (response.data.length > 1) {
      console.error('multiple tournaments');
    }
  }
  ready.value = true;
})

</script>
