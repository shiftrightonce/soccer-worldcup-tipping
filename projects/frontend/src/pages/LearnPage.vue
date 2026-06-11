<template>
  <div class="col-12 bg-blue-grey-10" style="min-height: 100vh;">
    <div class="row itemss-center" style="height:100%">
      <div class="col-8 col-xs-12 q-mt-xl">
        <div class="row justify-center">
          <div class="col-auto q-mb-md q-mr-md-md">
            <div class="text-h2 text-bold text-positive">Learn</div>
          </div>
        </div>
        <div class="row justify-center">
          <div class="col-auto q-mb-md q-mr-md-md q-pa-md">
            <div class="text-h4 text-bold text-deep-orange-8">Scoring System</div>
            <ul>
              <li>
                <b>Match Winner:</b> <br />
                &nbsp; &nbsp; 5 points
              </li>
              <li>
                <b>Group Ranking:</b> <br />
                &nbsp; &nbsp; 5 points for each
              </li>
              <li>
                <b>Goals:</b> <br />
                &nbsp; &nbsp; 10 points if both teams predication are right<br />
                &nbsp; &nbsp; 5 points if either predication is right
              </li>
              <li>
                <b>Cup Winner:</b> <br />
                &nbsp; &nbsp; 10 points
              </li>
              <li>
                <b>Game to Penalty:</b> <br />
                &nbsp; &nbsp; 5 points
              </li>
              <li>
                <b>First Red or Yellow card:</b> <br />
                &nbsp; &nbsp; 5 points
              </li>
              <li>
                <b>Penalty Goals:</b> <br />
                &nbsp; &nbsp; 10 points if both teams predication are right<br />
                &nbsp; &nbsp; 5 points if either predication is right
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { useLayoutStore } from 'src/stores/layout-store'
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'LearnPage',
  setup () {
    const layoutStore = useLayoutStore()
    layoutStore.activeLeftDrawer(false)
    layoutStore.activeRightDrawer(false)

    return {}
  }
})
</script>
