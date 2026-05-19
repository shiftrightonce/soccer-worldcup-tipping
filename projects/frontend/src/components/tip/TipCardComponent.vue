<template>
  <q-card  flat bordered>
      <q-card-section>
        <div class="text-h6">Tip Strategy Label</div>
        <div class="text-subtitle2">Description</div>
      </q-card-section>
      <q-card-section class="flex flex-center items-center">
        <span class="fi fi-af" style="width: 11.33em;line-height: 6em;"></span>
            vs
          <span class="fi fi-bj" style="width: 11.33em;line-height: 6em;" ></span>
      </q-card-section>
       <q-separator />
      <q-card-actions align="right">
        <q-btn flat> 7:30PM </q-btn>
        <q-btn flat color="warning"> pt: 100 </q-btn>
        <q-space />
        <q-btn flat round color="primary" :icon="icon" @click="onManageBtnClick"></q-btn>
      </q-card-actions>
</q-card>
</template>

<script setup lang="ts">
import type { Tip } from 'src/api/Tip';
import type { TipStrategy } from 'src/api/TipStrategy';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const props = defineProps<{tip?: Tip, tipStrategy?: TipStrategy}>()
const icon = ref('tips_and_updates')
const router = useRouter()

const onManageBtnClick = async () => {
  await router.push({
    name: 'manage-tip',
    params: {
      tournamentId: 'tournament-id-here',
      tipStrategyId: 'tip-strategy-id-here',
      id: props.tip?.id || ''
    }
  })
}

if (props.tipStrategy?.completed || props.tip?.tipStrategy?.completed) {
  icon.value =  'query_stats'
} else if (props.tip) {
  icon.value =  'check'
}


</script>
