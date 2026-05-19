<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-9">
        <div class="text-h6">Groups</div>
        <div class="text-body2 text-muted-foreground">{{ props.id ? 'Edit': 'New'  }} Group</div>
      </div>
      <div class="col-1">
        <q-btn color="primary" icon="add" outline no-caps label="Save" @click="saveRecord"></q-btn>
      </div>
      <div class="col-2">
        <q-btn
          v-show="props.id"
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
         <div class="row"></div>
      </q-form>
  </q-page>
</template>

<script setup lang="ts">
import { ref } from 'vue';

   const  props  = defineProps<{tournamentId?: string, id?:  string}>()
   const form = ref(null)

console.log({
  ...props
});

const saveRecord = () => {}
const deleteRecord = () => {}
</script>
