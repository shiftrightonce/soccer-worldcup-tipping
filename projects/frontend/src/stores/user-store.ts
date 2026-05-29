import { defineStore } from 'pinia';
import { LocalStorage } from 'quasar';
import type { User } from 'src/api/User';

const tokenKey = '_t';
const vapidKey = 'vapid';
const token =
  'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOm51bGwsInN1YiI6IjAxOWU2YTA0LWE1OGYtNzcyMS05OTU4LWIwZjIwNjAyY2UyNSIsImV4cCI6MTc4MjQ4NzM5MywiaWF0IjoxNzc5ODk1MzkzLCJuYmYiOjE3Nzk4OTUzOTMsImp0aSI6IjFmZWQzYTViMGE0NjNiODI1N2QyMGQ5OTE1ZDBiOGRiNjE1NWE1NmRlNDY4NjFhOTE0NDE3OTdlYjkwNTNlNzUiLCJpc3MiOm51bGwsInByaXZhdGUiOnsiX2FyIjoiMDE5ZTZhMDQtYTE1My03YzAzLThmMWUtMjAyOTJhN2E3NjBlIn19.x88Wy-icyhrpXm-z-WK8JkNF8EMSUH0DzwybKOiYM1o';

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
