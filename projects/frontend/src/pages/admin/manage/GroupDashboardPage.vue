<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10">
        <div class="text-h6">Groups</div>
        <div class="text-body2 text-muted-foreground">Manage Groups</div>
      </div>

      <div class="col-2">
        <q-btn
          color="primary"
          icon="add"
          outline
          no-caps
          label="New Group"
          :to="{ name: 'manage-group'}"
        ></q-btn>
      </div>

      </div>
  </q-page>
</template>

<script setup lang="ts"></script>
