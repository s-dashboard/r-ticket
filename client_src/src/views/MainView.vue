<script setup lang="ts">
  import { RouterView } from 'vue-router'
</script>
<template>
    <div class="d-flex container-fluid m-0 p-0 flex-1 align-items-stretch">
        <div class="row flex-1 bg-dark">
            <div class="col mt-2">
                <div>
                    <RouterView />
                </div>
            </div>
        </div>
    </div>
</template>