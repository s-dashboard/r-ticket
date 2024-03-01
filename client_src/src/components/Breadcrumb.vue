<script lang="ts" setup>
    import { ref, watch } from 'vue';
    import { useRoute, useRouter } from 'vue-router';
    
    const router = useRouter(),
        activeRoute = useRoute(), 
        breadcrumbs = ref<any[]>([]),
        routes = router.getRoutes();

    const getNextIndex = (index: number, lengthOfPath: number) => {
        return index === undefined ? lengthOfPath : index;
    };
    
    const pathLength = (fullPath: string) => {
        const pathsIndex: number[] = [];
        for(let i = 0; i < fullPath.length; i++) {
            if(fullPath[i] === '/') {
                pathsIndex.push(i);
            }
        }
        return pathsIndex;
    }; 

    const buildBreadCrumbs = (fullPath: string) => {
        const pathsIndex = pathLength(fullPath);

        for (let i = 0; i < pathsIndex.length; i++) {
            const nextIndex = getNextIndex(pathsIndex[i + 1], fullPath.length),
                path = fullPath.substring(0, nextIndex),
                route = routes.find(p => p.path === path);
                
            breadcrumbs.value.push({
                path: path, 
                meta: route?.meta
            });
        }
    }; 


    watch(activeRoute, (newValue, oldValue) => {
        buildBreadCrumbs(newValue.fullPath);
    });

</script>
<template>
    {{ breadcrumbs  }}
    <nav aria-label="breadcrumb">
        <ol class="breadcrumb">
            <li class="breadcrumb-item" v-for="breadcrumb in breadcrumbs">
                <a v-bind:href="breadcrumb.path">
                    {{ breadcrumb.meta?.title }}
                </a>
            </li>
        </ol>
    </nav>
</template>