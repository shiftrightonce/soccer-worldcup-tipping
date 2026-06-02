<template>
  <div class="col-12"
    style="background-image: url('img/bg_3.jpg'); background-size: cover;background-position: center center;height: 100vh; min-height: 500px;">

    <div class="row justify-evenly q-mt-xl">
      <div class="col-md-9 col-xs-11 q-mt-md q-mt-xl">
        <div class="row">
          <div class="q-ml-auto col-md-6 col-xs-12 bg-blue-grey-10" style="border-radius: 10px">
            <LoginForm></LoginForm>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import LoginForm from 'src/components/login_signup/LoginForm.vue'

</script>
