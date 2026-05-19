<template>
  <q-page class="row items-center justify-evenly">
    <h1>Admin Tips Page</h1>
    <FlagComponent country-code="au" width="100px" height="100px"></FlagComponent>
    <FlagComponent country-code="de"></FlagComponent>
    <FlagComponent country-code="jp" width="100px" height="100px"></FlagComponent>
  </q-page>
</template>

<script setup lang="ts">
import FlagComponent from 'components/FlagComponent.vue';
</script>
