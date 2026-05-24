<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10">
        <div class="text-h6">Games</div>
        <div class="text-body2 text-muted-foreground">Manage Games</div>
      </div>
      <div class="col-2">
        <q-btn color="primary" icon="add" outline no-caps label="New Game" :to="{ name: 'manage-game' }"></q-btn>
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
import type { Country } from 'src/api/Country';
import type { Game } from 'src/api/Game';
import GameClient from 'src/api/v1/clients/GameClient';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

const result = ref<Array<Game>>([])
const router = useRouter()
const props = defineProps<{ tournamentId: string }>()
const userStore = useUserStore()
const client = GameClient(userStore.authHeader(), props.tournamentId)

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
    align: 'left',
  },
  {
    name: 'status',
    label: 'Status',
    field: 'status',
    align: 'left',
  },
  {
    name: 'count',
    label: 'Count',
    field: 'count',
    align: 'left',
  },
  {
    name: 'countryA',
    label: 'country A',
    field: 'countryA',
    align: 'left',
    format: (val: Country, _: Game) => val.name
  },
  {
    name: 'countryB',
    label: 'country B',
    field: 'countryB',
    align: 'left',
    format: (val: Country, _: Game) => val.name
  },
  {
    name: 'toConfigureOn',
    label: 'To Configure On',
    field: 'toConfigureOn',
    align: 'left',
  },
  {
    name: 'createdAt',
    label: 'Created At',
    field: 'createdAt',
    format: (val: string | null, _row: Game) => (val ? new Date(val).toLocaleString() : ''),
  },
  {
    name: 'updatedAt',
    label: 'Updated At',
    field: 'updatedAt',
    format: (val: string | null, _row: Game) => (val ? new Date(val).toLocaleString() : ''),
  },
  {
    name: 'deleted At',
    label: 'Deleted At',
    field: 'deletedAt',
    format: (val: string | null, _row: Game) => (val ? new Date(val).toLocaleString() : ''),
  },

] as QTableColumn[];

const onEditClick = async (id: string) => {
  await router.push({
    name: 'manage-game',
    params: {
      tournamentId: props.tournamentId,
      id
    }
  })
}

onMounted(async () => {
  try {
    const params = new URLSearchParams({ _sort: "-count" })
    const response = await client.all(params)
    if (response.data) {
      result.value = response.data
    }
  } catch (e) {
    console.error(e)
  }
})
</script>
