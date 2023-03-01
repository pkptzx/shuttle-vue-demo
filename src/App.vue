<template>
  <div class="btn-group">
  <router-link class="btn" active-class="btn-active" to="/" tag="button">二维码解析</router-link>
  <router-link class="btn" active-class="btn-active" to="/speechrecognition" tag="button">语音识别</router-link>
  <router-link class="btn" active-class="btn-active" to="/canvasdrawgif" tag="button">绘制gif到canvas上</router-link>
  <router-link to="/docs" class="btn" active-class="btn-active" target="_blank">api doc demo</router-link>
</div>
  <router-view></router-view>
</template>
<script setup lang="ts">

</script>
<style>
@import url('https://lf9-cdn-tos.bytecdntp.com/cdn/expire-1-M/tailwindcss/2.2.19/tailwind.min.css');
@import "https://lf9-cdn-tos.bytecdntp.com/cdn/expire-1-M/daisyui/2.2.2/full.min.css";
</style>