<template>
  <q-toolbar>
    <ProfileImage v-if="userStore.isLogin"></ProfileImage>
    <q-toolbar-title>Main Menu</q-toolbar-title>
  </q-toolbar>
  <q-list v-if="userStore.isLogin" padding class="rounded-borders text-primary">
    <q-item clickable v-ripple v-for="item in menuStore.main" :key="item.label" :to="item.to">
      <q-item-section avatar>
        <q-icon :name="item.icon"></q-icon>
      </q-item-section>
      <q-item-section>{{ item.label }}</q-item-section>
    </q-item>
    <q-item clickable v-ripple v-if="menuStore.shareIsSupported()" @click="menuStore.shareApp">
      <q-item-section avatar>
        <q-icon name="share"></q-icon>
      </q-item-section>
      <q-item-section>Share</q-item-section>
    </q-item>
    <q-separator v-if="userStore.isAdmin" />
    <q-item-label v-if="userStore.isAdmin" header>Adminstration</q-item-label>
    <div v-if="userStore.isAdmin">
      <q-item clickable v-ripple v-for="item in menuStore.admin" :key="item.label" :to="item.to">
        <q-item-section avatar>
          <q-icon :name="item.icon"></q-icon>
        </q-item-section>
        <q-item-section>{{ item.label }}</q-item-section>
      </q-item>
    </div>
    <q-separator v-if="userStore.isAdmin" />
    <q-item clickable v-ripple v-for="item in menuStore.user" :key="item.label" :to="item.to">
      <q-item-section avatar>
        <q-icon :name="item.icon"></q-icon>
      </q-item-section>
      <q-item-section>{{ item.label }}</q-item-section>
    </q-item>
  </q-list>
  <q-list v-else>
    <q-item clickable v-ripple v-for="item in menuStore.public" :key="item.label" :to="item.to">
      <q-item-section avatar>
        <q-icon :name="item.icon"></q-icon>
      </q-item-section>
      <q-item-section>{{ item.label }}</q-item-section>
    </q-item>
    <q-item clickable v-ripple v-if="menuStore.shareIsSupported()" @click="menuStore.shareApp">
      <q-item-section avatar>
        <q-icon name="share"></q-icon>
      </q-item-section>
      <q-item-section>Share</q-item-section>
    </q-item>
  </q-list>
</template>

<script lang="ts">
import { useMenuStore } from 'src/stores/menu-store';
import { useUserStore } from 'src/stores/user-store';
import { defineComponent } from 'vue';
import ProfileImage from '../user/ProfileImage.vue';
export default defineComponent({
  name: 'MainMenu',
  setup() {
    const menuStore = useMenuStore();
    const userStore = useUserStore();
    return {
      menuStore,
      userStore,
    };
  },
  components: { ProfileImage },
});
</script>
