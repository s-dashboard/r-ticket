<script lang="ts" setup>
    import { useRouter } from 'vue-router';

    const router = useRouter(), 
        routes = router.options.routes.filter(p => p.meta?.topMenu);
</script>
<template>
    <nav aria-label="breadcrumb">
        <ol class="breadcrumb">
            <template v-for="route in routes">
                <li v-if="($route.path.indexOf(route.path) >= 0 && route.path !== '/') || route.path === $route.path"
                    class="breadcrumb-item active" 
                    aria-current="page"
                >
                    <a :href="route.path">{{ route.meta?.title }}</a>
                </li>
            </template>
            <!-- <li class="breadcrumb-item active" aria-current="page">Data</li> -->
        </ol>
    </nav>
</template>
<style lang="scss">
</style>