<template>

  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10">
        <div class="text-h6">Tip Strategies</div>
        <div class="text-body2 text-muted-foreground">Manage Tip Strategies</div>
      </div>
      <div class="col-2">
        <q-btn color="primary" icon="add" outline no-caps label="New Tip Strategy"
          :to="{ name: 'manage-strategy' }"></q-btn>
      </div>
    </div>
    <div class="row">
      <div class="col q-pa-sm">
        <q-table title="Country Groups" :columns="columns" :rows="result" row-key="id">
          <template v-slot:header="props">
            <q-tr :props="props">
              <q-th auto-width />
              <q-th v-for="col in props.cols" :key="col.name" :props="props">
                {{ col.label }}
              </q-th>
            </q-tr>
          </template>
          <template v-slot:body="props">
            <q-tr :props="props">
              <q-td auto-width>
                <q-btn size="sm" color="primary" round flat dense icon="more_vert">
                  <q-menu>
                    <q-list>
                      <q-item clickable v-close-popup>
                        <q-item-section @click="() => onEditClick(props.key)">Edit</q-item-section>
                      </q-item>
                    </q-list>
                  </q-menu>
                </q-btn>
              </q-td>
              <q-td v-for="col in props.cols" :key="col.name" :props="props">
                {{ col.value }}
              </q-td>
            </q-tr>
          </template>
        </q-table>
      </div>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import type { QTableColumn } from 'quasar';
import type { Game } from 'src/api/Game';
import type { Group } from 'src/api/Group';
import type { StrategyType } from 'src/api/StrategyType';
import type { TipStrategy } from 'src/api/TipStrategy';
import type { Tournament } from 'src/api/Tournament';
import TipStrategyClient from 'src/api/v1/clients/TipStrategyClient'
import { strategyTypeKeyValue } from 'src/general/lists';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';


const strategyTypeKV = strategyTypeKeyValue;
const props = defineProps<{ tournamentId: string }>()
const userStore = useUserStore()
const client = TipStrategyClient(userStore.authHeader(), props.tournamentId)
const result = ref<Array<TipStrategy>>([])
const router = useRouter()
const columns = [
  {
    name: 'id',
    label: 'ID',
    field: 'id',
    align: 'left',
  },
  {
    name: 'label',
    label: 'Label',
    field: 'label',
    align: 'left'
  },
  {
    name: 'completed',
    label: 'Is Completed',
    field: 'completed',
    align: 'left'
  },
  {
    name: 'tournament',
    label: 'Tournament',
    field: 'tournament',
    align: 'left',
    format: (val: Tournament, _row: TipStrategy) => val.label
  },
  {
    name: 'endsAt',
    label: 'Ends At',
    field: 'endsAt',
    align: 'left',
    format: (val: string | null, _row: TipStrategy) => (val ? new Date(val).toLocaleString() : ''),
  },
  {
    name: 'game',
    label: 'Game',
    field: 'game',
    align: 'left',
    format: (val: Game, _row: TipStrategy) => val ? val.label : ''
  },
  {
    name: 'group',
    label: 'Group',
    field: 'group',
    align: 'left',
    format: (val: Group, _row: TipStrategy) => val ? val.name : ''
  },
  {
    name: 'strategyTypes',
    label: 'Strategy Types',
    field: 'strategyTypes',
    align: 'left',
    format: (val: StrategyType[], _row: TipStrategy) => val ? val.map((e) => strategyTypeKV[e]).join(',') : ''
  },
  {
    name: 'createdAt',
    label: 'Created At',
    field: 'createdAt',
    format: (val: string | null, _row: TipStrategy) => (val ? new Date(val).toLocaleString() : ''),
  },
  {
    name: 'updatedAt',
    label: 'Updated At',
    field: 'updatedAt',
    format: (val: string | null, _row: TipStrategy) => (val ? new Date(val).toLocaleString() : ''),
  },
  {
    name: 'deleted At',
    label: 'Deleted At',
    field: 'deletedAt',
    format: (val: string | null, _row: TipStrategy) => (val ? new Date(val).toLocaleString() : ''),
  },
] as QTableColumn[];

const onEditClick = async (id: string) => {
  await router.push({
    name: 'manage-strategy',
    query: {
      forResult: 'true'
    },
    params: {
      tournamentId: props.tournamentId,
      id
    }
  })
}

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
