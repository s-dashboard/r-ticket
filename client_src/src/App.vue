<script setup lang="ts">
  import { RouterView } from 'vue-router'
  import Topbar from './components/Topbar.vue'
  import Sidemenu from './components/Sidemenu.vue'
  import Breadcrumb from './components/Breadcrumb.vue'
</script>

<template>
  <div class="container-fluid">
    <Topbar />
    <div class="row">
      <div class="side-menu col">
        <Sidemenu />
      </div>
      <div class="col">
        <Breadcrumb />
        <RouterView />
      </div>
    </div>
  </div>
</template>
<style lang="scss">
  .side-menu {
    max-width: 250px;
  }
</style>