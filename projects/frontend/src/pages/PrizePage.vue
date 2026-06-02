<template>
  <div class="col-12"
    style="background-image: url('img/bg_3.jpg'); background-size: cover;background-position: center center;overflow: auto;min-height: 100vh;">
    <div class="row items-center" style="height:100%">
      <div class="col-8 col-xs-12 q-xs-mt-md">
        <div class="row justify-center">
          <div class="col-auto q-mb-md q-mr-md-md q-mt-xs-xl">
            <div class="text-h2 text-bold text-positive">Prizes</div>
          </div>
        </div>
        <div class="row justify-center">
          <div class="col-md-2 col-xs-11 q-mb-md q-mr-md-md" v-for="(prize, index) in data.prizes" :key="index">
            <PrizeCard :prize="prize"></PrizeCard>
          </div>

          <div class="col-12 lt-md q-mb-xl"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import PrizeCard from 'src/components/general/PrizeCard.vue'
import { useLayoutStore } from 'src/stores/layout-store'
import { useUserStore } from 'src/stores/user-store'
import { reactive } from 'vue'

const data = reactive<{ prizes: Array<{ image: string, title: string, position: number, description: string }> }>({ prizes: [] })
const userStore = useUserStore()
const layoutStore = useLayoutStore()

layoutStore.activeLeftDrawer(false)
layoutStore.activeRightDrawer(false);

data.prizes = userStore.prizes()
</script>
