import { defineStore } from 'pinia';

export const useCountryStore = defineStore('countryStore', {
  state: () => ({
    countries: [
      {
        "id": "019e3a0e-3b4d-71d1-9727-0e4679199f76",
        "name": "Afghanistan",
        "alpha2": "AF",
        "alpha3": "AFG",
        "tournaments": null,
        "groups": null,
        "coutryGroup": null,
        "createdAt": new Date("2026-05-18T07:47:51.757536211Z"),
        "updatedAt": null,
        "deletedAt": null
      },
      {
        "id": "019e3a0e-3b51-7e52-b05d-0eda097f3e1a",
        "name": "Albania",
        "alpha2": "AL",
        "alpha3": "ALB",
        "tournaments": null,
        "groups": null,
        "coutryGroup": null,
        "createdAt": new Date("2026-05-18T07:47:51.761857005Z"),
        "updatedAt": null,
        "deletedAt": null
      },
    ]
  }),
  getters: {
    countriesList: (state) => state.countries,
    countriesAsOptions: (state) => {
      return Object.fromEntries(state.countries.map((entry) => [entry.id, entry]))
    }
  }
})
