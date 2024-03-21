<script setup lang="ts">
  import { RouterView } from 'vue-router'
  import Sidemenu from '@/components/Sidemenu.vue'
  import Breadcrumb from '@/components/Breadcrumb.vue'
</script>
<template>
    <div class="d-flex container-fluid m-0 p-0 flex-1 align-items-stretch">
        <div class="row flex-1">
            <Sidemenu />
            <div class="col mt-4">
                <div>
                    <Breadcrumb />
                    <RouterView />
                </div>
            </div>
        </div>
    </div>
</template>