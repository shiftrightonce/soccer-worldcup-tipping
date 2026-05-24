<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10">
        <div class="text-h6">Tournaments</div>
        <div class="text-body2 text-muted-foreground">Manage Tournaments</div>
      </div>
      <div class="col-2">
        <q-btn color="primary" icon="add" outline no-caps label="New Tournament"
          :to="{ name: 'admin-manage-new-tournament' }"></q-btn>
      </div>
    </div>
    <div class="row">
      <div class="col q-pa-sm">
        <q-table title="Tournaments" :columns="columns" :rows="result" row-key="id">
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
                      <q-item clickable v-close-popup>
                        <q-item-section @click="() => onManageCountryGroupClick(props.key)">Manage Country
                          Groups</q-item-section>
                      </q-item>
                      <q-item clickable v-close-popup>
                        <q-item-section @click="() => onManageStrategiesClick(props.key)">Manage
                          Strategies</q-item-section>
                      </q-item>
                      <q-item clickable v-close-popup>
                        <q-item-section @click="() => onManageGamesClick(props.key)">Manage Games</q-item-section>
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
import type { Tournament } from 'src/api/Tournament';
import type { TournamentStatus } from 'src/api/TournamentStatus';
import TournamentClient from 'src/api/v1/clients/TournamentClient';
import { tournamentStatusKeyValue } from 'src/general/lists';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter()
const userStore = useUserStore();
const client = TournamentClient(userStore.authHeader());
const result = ref<Array<Tournament>>([]);
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
    format: (val, _row) => tournamentStatusKeyValue[val as TournamentStatus]
  },
  {
    name: 'createdAt',
    label: 'Created At',
    field: 'createdAt',
    format: (val, _row) => (val ? new Date(val).toLocaleString() : ''),
  },
  {
    name: 'updatedAt',
    label: 'Updated At',
    field: 'updatedAt',
    format: (val, _row) => (val ? new Date(val).toLocaleString() : ''),
  },
  {
    name: 'deleted At',
    label: 'Deleted At',
    field: 'deletedAt',
    format: (val, _row) => (val ? new Date(val).toLocaleString() : ''),
  },
] as QTableColumn[];

const onEditClick = async (id: string) => {
  await router.push({
    name: 'admin-manage-edit-tournament',
    params: {
      id
    }
  })
}

const onManageCountryGroupClick = async (id: string) => {
  await router.push({
    name: 'country-groups-dashboard',
    params: {
      tournamentId: id
    }
  })
}

const onManageStrategiesClick = async (id: string) => {
  await router.push({
    name: 'strategies-dashboard',
    params: {
      tournamentId: id
    }
  })
}
const onManageGamesClick = async (id: string) => {
  await router.push({
    name: 'games-dashboard',
    params: {
      tournamentId: id
    }
  })
}

onMounted(async () => {
  try {
    const params = new URLSearchParams({ _sort: "-id" })
    const response = await client.paginate(params);
    if (response.data) {
      result.value = response.data;
    }
  } catch (e) {
    console.error(e);
  }
});
</script>
