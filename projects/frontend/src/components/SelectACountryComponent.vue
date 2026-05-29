<template>
  <q-select outlined v-model="model" :options="countries" map-options emit-value option-value="id" option-label="name"
    :label="props.label">
    <template v-slot:option="scope">
      <q-item v-bind="scope.itemProps">
        <q-item-section avatar>
          <flag-component :country-code="scope.opt.alpha2" width="25px" height="18px" />
        </q-item-section>
        <q-item-section>
          <q-item-label>{{ scope.opt.name }}</q-item-label>
        </q-item-section>
      </q-item>
    </template>
  </q-select>
</template>

<script setup lang="ts">
import type { Country } from 'src/api/Country';
import FlagComponent from './FlagComponent.vue';

const props = defineProps<{ countries: Array<Country>, label: string }>()
const model = defineModel<string | null>()
</script>
