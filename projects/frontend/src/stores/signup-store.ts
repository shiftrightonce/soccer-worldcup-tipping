import { defineStore } from 'pinia';

export type SignupData = {
  username: string;
  password: string;
  confirmPassword: string;
  email: string;
};

export const useSignupStore = defineStore('signupStore', {
  state: () => ({
    model: {
      username: '',
      password: '',
      confirmPassword: '',
      email: '',
    },
  }),
  actions: {
    async signup() {
      // TODO: Fully implement sigining up
      return new Promise((resolve, reject) => {
        if (this.model.username) {
          return resolve(true);
        } else {
          return reject(Error('username not set'));
        }
      });
    },
  },
});
