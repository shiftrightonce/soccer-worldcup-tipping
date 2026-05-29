import { defineStore } from "pinia";
import type { Game } from "src/api/Game";
import GameClient from "src/api/v1/clients/GameClient";
import { useUserStore } from "./user-store";

const userStore = useUserStore();
export const useGameStore = defineStore('gameStore', {
  state: () => ({
    _cache: {},
  }),
  getters: {},
  actions: {
    fetchById (gameId: string, tournamentId: string) {
      const cached = (this._cache as Record<string, Game>)[gameId];
      if (cached) {
        return Promise.resolve(cached);
      }
      const gameClient = GameClient(userStore.authHeader(), tournamentId);
      return gameClient.byId(gameId).then((response) => {
        if (response.data) {
          (this._cache as Record<string, Game>)[gameId] = response.data;
          return response.data;
        }
        throw new Error('Game not found');
      });

    },
  },
});
