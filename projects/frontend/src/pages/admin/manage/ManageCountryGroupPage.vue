<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10">
        <div class="text-h6">Country Groups</div>
        <div class="text-body2 text-muted-foreground">Manage Country Groups</div>
      </div>
      <div class="col-2">
        <q-btn-dropdown color="primary" label="Action">
          <q-list>
            <q-item clickable v-close-popup @click="saveRecord">
              <q-item-section>
                <q-item-label>Save</q-item-label>
              </q-item-section>
            </q-item>
            <q-item clickable v-close-popup @click="saveAndNew">
              <q-item-section>
                <q-item-label>Save and New</q-item-label>
              </q-item-section>
            </q-item>
            <q-separator />
            <q-item clickable v-close-popup @click="deleteRecord">
              <q-item-section>
                <q-item-label>Delete</q-item-label>
              </q-item-section>
            </q-item>
          </q-list>
        </q-btn-dropdown>
      </div>
    </div>

    <q-form ref="form">
      <div class="row">
        <div class="col-6 q-pa-sm">
          <CountryDropdown :countries="countries" label="Country" v-model="model.data.countryId" />
        </div>

        <div class="col-6 q-pa-sm">
          <q-select outlined v-model="model.data.groupId" :options="groups" map-options emit-value option-value="id"
            option-label="name" label="Group" />
        </div>
      </div>
      <div class="row">
        <div class="col-6 q-pa-sm">
          <q-checkbox v-model="model.data.isOut" label="Is out of the tournament" />
        </div>
        <div class="col-6 q-pa-sm">
          <q-input type="number" outlined v-model="model.data.points" label="Points" />
        </div>
      </div>

    </q-form>
  </q-page>
</template>

<script setup lang="ts">
import type { Country } from 'src/api/Country';
import type { CountryGroup } from 'src/api/CountryGroup';
import type { Group } from 'src/api/Group';
import CountryClient from 'src/api/v1/clients/CountryClient';
import CountryGroupClient, { makeNewPayload } from 'src/api/v1/clients/CountryGroupClient';
import GroupClient from 'src/api/v1/clients/GroupClient';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, reactive, ref } from 'vue';
import { useRouter } from 'vue-router';
import CountryDropdown from 'src/components/CountryDropdown.vue';

const form = ref(null)
const userStore = useUserStore()
const props = defineProps<{ tournamentId: string, id?: string }>()
const client = CountryGroupClient(userStore.authHeader(), props.tournamentId)
const countryClient = CountryClient(userStore.authHeader())
const groupClient = GroupClient(userStore.authHeader())
const router = useRouter()

const countries = reactive<Array<Country>>([])
const groups = reactive<Array<Group>>([])
const model = reactive({ data: makeNewPayload() })
model.data.tournamentId = props.tournamentId


const doSave = async (): Promise<CountryGroup | null> => {
  try {
    const response = await client.save(model.data, props.id);
    model.data = makeNewPayload();
    return response.data
  } catch (e) {
    console.error(e)
  }
  return null
}

const saveRecord = async () => {
  const data = await doSave()
  if (data) {
    await router.push({
      name: 'manage-country-group',
      params: {
        tournamentId: props.tournamentId,
        id: data.id
      }
    })
  }
}
const saveAndNew = async () => {
  const data = await doSave()
  if (data) {
    await router.push({
      name: 'manage-country-group',
      params: {
        tournamentId: props.tournamentId,
        id: null
      }
    })
    location.reload()
  }
}
const deleteRecord = () => { }

onMounted(async () => {
  try {
    (await countryClient.all()).data?.forEach((entry) => countries.push(entry));
    (await groupClient.all()).data?.forEach((entry) => groups.push(entry));
    if (props.id) {
      const response = await client.byId(props.id)
      if (response.data) {
        model.data = response.data
      }
      console.log('response', response.data)
    }
  } catch (e) {
    console.error(e)
  }
})

</script>
