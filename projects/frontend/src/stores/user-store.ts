import { defineStore } from 'pinia';
import { LocalStorage } from 'quasar';
import type { User } from 'src/api/User';

const tokenKey = '_t';
const vapidKey = 'vapid';
const token =
  'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOm51bGwsInN1YiI6IjAxOWU0ZmE4LTNiZjAtNzk5Mi05YzdhLWUyOWY1OTA2NmZjMSIsImV4cCI6MTc4MjA0NDkwMSwiaWF0IjoxNzc5NDUyOTAxLCJuYmYiOjE3Nzk0NTI5MDEsImp0aSI6ImRiZmJlNDFkMTE1NGYyNGY2NDA2MzI1YjU3NmQ1ZDI4Zjg4YmQ3YjI3NmY2Yzk3Nzk2YzgxMThlNjg1NDJmMTkiLCJpc3MiOm51bGwsInByaXZhdGUiOnsiX2FyIjoiMDE5ZTRmYTgtMzc4ZC03OTkyLWFhYjAtMWNiNDU1YmZkODA1In19.YTUDnCZK5C6h3RkZIHl356Rjz-BX-t0Ht832jTfI2Uw';

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
    login (username: string, password: string) {
      console.log('do login', { username, password });
    },
    authHeader () {
      return new Headers([
        ['Authorization', `Bearer ${token}`],
        ['Accept', 'application/json'],
        ['Content-Type', 'application/json'],
      ]);
    },
  },
});
