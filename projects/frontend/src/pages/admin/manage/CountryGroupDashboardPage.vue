<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10">
        <div class="text-h6">Country Groups</div>
        <div class="text-body2 text-muted-foreground">Manage Country Groups</div>
      </div>
      <div class="col-2">
        <q-btn color="primary" icon="add" outline no-caps label="New Country Group"
          :to="{ name: 'manage-country-group' }"></q-btn>
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
import type { CountryGroup } from 'src/api/CountryGroup';
import type { Group } from 'src/api/Group';
import type { Tournament } from 'src/api/Tournament';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router'
import CountryGroupClient from 'src/api/v1/clients/CountryGroupClient';

const result = ref<Array<CountryGroup>>([])
const props = defineProps<{ tournamentId: string }>()
const userStore = useUserStore()
const client = CountryGroupClient(userStore.authHeader(), props.tournamentId)
const router = useRouter()
const columns = [
  {
    name: 'id',
    label: 'ID',
    field: 'id',
    align: 'left',
  },
  {
    name: 'country',
    label: 'Country',
    field: 'country',
    align: 'left',
    format: (val: Country, _row: CountryGroup) => val.name
  },
  {
    name: 'isOut',
    label: 'Is Out',
    field: 'isOut',
    align: 'left',
    format: (val: boolean, _row: CountryGroup) => val ? 'Yes' : 'No'
  },
  {
    name: 'group',
    label: 'Group',
    field: 'group',
    align: 'left',
    format: (val: Group, _row: CountryGroup) => val.name
  },
  {
    name: 'tournament',
    label: 'Tournament',
    field: 'tournament',
    align: 'left',
    format: (val: Tournament, _row: CountryGroup) => val.label
  },
  {
    name: 'createdAt',
    label: 'Created At',
    field: 'createdAt',
    format: (val: string | null, _row: CountryGroup) => (val ? new Date(val).toLocaleString() : ''),
  },
  {
    name: 'updatedAt',
    label: 'Updated At',
    field: 'updatedAt',
    format: (val: string | null, _row: CountryGroup) => (val ? new Date(val).toLocaleString() : ''),
  },
  {
    name: 'deleted At',
    label: 'Deleted At',
    field: 'deletedAt',
    format: (val: string | null, _row: CountryGroup) => (val ? new Date(val).toLocaleString() : ''),
  },
] as QTableColumn[];

const onEditClick = async (id: string) => {
  await router.push({
    name: 'manage-country-group',
    params: {
      tournamentId: props.tournamentId,
      id
    }
  })
}

onMounted(async () => {
  try {
    const params = new URLSearchParams({ _sort: "id", _limit: "400" })
    const response = await client.paginate(params)
    if (response.data) {
      result.value = response.data
    }
  } catch (e) {
    console.error(e)
  }
})

</script>
