<template>
  <q-page padding>

    <transition appear enter-active-class="animated slideInDown" leave-active-class="animated slideOutUp">
      <div class="row justify-center">
        <div class="col-md-6 col-xs-12">
          <q-form @submit="saveInfoChanges">
            <q-card-section>
              <div class="text-h6">Login Crendential</div>
            </q-card-section>
            <q-card-section class="q-pt-none">
              <div class="row">
                <div class="col-xs-12 col-md-4 q-mb-md q-pr-md-md">
                  <q-input outlined label="Username" v-model="credential.username"
                    :rules="[val => val !== null && val.trim() !== '' || 'Username is required', val => val.toString().trim().length >= 3 && val.toString().trim().length <= 16 || 'Must be between 3 and 16 characters']" />
                </div>
                <div class="col-xs-12 col-md-4 q-mb-md q-pr-md-md">
                  <q-input outlined label="Password" v-model="credential.password" type="password" />
                </div>
                <div class="col-xs-12 col-md-4 q-mb-md">
                  <q-input outlined label="Confirm Password" v-model="confirmPassword" type="password" />
                </div>
              </div>
            </q-card-section>

            <q-separator inset />
            <q-card-actions align="right">
              <q-btn color="primary" type="submit">Save Changes</q-btn>
            </q-card-actions>
          </q-form>

          <q-form class="q-mt-md" @submit="saveInfoChanges">
            <q-card flat bordered class="my-card">
              <q-card-section>
                <div class="text-h6">Personal Information</div>
              </q-card-section>

              <q-card-section class="q-pt-none">
                <div class="row">
                  <div class="col-xs-12 col-md-5 q-mb-md q-pr-md-md">
                    <q-input outlined label="Email" v-model="credential.email" type="email"
                      :rules="[val => val !== null && val.trim() !== '' || 'Email is required']" />
                  </div>
                </div>
              </q-card-section>

              <q-separator inset />

              <q-card-actions align="right">
                <q-btn color="primary" type="submit">Save Changes</q-btn>
              </q-card-actions>
            </q-card>
          </q-form>

          <div class="q-mt-md">
            <q-card flat bordered class="my-card">
              <q-card-section class="bg-negative">
                <div class="text-h6">Application Information</div>
                <div class="text-subtitle2">This is how we identify you</div>
              </q-card-section>
              <q-card-section class="q-pt-sm">
                <div class="row">
                  <div class="col-11 q-mb-md q-pr-md-md">
                    <q-input outlined label="ID" v-model="userId" disable />
                  </div>
                  <div class="col-1 q-mb-md q-pr-md-md" v-if="clipboardApi.isSupported">
                    <q-btn flat dense icon="content_copy" class="q-mt-md" @click="() => copyToClipBoard(userId)">
                    </q-btn>
                  </div>
                  <!-- <div class="col-11 q-mb-md q-pr-md-md">
                    <q-input outlined label="API Token" v-model="userToken" disable />
                  </div> -->
                  <!-- <div class="col-1 q-mb-md q-pr-md-md" v-if="clipboardApi.isSupported">
                    <q-btn flat dense icon="content_copy" class="q-mt-md" @click="() => copyToClipBoard(userToken)">
                    </q-btn>
                  </div> -->
                </div>
              </q-card-section>
            </q-card>
          </div>

          <q-form class="q-mt-md">
            <q-card flat bordered class="my-card">
              <q-card-section>
                <div class="text-h6">Settings</div>
              </q-card-section>
              <q-card-section class="q-pt-none">
                <q-btn label="Delete my Account" color="negative" @click="deleteAccount" />
              </q-card-section>
            </q-card>
          </q-form>

        </div>
      </div>
    </transition>
  </q-page>

</template>

<script setup lang="ts">
import type { Ref } from 'vue';
import { onMounted, reactive, ref } from 'vue'
import { useClipboard } from '@vueuse/core'
import { useQuasar } from 'quasar'
import UserClient from 'src/api/v1/clients/UserClient'
import { useUserStore } from 'src/stores/user-store'
import type { CredentialUpdatePayload } from 'src/api/v1/CredentialUpdatePayload'


const userStore = useUserStore()
const userClient = new UserClient(userStore.authHeader())

const clipboardApi = useClipboard()
const q = useQuasar()

const credential = reactive({
  email: null,
  password: null,
  username: null

}) as CredentialUpdatePayload;

const confirmPassword: Ref<string | null> = ref(null)
const userId: Ref<string> = ref('')
// const userToken = ref('')


onMounted(async () => {
  try {
    const response = await userClient.myInfo()
    if (response.data) {
      userId.value = userStore.user?.id || ''
      credential.username = response.data.username;
      credential.email = userStore.user?.email || ''
      // credential.username = response.username
    }
  } catch (e) {
    q.dialog({
      title: 'Error fetching your data',
      message: (e as { message: string }).message
    })
  }
})

const saveInfoChanges = async () => {
  if ((credential.password || confirmPassword.value) && credential.password !== confirmPassword.value) {
    q.dialog({
      title: 'Password and confirmation do not match',
      message: 'Your password and the confirmation do not match'
    })
    return
  }

  try {
    const response = await userClient.updateMe(credential);
    if (response.data) {
      userStore.setLoginData(response.data);
    } else {
      q.dialog({
        title: 'Could not update your data',
        message: response.error || ''
      })
    }
  } catch (e) {
    q.dialog({
      title: 'Could not update your data',
      message: (e as { message: string }).message
    })
  }
}


const copyToClipBoard = async (source: string) => {
  await clipboardApi.copy(source)
  q.notify('copied')
}

const deleteAccount = () => {
  q.dialog({
    title: 'Delete your account?',
    message: 'You are about to delete your account',
    cancel: true
  }).onOk(() => { })
}

</script>
