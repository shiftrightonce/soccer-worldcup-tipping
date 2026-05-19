<template>

  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10">
        <div class="text-h6">Tip Strategies</div>
        <div class="text-body2 text-muted-foreground">Manage Tip Strategies</div>
      </div>
      <div class="col-2">
        <q-btn
          color="primary"
          icon="add"
          outline
          no-caps
          label="New Tip Strategy"
          :to="{ name: 'manage-strategy' }"
        ></q-btn>
      </div>
    </div>
    <div class="row">
        <div>
          {{ result }}
        </div>
    </div>
    </q-page>
</template>

<script setup lang="ts">
import type { TipStrategy } from 'src/api/TipStrategy';
import TipStrategyClient  from 'src/api/v1/clients/StrategyClient'
import { useUserStore } from 'src/stores/user-store';
import { onMounted, ref } from 'vue';


const props = defineProps<{tournamentId: string}>()
const userStore = useUserStore()
const client = TipStrategyClient(userStore.authHeader(), props.tournamentId)
const result =  ref<Array<TipStrategy>>([])

onMounted(async () => {
  try {
    const response = await client.all()
    if (response.data) {
      result.value = response.data
    }
  } catch (e) {
    console.error(e)
  }
});

</script>
