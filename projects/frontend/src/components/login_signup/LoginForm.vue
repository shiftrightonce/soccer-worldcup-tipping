<template>
  <q-form @submit="onSignIn" autofocus>
    <div class="row">
      <div class="col-xs-12 col-md-12 q-mt-md q-pl-lg q-pr-lg q-mb-md">
        <div class="text-h4">Login in</div>
      </div>
      <div class="col-xs-12 col-md-12 q-pl-lg q-pr-lg q-mb-md">
        <q-input
          v-model="username"
          label="Username"
          outlined
          @update:model-value="onInput"
        ></q-input>
      </div>
      <div class="col-xs-12 col-md-12 q-pl-lg q-pr-lg q-mb-md">
        <q-input
          v-model="password"
          label="Password"
          outlined
          @update:model-value="onInput"
          :type="isPassword ? 'password' : 'text'"
        >
          <template v-slot:append>
            <q-icon
              :name="isPassword ? 'visibility_off' : 'visibility'"
              class="cursor-pointer"
              @click="isPassword = !isPassword"
            />
          </template>
        </q-input>
      </div>
      <div class="col-xs-12 q-pl-lg q-pr-lg q-mb-md">
        <q-btn label="Login" color="primary" class="float-right" type="submit"></q-btn>
      </div>
      <div class="col-xs-12 q-pl-lg q-pr-lg q-mt-lg q-mb-md">
        <div class="text-caption">
          <q-btn
            flat
            dense
            label="Having issue logging in? Click here"
            no-caps
            :to="{ name: 'forgot-login' }"
          ></q-btn>
        </div>
      </div>
    </div>
  </q-form>
</template>

<script setup lang="ts">
import { useUserStore } from 'src/stores/user-store';
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useQuasar } from 'quasar';

const emit = defineEmits<{
  'update:username': [value: string];
  'update:password': [value: string];
  'update:modelValue': [data: { username: string; password: string }];
  signup: [];
  signin: [];
}>();

// const props = defineProps<{username: string, password: string}>()
const username = ref('');
const password = ref('');
const isPassword = ref(true);
const userStore = useUserStore();
const router = useRouter();
const $q = useQuasar();

const onInput = () => {
  emit('update:password', password.value);
  emit('update:username', username.value);
  emit('update:modelValue', {
    username: username.value,
    password: password.value,
  });
};

const onSignIn = () => {
  if (username.value && password.value) {
    void (async () => {
      try {
        userStore.login(username.value, password.value);
        await router.push({ name: 'scoreboard' });
      } catch (_e) {
        // @todo Make error message more helpful
        $q.dialog({
          title: 'Authentication failed',
          message: 'Username or password incorrect',
        });
      }
    })();
  }
};
</script>
