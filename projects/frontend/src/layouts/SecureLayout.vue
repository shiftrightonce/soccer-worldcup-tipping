<template>
  <q-layout view="hHr lpr fFr">
    <q-drawer
      show-if-above
      v-model="layoutStore.rightDrawer"
      side="right"
      overlay
      behavior="mobile"
      bordered
    >
      <MainMenu></MainMenu>
    </q-drawer>

    <q-page-container>
      <q-page class="row" style="background-color: #056480">
        <div class="col-12 q-pb-md" style="background-color: #056480; max-height: 110px">
          <div class="row justify-evenly">
            <div class="col-xs-12 col-md-8 items-center">
              <q-toolbar class="text-white q-mt-md">
                <AppLogo size="70px"></AppLogo>
                <q-toolbar-title class="text-positive text-bold"> </q-toolbar-title>
                <div class="gt-sm">
                  <div v-if="!userStore.isLogin">
                    <q-btn
                      flat
                      v-for="item in menuStore.public"
                      :to="item.to"
                      :icon="item.icon"
                      :key="item.label"
                    >
                      &nbsp;{{ item.label }}</q-btn
                    >
                    <q-btn
                      v-if="menuStore.shareIsSupported()"
                      flat
                      @click="shareApp"
                      label="Share"
                      icon="share"
                    >
                    </q-btn>
                  </div>
                  <div v-else>
                    <q-btn
                      flat
                      round
                      dense
                      icon="menu"
                      class="gt-sm"
                      @click="() => (layoutStore.rightDrawer = !layoutStore.rightDrawer)"
                    />
                  </div>
                </div>
                <q-btn
                  flat
                  round
                  dense
                  icon="menu"
                  class="lt-md"
                  @click="() => (layoutStore.rightDrawer = !layoutStore.rightDrawer)"
                />
              </q-toolbar>
            </div>
          </div>
        </div>

        <router-view />

        <div class="col-12" style="background-color: #056480">
          <FooterComponent></FooterComponent>
        </div>
      </q-page>
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import { useLayoutStore } from '../stores/layout-store';
import AppLogo from 'src/components/general/AppLogo.vue';
import MainMenu from 'src/components/general/MainMenu.vue';
import { useMenuStore } from 'src/stores/menu-store';
import FooterComponent from 'src/components/general/FooterComponent.vue';
import { useUserStore } from 'src/stores/user-store';

const layoutStore = useLayoutStore();
const menuStore = useMenuStore();
const userStore = useUserStore();

layoutStore.setTitle('Home');

const shareApp = async () => {
  await menuStore.shareApp();
};
</script>
