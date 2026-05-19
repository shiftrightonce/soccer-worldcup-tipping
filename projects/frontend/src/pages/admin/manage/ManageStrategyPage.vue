<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-9">
        <div class="text-h6">Tip Strategies</div>
        <div class="text-body2 text-muted-foreground">Manage Tip Strategies</div>
      </div>
      <div class="col-1">
        <q-btn color="primary" icon="add" outline no-caps label="Save" @click="saveRecord"></q-btn>
      </div>
      <div class="col-2">
        <q-btn color="red" icon="delete" outline no-caps label="Delete" @click="deleteRecord"></q-btn>
      </div>
    </div>

    <q-form ref="form">
      <div class="row">
        <div class="col-6 q-pa-sm">
          <q-input outlined v-model="model.data.label" label="Label" />
          <q-input outlined v-model="model.data.label" label="Game" class="q-mt-md" />
        </div>
        <div class="col-6 q-pa-sm">
          <q-input outlined v-model="model.data.description" label="Description" type="textarea" />
        </div>
      </div>
      <div class="row">
        <div class="col-6 q-pa-sm">
          <q-list>
            <q-item tag="label" v-ripple v-for="aType in types" :key="aType">
              <q-item-section avatar>
                <q-checkbox v-model="model.data.strategy_types" :val="aType" />
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ typeKV[aType] }}</q-item-label>
              </q-item-section>
            </q-item>
          </q-list>
        </div>
        <div class="col-6 q-pa-sm">
          Dates
          <date-time-component></date-time-component>
        </div>
      </div>
    </q-form>

  </q-page>

</template>

<script setup lang="ts">
import TipStrategyClient, { makeNewPayload } from 'src/api/v1/clients/StrategyClient'
import DateTimeComponent from 'src/components/DateTimeComponent.vue';
import { strategyTypeKeyValue, strategyTypeList } from 'src/general/lists';
import { useUserStore } from 'src/stores/user-store';
import { onMounted, reactive } from 'vue';

const userStore = useUserStore()
const props = defineProps<{ tournamentId: string, id?: string }>()
const client = TipStrategyClient(userStore.authHeader(), props.tournamentId)
const model = reactive({ data: makeNewPayload() })
const typeKV = strategyTypeKeyValue
const types = strategyTypeList.sort((a, b) => a.toString().localeCompare(b.toString()));

model.data.tournament_id = props.tournamentId;

const deleteRecord = () => { }
const saveRecord = () => { }

if (props.id) {
  onMounted(async () => {
    try {
      const response = await client.byId(props.id as string)
      console.log('response', response.data)
    } catch (e) {
      console.error(e)
    }
  })
}

console.log('here....', {
  ...props
})
</script>
