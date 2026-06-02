<template>
  <div class="col-12 bg-blue-grey-10" style="min-height: 100vh;">
    <div class="row itemss-center" style="height:100%">
      <div class="col-8 col-xs-12 q-mt-xl">
        <div class="row justify-center">
          <div class="col-auto q-mb-lg q-mr-md-md q-mt-xs-xl">
            <div class="text-h2 text-bold text-positive">Contact Me</div>
          </div>
        </div>
        <div class="row justify-center">
          <div class="col-md-2 col-xs-11 q-mb-md q-mr-md-md">
            <q-card flat bordered class="my-card">
              <q-card-section class="q-pt-md">
                <div class="q-mb-sm">
                  <span class="text-bold text-deep-orange-8">Email</span> <br />
                  jibaomansaray at kpama dot com dot au
                </div>
                <div class="q-mb-sm">
                  <span class="text-bold text-deep-orange-8">Twitter</span> <br />
                  @JibaoMansaray
                </div>
                <div class="q-mb-sm">
                  <span class="text-bold text-deep-orange-8">Github</span> <br />
                  <a href="https://github.com/jibaomansaray" target="_blank">jibaomansaray</a>
                </div>
              </q-card-section>

              <q-separator inset />

              <q-card-section>
                <div class="q-mb-sm">
                  <span class="text-bold text-deep-orange-8">Full Source Code</span> <br />
                  <a href="https://github.com/jibaomansaray/worldcup_tipping" target="_blank">worldcup_tipping</a>
                </div>
              </q-card-section>
            </q-card>
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
  name: 'ContactPage',
  setup () {
    const layoutStore = useLayoutStore()

    layoutStore.activeLeftDrawer(false)
    layoutStore.activeRightDrawer(false)

    return {}
  }
})
</script>
