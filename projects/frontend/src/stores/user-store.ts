import { defineStore } from 'pinia';
import { LocalStorage } from 'quasar';
import type { User } from 'src/api/User';

const tokenKey = '_t';
const vapidKey = 'vapid';
const token =
  'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOm51bGwsInN1YiI6IjAxOWUxZjRhLTNkMDktN2VhMi1iZjk0LTNiZGMyNGQxMDIxYyIsImV4cCI6MTc4MTI2NDgyNiwiaWF0IjoxNzc4NjcyODI2LCJuYmYiOjE3Nzg2NzI4MjYsImp0aSI6IjJmYTc3ZGE4NGUyMjQzNzY5MDNmMjg3NWVmNzM4NGUyYzdlMjc3NmI0MGRlOTBkOWM5ZWE3YTUwOGJmM2M1NzAiLCJpc3MiOm51bGwsInByaXZhdGUiOnsiX2FyIjoiMDE5ZTFmNGEtMzhiYS03ZGEzLWI0ZTUtYmQxMWIzOTAzOTFkIn19.y84KtPptmRQRkp0Rl1X44eLmrKOLdHYHcYWrnYqCai0';

export const useUserStore = defineStore('userStore', {
  state: () => ({
    activeToken: LocalStorage.getItem(tokenKey),
    vapid: LocalStorage.getItem(vapidKey),
    user: null as null | User,
  }),
  getters: {
    isLogin: () => true,
    isAdmin: () => true,
    avatar: (state) => `/public/user/${state.user?.id || 'placeholder'}.png`,
    token: (state) => state.activeToken,
  },
  actions: {
    login(username: string, password: string) {
      console.log('do login', { username, password });
    },
    authHeader() {
      return new Headers([
        ['Authorization', `Bearer ${token}`],
        ['Accept', 'application/json'],
        ['Content-Type', 'application/json'],
      ]);
    },
  },
});
