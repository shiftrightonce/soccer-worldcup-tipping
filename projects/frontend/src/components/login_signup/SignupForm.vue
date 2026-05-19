<template>
  <q-form @submit="onSubmit">
    <div class="row">
      <div class="col-12 q-mb-sm">
        <div class="text-h4">Sign up</div>
      </div>
      <div class="col-md-4 col-xs-12 q-mb-sm">
        <q-input
          label="Username *"
          dense
          outlined
          v-model="model.username"
          :rules="[
            (val) => (val !== null && val.trim() !== '') || 'Username is required',
            (val) => val.toString().length <= 50 || 'Must be 50 characters or less',
          ]"
          style="max-width: 420px"
        >
        </q-input>
      </div>
      <div class="col-12"></div>
      <div class="col-md-4 col-xs-12 q-mb-sm q-mr-sm">
        <q-input
          label="Password *"
          dense
          outlined
          v-model="model.password"
          :rules="[(val) => (val !== null && val.trim() !== '') || 'Password is required']"
          type="password"
          style="max-width: 420px"
        ></q-input>
      </div>
      <div class="col-md-5 col-xs-12 q-mb-sm">
        <q-input
          label="Confirm Password *"
          dense
          outlined
          v-model="model.confirmPassword"
          :rules="[(val) => (val && val === model.password) || 'Confirmation does not match']"
          type="password"
          style="max-width: 420px"
        ></q-input>
      </div>
      <div class="col-12"></div>
      <div class="col-md-4 col-xs-12 q-mb-sm q-mr-sm">
        <q-input
          label="Email *"
          dense
          outlined
          v-model="model.email"
          :rules="[(val) => (val !== null && val.trim() !== '') || 'Email is required']"
          type="email"
          style="max-width: 420px"
        ></q-input>
      </div>
      <div class="col-md-5 col-xs-12 q-mb-sm">
        <q-input
          :label="number + ' + ' + add + ' ='"
          dense
          outlined
          v-model="answer"
          :rules="[(val) => Number(val) === number + add || 'Answer is incorrect']"
          style="max-width: 420px"
          type="number"
          hint="Answer this math question"
        ></q-input>
      </div>
      <div class="col-12 q-mb-sm">
        <q-btn label="Sign up" color="positive" type="submit"></q-btn>
      </div>
    </div>
  </q-form>
</template>

<script setup lang="ts">
import { useLayoutStore } from 'src/stores/layout-store';
import { ref } from 'vue';
import { useSignupStore } from 'src/stores/signup-store';
import { useRouter } from 'vue-router';

const answer = ref('');
const layoutStore = useLayoutStore();
const number = ref(0);
const add = ref(0);
const signupStore = useSignupStore();
const router = useRouter();
const model = signupStore.model;

const generateQuestion = () => {
  number.value = Math.floor(Math.random() * 30000);
  add.value = Math.floor(Math.random() * 11);
};

const onSubmit = () => {
  signupStore
    .signup()
    .then((result) => {
      if (result) {
        void router.push({
          name: 'scoreboard',
        });
      }
    })
    .catch(() => {
      void router.push({
        name: 'login',
      });
    });
};

layoutStore.setTitle('Sign up');
generateQuestion();
</script>
