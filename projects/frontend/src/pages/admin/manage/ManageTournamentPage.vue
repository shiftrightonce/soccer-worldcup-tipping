<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-9">
        <div class="text-h6">Tournaments</div>
        <div class="text-body2 text-muted-foreground">Manage Tournaments</div>
      </div>
      <div class="col-1">
        <q-btn color="primary" icon="add" outline no-caps label="Save" @click="saveRecord"></q-btn>
      </div>
      <div class="col-2">
        <q-btn
          color="red"
          icon="delete"
          outline
          no-caps
          label="Delete"
          @click="deleteRecord"
        ></q-btn>
      </div>
    </div>

    <q-form ref="form">
      <div class="row">
        <div class="col q-pa-sm">
          <q-input outlined v-model="model.data.label" label="Label" />
        </div>
        <div class="col q-pa-sm">
          <q-select
            outlined
            v-model="model.data.status"
            emit-value
            :options="statusOption"
            label="Status"
          />
        </div>
      </div>
      <div class="row">
        <div class="col-12 q-pa-sm">
          <q-input outlined label="Description" v-model="model.data.description" type="textarea" />
        </div>
      </div>
    </q-form>
  </q-page>
</template>

<script setup lang="ts">
import makeClient, { makeNewPayload } from 'src/api/v1/clients/TournamentClient';
import { tournamentStatusKeyValue } from 'src/general/lists';
import { useUserStore } from 'src/stores/user-store';
import { onMounted } from 'vue';
import { reactive, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const router = useRouter();
const route = useRoute();
const tournamentId = (route.params.id || '') as string;
const userStore = useUserStore();
const form = ref(null);
const client = makeClient(userStore.authHeader());
const statusOption = Object.entries(tournamentStatusKeyValue).map(([k, l]) => {
  return {
    value: k,
    label: l,
  };
});

const model = reactive({ data: makeNewPayload() });

onMounted(async () => {
  if (tournamentId) {
    const response = await client.getById(tournamentId);
    if (response.data) {
      model.data = reactive(response.data);
    }
  }
});

const saveRecord = async () => {
  const response = await client.save(model.data, tournamentId);
  if (response.data) {
    await router.push({
      name: 'admin-manage-edit-tournament',
      params: {
        id: response.data.id,
      },
    });
  }
};
const deleteRecord = async () => {};
</script>
