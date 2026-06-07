import { defineStore } from 'pinia';
import { LocalStorage } from 'quasar';
import type { SignInReponse } from 'src/api/v1/SignInReponse';

const userKey = '_u';

export const useUserStore = defineStore('userStore', {
  state: () => {
    const data: SignInReponse | null = LocalStorage.getItem(userKey);
    return {
      activeToken: data ? data.token : '',
      vapid: data?.user.data.pushSubscription,
      loginData: data,
    }
  },
  getters: {
    isLogin: (state) => state.activeToken || false,
    isAdmin: (state) => {
      if (!state.loginData) {
        return false
      }
      return state.loginData.roles.findIndex((entry) => entry.name == 'administrator') > -1
    },
    avatar: (state) => `/assets/avartar/${state.loginData?.user.avatar || 'placeholder.png'}`,
    token: (state) => state.activeToken,
    user: (state) => state.loginData?.user
  },
  actions: {
    authHeader () {
      return new Headers([
        ['Authorization', `Bearer ${this.activeToken}`],
        ['Accept', 'application/json'],
        ['Content-Type', 'application/json'],
      ]);
    },
    prizes () {
      return [
        {
          image: '/prizes/1st_place.jpg',
          title: 'First Place',
          position: 1,
          description: ''
        },
        {
          image: '/prizes/2nd_place.jpg',
          title: 'Second Place',
          position: 2,
          description: ''
        },
        {
          image: '/prizes/3rd_place.jpg',
          title: 'Third Place',
          position: 3,
          description: ''
        }
      ]
    },
    setLoginData (data: SignInReponse) {
      this.activeToken = data.token
      LocalStorage.set(userKey, data);
      window.location.reload()
    },
    clearLoginData () {
      LocalStorage.clear()
      this.loginData = null;
    },
    loginWithToken (_token: string) {
      // TODO: fully implement
      return true;
    },
    requestPasswordReset (_email: string) {
      // TODO: fully implement
      return {
        data: {
          message: 'function needs implemenation'
        }
      }
    }
  },
});
