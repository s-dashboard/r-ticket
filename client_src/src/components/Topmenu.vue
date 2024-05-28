<script lang="ts" setup>
    import { useRouter } from 'vue-router';

    const router = useRouter(), 
        routes = router.options.routes.filter(p => p.meta?.topMenu);

        const signOut = (e: any) => {
            localStorage.removeItem('TICKETS_TOKEN');
            router.push({
                path: '/'
            });
            location.reload();
            return false;
        };
</script> 
<template>
    <nav class="topmenu navbar navbar-light bg-dark">
        <ul class="nav nav-tabs flex-1" data-bs-theme="dark">
            <li class="nav-item" v-for="route in routes" role="presentation">
                <a class="nav-link"
                    :class="{ 'active': ($route.path.indexOf(route.path) >= 0 && route.path !== '/') || route.path === $route.path }"
                    v-bind:href="route.path" data-toggle="collapse" aria-expanded="false" :title="route.meta?.title">
                    <i :class="'fas ' + route.meta?.iconCls"></i> <span>{{ route.meta?.title }}</span>
                </a>
            </li>
        </ul>
        <ul class="nav nav-tabs" data-bs-theme="dark">
            <li class="nav-item no-border">
                <a href="#" class="nav-link" @click="signOut($event)">
                    <i class="fas fa-user"></i> <span>My page</span>
                </a>
            </li>
            <li class="nav-item f-right">
                <a href="#" class="nav-link" @click="signOut($event)">
                    <i class="fas fa-sign-out-alt"></i> <span>Sign out</span>
                </a>
            </li>
        </ul>
    </nav>
</template>
<style lang="scss">
    .topmenu {
        margin-top: 13px;
        margin-left: -12px;
        margin-right: -12px;
        padding-bottom: 0px;
    }
</style>