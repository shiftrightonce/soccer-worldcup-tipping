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
            <p>
            <ul>
              <li>
                1 point for predicting the winner of a match
              </li>
              <li>
                1 point for predicting a draw
              </li>
              <li>
                5 points for prediction that a match will go into penalty
              </li>
              <li>
                If you predict the exact goal for each team, you will get a point for each goal. <br />
                <span style="color:#e64a19">Example</span>: <b>You predicted Australia to score 5 goals 😜 and France to
                  score 1 😒. You will get a total of
                  six points.</b>
              </li>
              <li>
                2 points for predicting the exact score of the match. <br />
                <span style="color:#e64a19">Example</span>: <b>You predictd the score 2 - 2</b>
              </li>
              <li>
                If you predicted the exact score in a penalty shootout, you will also get 1 point for each goal.
              </li>
            </ul>
            </p>
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
