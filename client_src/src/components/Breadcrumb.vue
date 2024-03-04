<script lang="ts" setup>
    import { ref, watch } from 'vue';
    import { useRoute } from 'vue-router';

    const route = useRoute(),
        breadcrumbs = ref<any[]>([]);

    watch(route, (to) => {
        const breads: any[] = [], 
        items: any[] = route.fullPath.split('/');
        items.shift();

        breadcrumbs.value= items.reduce((breadcrumbArray, path, idx) => {
            breadcrumbArray.push({
                path: path,
                to: breadcrumbArray[idx - 1]
                    ? "/" + breadcrumbArray[idx - 1].path + "/" + path
                    : "/" + path,
                text: to.matched[idx].meta.title || path,
            });
            return breadcrumbArray;
        }, [])
    });

</script>
<template>
    <nav aria-label="breadcrumb">
        <ol class="breadcrumb">
            <li class="breadcrumb-item" v-for="(breadcrumb, index) in breadcrumbs">
                <template v-if="index < breadcrumbs.length - 1">
                    <a v-bind:href="breadcrumb.to">
                        {{ breadcrumb.text }}
                    </a>
                </template>
                <template v-else>
                    {{ breadcrumb.text }}
                </template>
            </li>
        </ol>
    </nav>
</template>
<style lang="scss">
    .breadcrumb {
        font-size: 0.8em;
        margin: 8px 0px;
    }
</style>