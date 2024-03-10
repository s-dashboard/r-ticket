<script setup lang="ts">
  import Topbar from '@/components/Topbar.vue';
  import MainView from '@/views/MainView.vue';
</script>
<template>
    <div class="wrapper">
      <Topbar />
      <MainView />
    </div>
</template>
<style lang="scss">
  .wrapper {
      display: flex;
      width: 100%;
      height: 100%;
      flex-direction: column;
      align-items: stretch;
      padding: 12px 12px 0px 12px
  }

  #app {
    height: 100vh;
  }
</style>