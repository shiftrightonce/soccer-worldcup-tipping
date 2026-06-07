<template>
  <q-page class="row items-center justify-evenly">
    <h1>Score Board Page</h1>
  </q-page>
</template>

<script setup lang="ts">
import TipClient from 'src/api/v1/clients/TipClient';
import TournamentClient from 'src/api/v1/clients/TournamentClient';
import { useUserStore } from 'src/stores/user-store';
import { onMounted } from 'vue';


const userStore = useUserStore()
const tournamentClient = TournamentClient(userStore.authHeader())

onMounted(async () => {

  const params = new URLSearchParams({ filter: 'status=active' });
  const response = await tournamentClient.paginate(params)

  if (response.data) {
    if (response.data.length == 1) {
      const tipClient = TipClient(userStore.authHeader(), response.data[0]?.id || '')
      const r = await tipClient.leaderBoard();
      console.log('leader board response', r)
    } else if (response.data.length > 1) {
      console.error('multiple tournaments');
    }
  }
})

</script>
