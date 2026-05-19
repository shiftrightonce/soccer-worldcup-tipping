<template>
  <q-layout view="hHh lpR fFf">
    <q-header reveal class="bg-primary text-white">
      <q-toolbar>
        <q-btn
          dense
          flat
          round
          icon="menu"
          @click="layoutStore.toggleLeftDrawer"
          v-if="layoutStore.isLeftDrawerEnabled"
        />

        <q-toolbar-title>
          <AppLogo></AppLogo>
          {{ layoutStore.title }}
        </q-toolbar-title>

        <q-btn
          dense
          flat
          round
          icon="menu"
          @click="layoutStore.toggleRighDrawer"
          v-if="layoutStore.isRightDrawerEnabled"
        />
      </q-toolbar>
    </q-header>

    <q-drawer
      v-if="userStore.isLogin"
      show-if-above
      v-model="layoutStore.leftDrawer"
      side="left"
      bordered
    >
      <q-list padding class="rounded-borders text-primary">
        <span v-for="item in menuStore.managementMenuItems" :key="item.label">
          <q-expansion-item
            v-if="item.children"
            :icon="item.icon"
            :label="item.label"
            group="manage-menu"
          >
            <q-list padding class="rounded-borders text-primary">
              <q-item
                clickable
                v-ripple
                v-for="child in item.children"
                :key="child.label"
                :to="child.to"
              >
                <q-item-section avatar>
                  <q-icon :name="child.icon"></q-icon>
                </q-item-section>
                <q-item-section>{{ child.label }}</q-item-section>
              </q-item>
            </q-list>
          </q-expansion-item>
          <q-item clickable v-ripple v-else :to="item.to">
            <q-item-section avatar>
              <q-icon :name="item.icon"></q-icon>
            </q-item-section>
            <q-item-section>{{ item.label }}</q-item-section>
          </q-item>
        </span>
      </q-list>
    </q-drawer>

    <q-drawer show-if-above v-model="layoutStore.rightDrawer" side="right" bordered>
      <MainMenu></MainMenu>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import { useMenuStore } from 'src/stores/menu-store';
import { useUserStore } from 'src/stores/user-store';
// import { useRouter } from 'vue-router'
import { useLayoutStore } from '../stores/layout-store';
import MainMenu from '../components/general/MainMenu.vue';
import AppLogo from 'src/components/general/AppLogo.vue';

const menuStore = useMenuStore();
// const router = useRouter()
const userStore = useUserStore();
// const menuStore = useMenuStore()
const layoutStore = useLayoutStore();

layoutStore.setTitle('Admin - Dashboard');

if (!userStore.isLogin || !userStore.isAdmin) {
  // router.push({ name: 'home' })
} else {
  // userStore.setupSocket()
}
</script>

<style lang="scss"></style>
