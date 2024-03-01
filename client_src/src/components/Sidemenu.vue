<script lang="ts" setup>
    import { useRoute, useRouter } from 'vue-router';
    const router = useRouter(), 
        activeRoute = useRoute(),
        routes = router.getRoutes()
            .filter(item => item.meta.sideMenu);
</script>
<template>
    <ul class="nav flex-column nav-pills">
        <li class="nav-item" v-for="route in routes">
            <a class="nav-link"
                v-bind:class="activeRoute.path == route.path ? 'active' : ''"
                v-bind:href="route.path">
                {{ route.meta.title }}
            </a>
        </li>
    </ul>
</template>