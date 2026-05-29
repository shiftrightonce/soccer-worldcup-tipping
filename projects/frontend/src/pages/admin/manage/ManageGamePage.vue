<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10">
        <div class="text-h6">Games</div>
        <div class="text-body2 text-muted-foreground">Manage Game</div>
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
        <div class="col-3 q-pa-sm">
          <SelectACountry :countries="countries" label="Country A" v-model="model.data.countryAId" />
        </div>
        <div class="col-3 q-pa-sm">
          <SelectACountry :countries="countries" label="Country B" v-model="model.data.countryBId" />
        </div>
        <div class="col-3 q-pa-sm">
          <q-input type="text" outlined label="Label" v-model="model.data.label" />
        </div>
        <div class="col-3 q-pa-sm">
          <date-time-component v-model="model.data.toConfigureOn"></date-time-component>
        </div>
      </div>
      <div class="row">
        <div class="col-3 q-pa-sm">
          <q-input type="number" outlined label="Count" v-model="model.data.count" />
        </div>
        <div class="col-3 q-pa-sm">
          <q-select outlined v-model="model.data.stage" :options="stages" map-options emit-value option-value="value"
            option-label="label" label="Stage" />
        </div>
        <div class="col-3 q-pa-sm">
          <q-select outlined v-model="model.data.status" :options="gameStatuses" map-options emit-value
            option-value="value" option-label="label" label="Status" />
        </div>
        <div class="col-3 q-pa-sm">
          <q-checkbox outlined label="To Penalty" v-model="model.data.penalty" />
        </div>
      </div>
      <div class="row">
        <div class="col-3 q-pa-sm">
          <q-input type="number" outlined label="Country A Goals" v-model="model.data.countryAGoals" />
        </div>
        <div class="col-3 q-pa-sm">
          <q-input type="number" outlined label="Country B Goals" v-model="model.data.countryBGoals" />
        </div>
        <div class="col-3 q-pa-sm">
          <q-input type="number" outlined label="Country A Penalty Goals" v-model="model.data.countryAPenaltyGoals" />
        </div>
        <div class="col-3 q-pa-sm">
          <q-input type="number" outlined label="Country B Penalty Goals" v-model="model.data.countryBPenaltyGoals" />
        </div>
      </div>
      <div class="row" v-show="props.id">
        <div class="col-3 q-pa-sm">
          <q-select outlined v-model="model.data.winnerId" :options="twoCountires" map-options emit-value
            option-value="id" option-label="name" label="Winner" />
        </div>
      </div>
    </q-form>
  </q-page>
</template>

<script setup lang="ts">
import type { Country } from 'src/api/Country';
import CountryGroupClient from 'src/api/v1/clients/CountryGroupClient';
import GameClient, { makeNewPayload } from 'src/api/v1/clients/GameClient';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, reactive } from 'vue';
import DateTimeComponent from 'src/components/DateTimeComponent.vue';
import type { GamePayload } from 'src/api/v1/GamePayload';
import { useRouter } from 'vue-router';
import { stageKeyValue, gameStatusKeyValue } from 'src/general/lists'
import SelectACountry from 'src/components/SelectACountryComponent.vue';

const router = useRouter()
const countries = reactive<Array<Country>>([])
const twoCountires = reactive<Array<Country>>([])
const userStore = useUserStore()
const props = defineProps<{ tournamentId: string, id?: string }>()
const client = GameClient(userStore.authHeader(), props.tournamentId)
const model = reactive({ data: makeNewPayload(props.tournamentId) })
const countryGroupClient = CountryGroupClient(userStore.authHeader(), props.tournamentId)
const stages = Object.entries(stageKeyValue).map(([key, value]) => { return { label: value, value: key }; })
const gameStatuses = Object.entries(gameStatusKeyValue).map(([key, value]) => { return { label: value, value: key }; })



onMounted(async () => {
  try {
    (await countryGroupClient.all(new URLSearchParams({ _sort: "-count" }))).data?.forEach((entry) => countries.push(entry.country as Country))
    if (props.id) {
      const response = await client.byId(props.id)
      if (response.data) {
        model.data = response.data as GamePayload
        twoCountires.push(response.data.countryA as Country)
        twoCountires.push(response.data.countryB as Country)
        console.log('two countries', twoCountires)
      }
    }
  } catch (e) {
    console.error(e)
  }
})

const doSave = async () => {
  const response = await client.save(model.data, props.id)
  return response.data
}
const saveRecord = async () => {
  try {
    const data = await doSave()
    if (data) {
      await router.push({
        name: 'manage-game',
        params: {
          tournamentId: props.tournamentId,
          id: data.id
        }
      })
      location.reload()
    }
  } catch (e) {
    console.error(e)
  }
}
const saveAndNew = async () => {
  try {
    const data = await doSave()
    if (data) {
      await router.push({
        name: 'manage-game',
        params: {
          tournamentId: props.tournamentId,
          id: null
        }
      })
      location.reload()
    }
  } catch (e) {
    console.error(e)
  }
}
const deleteRecord = async () => { }
</script>
