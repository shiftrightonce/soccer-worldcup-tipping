<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10">
        <div class="text-h6">Tip Strategies</div>
        <div class="text-body2 text-muted-foreground">Manage Tip Strategies</div>
      </div>
      <div class="col-2">
        <q-btn-dropdown color="primary" label="Action">
          <q-list>
            <q-item clickable v-close-popup @click="saveRecord">
              <q-item-section>
                <q-item-label>Save</q-item-label>
              </q-item-section>
            </q-item>
            <q-item clickable v-close-popup @click="saveRecordAndNew">
              <q-item-section>
                <q-item-label>Save And New</q-item-label>
              </q-item-section>
            </q-item>
            <q-item clickable v-close-popup @click="newRecord">
              <q-item-section>
                <q-item-label>New</q-item-label>
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
          <q-select outlined v-model="model.data.gameId" option-value="id" option-label="label" emit-value map-options
            :options="games" label="Game" @update:modelValue="onChangeGame" />
          <q-select outlined v-model="model.data.groupId" emit-value map-options option-value="id" option-label="name"
            :options="groups" label="Group" class="q-mt-md" @update:model-value="onChangeGroup" />
        </div>
        <div class="col-6 q-pa-sm">
          <q-input outlined v-model="model.data.label" label="Label" />
          <q-input outlined v-model="model.data.description" label="Description" type="textarea" class="q-mt-md" />
        </div>
      </div>
      <div class="row">
        <div class="col-6 q-pa-sm">
          <q-list>
            <q-item tag="label" v-ripple v-for="aType in types" :key="aType">
              <q-item-section avatar>
                <q-checkbox v-model="model.data.strategyTypes" :val="aType" />
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ typeKV[aType] }}</q-item-label>
              </q-item-section>
            </q-item>
          </q-list>
        </div>
        <div class="col-6 q-pa-sm">
          Opens At
          <date-time-component v-model="model.data.opensAt"></date-time-component>
          Ends At
          <date-time-component v-model="model.data.endsAt"></date-time-component>
          Calculate Points On
          <date-time-component v-model="model.data.calculatePointsOn"></date-time-component>

          <div v-if="tipStrategy">
            <div class="text-h6 q-mt-md">End Result</div>
            <tip-card-component :tip-strategy="tipStrategy" forResult></tip-card-component>
          </div>
        </div>
      </div>
    </q-form>

  </q-page>

</template>

<script setup lang="ts">
import TipStrategyClient, { makeNewPayload } from 'src/api/v1/clients/TipStrategyClient'
import GameClient from 'src/api/v1/clients/GameClient'
import DateTimeComponent from 'src/components/DateTimeComponent.vue';
import { strategyTypeKeyValue, strategyTypeList } from 'src/general/lists';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, reactive, ref } from 'vue';
import type { Game } from 'src/api/Game';
import GroupClient from 'src/api/v1/clients/GroupClient';
import type { Group } from 'src/api/Group';
import { useRouter } from 'vue-router';
import type { TipStrategyPayload } from 'src/api/v1/TipStrategyPayload';
import type { TipStrategy } from 'src/api/TipStrategy';
import TipCardComponent from 'src/components/tip/TipCardComponent.vue';
import type { StrategyType } from 'src/api/StrategyType';

const userStore = useUserStore()
const props = defineProps<{ tournamentId: string, id?: string }>()
const tipStrategyClient = TipStrategyClient(userStore.authHeader(), props.tournamentId)
const gameClient = GameClient(userStore.authHeader(), props.tournamentId)
const groupClient = GroupClient(userStore.authHeader())
const typeKV = strategyTypeKeyValue
const types = strategyTypeList.sort((a, b) => a.toString().localeCompare(b.toString()));
const games = reactive<Game[]>([])
const groups = reactive<Group[]>([])
const router = useRouter()

const tipStrategy = ref<TipStrategy | null>(null)
const model = reactive({ data: makeNewPayload() })
model.data.tournamentId = props.tournamentId;

const onChangeGame = (gameId: string) => {
  if (!model.data.label) {
    const game = games.find((g) => g.id === gameId);
    if (game) {
      model.data.label = game.count + ' - ' + game.countryA?.name + ' vs ' + game.countryB?.name
      if (game.dateAndTime) {
        model.data.endsAt = game.dateAndTime
      }


      if (game.count <= 72) {
        [
          'goals',
          'first_red_card',
          'first_yellow_card',
          'winner',
        ].forEach((entry) => {
          model.data.strategyTypes.push(entry as StrategyType)
        })
      } else {
        [
          'goals',
          'first_red_card',
          'first_yellow_card',
          'winner',
          'penalty_goals',
          'game_to_penalty',
        ].forEach((entry) => {
          model.data.strategyTypes.push(entry as StrategyType)
        })
      }
    }
  }
}

const onChangeGroup = (groupId: string) => {
  if (!model.data.label) {
    model.data.label = groups.find((g) => g.id === groupId)?.name || ''
  }
}

const doSave = async () => {
  const response = await tipStrategyClient.save(model.data, props.id);
  return response.data
}
const deleteRecord = () => { }
const saveRecord = async () => {
  try {
    const data = await doSave()
    if (data) {
      await router.push({
        name: 'manage-strategy',
        params: {
          tournamentId: props.tournamentId,
          id: data.id
        }
      })
    }
  } catch (e) {
    console.error(e)
  }
}

const newRecord = async () => {
  await router.push({
    name: 'manage-strategy',
    params: {
      tournamentId: props.tournamentId,
      id: null
    }
  })
  location.reload()
}

const saveRecordAndNew = async () => {
  try {
    const data = await doSave()
    if (data) {
      await router.push({
        name: 'manage-strategy',
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

onMounted(async () => {
  try {
    if (props.id) {
      const response = await tipStrategyClient.byId(props.id);
      if (response.data) {
        tipStrategy.value = response.data
        model.data = response.data as TipStrategyPayload
      }
    }
    (await groupClient.all()).data?.forEach((entry) => groups.push(entry));
    const gamesResponse = await gameClient.all()
    if (gamesResponse.data) {
      gamesResponse.data.forEach((g) => games.push(g))
    }

  } catch (e) {
    // FIXME: Handle error properly
    console.log(e)
  }
})

</script>
