<template>
  <div class="WAL position-relative" :style="style">
    <q-layout view="lHr LpR lFr" v-if="isReady">
      <q-header elevated>
        <q-toolbar>
          <q-btn round flat icon="menu" class="WAL__drawer-open q-mr-sm" @click="toggleLeftDrawer" />

          <AppLogo></AppLogo>
          <q-btn round flat>
            <!-- <q-avatar>
              <img :src="currentConversation.avatar">
            </q-avatar> -->
          </q-btn>

          <span class="q-subtitle-1 q-pl-md">
            <!-- {{ currentConversation.name }} -->
          </span>

          <q-space />
          <q-btn flat round dense icon="menu" @click="toggleRightDrawer" />
        </q-toolbar>
      </q-header>

      <q-drawer v-model="leftDrawerOpen" show-if-above side="left" bordered :breakpoint="690">
        <q-toolbar>
          <q-avatar class="cursor-pointer">
            <ProfileImage></ProfileImage>
          </q-avatar>
          User Name Here
          <q-space />
          <q-btn round flat icon="close" class="WAL__drawer-close" @click="toggleLeftDrawer" />
        </q-toolbar>

        <q-toolbar cllass="bg-grey-2">
          <q-input rounded outlined dense class="WAL__field full-width" v-model="search"
            placeholder="Search or start a new conversation">
            <template v-slot:prepend>
              <q-icon name="search" />
            </template>
          </q-input>
        </q-toolbar>

        <!-- <q-scroll-area style="height: calc(100% - 100px)"> -->
        <q-list>
          <q-item v-for="(conversation, index) in state" :key="conversation.id" clickable v-ripple
            @click="setCurrentConversation(index)">
            <q-item-section avatar>
              <q-avatar>
                <img :src="conversation.avatar">
              </q-avatar>
            </q-item-section>

            <q-item-section>
              <q-item-label lines="2">
                <!-- {{ conversation.name }} -->
                Name...
              </q-item-label>
              <q-item-label class="conversation__summary" caption>
              </q-item-label>
            </q-item-section>

            <q-item-section side>
              <!-- <q-badge rounded color="green" v-if="conversation.internalId === currentConversation.internalId" /> -->
              <q-badge rounded color="green" v-if="true" />
            </q-item-section>
          </q-item>
        </q-list>
        <!-- </q-scroll-area> -->
      </q-drawer>

      <q-drawer show-if-above v-model="rightDrawerOpen" :breakpoint="690" side="right" bordered>
        <MainMenu></MainMenu>
      </q-drawer>

      <q-page-container>
        <!-- <q-page class="q-pa-md"> -->
        <router-view />
        <!-- </q-page> -->
      </q-page-container>

      <q-footer>
        <q-toolbar class="bg-grey-6 text-black row">
          <!-- <q-btn round flat icon="insert_emoticon" class="q-mr-sm" /> -->
          <q-input rounded outlined dense class="WAL__field col-grow q-mr-sm text-black" :dark="false" bg-color="white"
            v-model="message" placeholder="Type a message" @keydown="onKeyDown" @keyup="onKeyUp" />
          <q-btn round flat icon="send" @click="sendMessage" />
        </q-toolbar>
      </q-footer>
    </q-layout>
  </div>
</template>

<script setup lang="ts">
import { useQuasar } from 'quasar'
// import { useMenuStore } from 'src/stores/menu-store'
import { useUserStore } from 'src/stores/user-store'
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import ProfileImage from '../components/user/ProfileImage.vue'
import MainMenu from '../components/general/MainMenu.vue'
import { useChatStore } from 'src/stores/chat-store'
import AppLogo from 'src/components/general/AppLogo.vue'

console.log('-------------------------------------');
const $q = useQuasar()

const leftDrawerOpen = ref(false)
const rightDrawerOpen = ref(false)
const router = useRouter()
const userStore = useUserStore()
// const menuStore = useMenuStore()

const search = ref('')
const message = ref('')
const currentConversationIndex = ref(0)
const chatStore = useChatStore()
const { isReady, state } = await chatStore.fetchRooms({
  onError: (e) => {
    if ((e as Error).message.indexOf('401') >= 0) {
      // console.log('very good')
      // userStore.logout().then(() => {
      //   void router.push({
      //     name: 'home'
      //   })
      // }).catch((e) => {
      //   console.log('error...', e)
      // })
    }
  }
})

// const currentConversation = computed(() => {
//   return state.value[currentConversationIndex.value]
// })

const style = computed(() => ({
  height: $q.screen.height + 'px'
}))

if (!userStore.isLogin) {
  await router.push({ name: 'home' })
} else {
  userStore.setupSocket()
}

function toggleLeftDrawer () {
  leftDrawerOpen.value = !leftDrawerOpen.value
}

function toggleRightDrawer () {
  rightDrawerOpen.value = !rightDrawerOpen.value
}

function setCurrentConversation (index: number) {
  // chatStore.currentRoom = state.value[index]
  currentConversationIndex.value = index
}

const sendMessage = () => {
  if (message.value.trim()) {
    // (async () => {
    //   void await chatStore.postMessage(currentConversation.value.internalId, message.value.trim())
    //   message.value = ''
    // })()
  }
}

const keysHistory: Record<string, boolean> = {}
const onKeyDown = (e: KeyboardEvent) => {
  keysHistory[e.key] = true
  handleKeyEvent()
}

const onKeyUp = (e: KeyboardEvent) => {
  delete keysHistory[e.key]
}

const handleKeyEvent = () => {
  if (!keysHistory.Shift && keysHistory.Enter) { // User press the enter key without holding down the shift key
    sendMessage()
  }
}

// export default defineComponent({
//   name: 'GroupChatLayout',
//   components: { ProfileImage, MainMenu, AppLogo },
//   setup () {
//     const $q = useQuasar()

//     const leftDrawerOpen = ref(false)
//     const rightDrawerOpen = ref(false)
//     const router = useRouter()
//     const userStore = useUserStore()
//     const menuStore = useMenuStore()

//     const search = ref('')
//     const message = ref('')
//     const currentConversationIndex = ref(0)
//     const chatStore = useChatStore()
//     const { isReady, state } = chatStore.fetchRooms({
//       onError: (e) => {
//         if ((e as Error).message.indexOf('401') >= 0) {
//           console.log('very good')
//           userStore.logout().then(() => {
//             router.push({
//               name: 'home'
//             })
//           })
//         }
//       }
//     })

//     const currentConversation = computed(() => {
//       return state.value[currentConversationIndex.value]
//     })

//     const style = computed(() => ({
//       height: $q.screen.height + 'px'
//     }))

//     if (!userStore.isLogin) {
//       router.push({ name: 'home' })
//     } else {
//       userStore.setupSocket()
//     }

//     function toggleLeftDrawer () {
//       leftDrawerOpen.value = !leftDrawerOpen.value
//     }

//     function toggleRightDrawer () {
//       rightDrawerOpen.value = !rightDrawerOpen.value
//     }

//     function setCurrentConversation (index: number) {
//       // chatStore.currentRoom = state.value[index]
//       currentConversationIndex.value = index
//     }

//     const sendMessage = () => {
//       if (message.value.trim()) {
//         // (async () => {
//         //   void await chatStore.postMessage(currentConversation.value.internalId, message.value.trim())
//         //   message.value = ''
//         // })()
//       }
//     }

//     const keysHistory: Record<string, boolean> = {}
//     const onKeyDown = (e: KeyboardEvent) => {
//       keysHistory[e.key] = true
//       handleKeyEvent()
//     }

//     const onKeyUp = (e: KeyboardEvent) => {
//       delete keysHistory[e.key]
//     }

//     const handleKeyEvent = () => {
//       if (!keysHistory.Shift && keysHistory.Enter) { // User press the enter key without holding down the shift key
//         sendMessage()
//       }

//       // @todo handle new line hint
//     }

//     return {
//       isReady,
//       state,
//       leftDrawerOpen,
//       search,
//       message,
//       currentConversationIndex,
//       menuStore,
//       rightDrawerOpen,
//       style,
//       currentConversation,
//       userStore,
//       setCurrentConversation,
//       toggleLeftDrawer,
//       toggleRightDrawer,
//       sendMessage,
//       onKeyDown,
//       onKeyUp
//     }
//   }
// })
</script>

<style lang="sass">
.WAL
  width: 100%
  height: 100%

  &__layout
    margin: 0 auto
    z-index: 4000
    height: 100%
    width: 90%
    max-width: 950px
    border-radius: 5px

  &__field.q-field--outlined .q-field__control:before
    border: none

  .q-drawer--standard
    .WAL__drawer-close
      display: none

@media (max-width: 850px)
  .WAL
    padding: 0
    &__layout
      width: 100%
      border-radius: 0

@media (min-width: 691px)
  .WAL
    &__drawer-open
      display: none

.conversation__summary
  margin-top: 4px

.conversation__more
  margin-top: 0!important
  font-size: 1.4rem
</style>
