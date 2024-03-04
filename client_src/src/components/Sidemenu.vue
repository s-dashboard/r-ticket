<script lang="ts" setup>
    import { useRoute, useRouter, type RouteRecordNormalized } from 'vue-router';
    const router = useRouter(), 
        activeRoute = useRoute(); 

    const routes = router.options.routes.filter(p => p.meta?.sideMenu);
</script> 
<template>
    <ul class="nav flex-column nav-pills">
        <li class="nav-item" v-for="route in routes">
            <a class="nav-link" v-bind:href="route.path">
                {{ route.meta?.title }}
            </a>
            <template v-if="route.children != null && route.children.length > 0">
                <ul class="nav nav-level-1 flex-column nav-pills">
                    <li class="nav-item" v-for="childRoute in route.children?.filter(p => p.meta?.showMenu)">
                        <a class="nav-link" v-bind:href="childRoute.path">
                            {{ childRoute.meta?.title || childRoute.name }}
                        </a>
                    </li>
                </ul>
            </template>
        </li>
    </ul>
</template>
<style lang="scss">

    .nav {
        &-level-1 {
            margin-left: 15px;
            font-size: 0.8em;
        }
    }

</style>