import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

const APP_VERSION = '0.0.6';
let watchingForUpdate = false;

export const useLayoutStore = defineStore('layoutStore', {
  state: () => ({
    layoutTitle: 'Default Layout Title',
    version: APP_VERSION,
    leftDrawer: ref(true),
    rightDrawer: ref(false),
    enableLeftDrawer: ref(true),
    enableRightDrawer: ref(true),
    backgroundImage: '',
  }),
  getters: {
    title: (state) => state.layoutTitle,
    appVersion: (state) => state.version,
    isLeftDrawerEnabled: (state) => state.enableLeftDrawer,
    isRightDrawerEnabled: (state) => state.enableRightDrawer,
  },
  actions: {
    setTitle (title: string) {
      this.layoutTitle = title;
    },
    activeLeftDrawer (enable = true) {
      this.enableLeftDrawer = enable;
      this.leftDrawer = enable;
    },
    activeRightDrawer (enable = true) {
      this.enableRightDrawer = enable;
      this.rightDrawer = enable;
    },
    toggleLeftDrawer () {
      if (this.isLeftDrawerEnabled) {
        this.leftDrawer = !this.leftDrawer;
      }
    },
    toggleRighDrawer () {
      this.rightDrawer = !this.rightDrawer;
    },
    leftDrawerComputed () {
      return computed(() => this.leftDrawer);
    },
    onAppUpdate (callback: () => void) {
      if (!watchingForUpdate) {
        watchingForUpdate = true;
        const channel = new BroadcastChannel('world-cup-tipping');
        channel.onmessage = (event) => {
          if (event.data && event.data.type === 'client:app-update-found') {
            callback();
          }
        };
      }
    },
  },
});
