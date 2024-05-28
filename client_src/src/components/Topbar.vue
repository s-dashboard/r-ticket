
<script lang="ts" setup>
    import { ref } from 'vue';
    import { useRoute, useRouter } from 'vue-router';
    import Topmenu from '@/components/Topmenu.vue'

    const route = useRoute(),
        router = useRouter(),
        search = ref(''),
        paramsToObject = (entries: any) => {
            const result: any = {}
            for(const [key, value] of entries) { // each 'entry' is a [key, value] tupple
                result[key] = value;
            }
            return result;
        },
        onSubmit = (e: any) => {

            const url: any = new URL(<any>window.location), 
                hasSearch = url.searchParams.has('search'); 
            
            if(search.value === '' && hasSearch) {
                url.searchParams.delete('search');
            } else {
                url.searchParams.set('search', search.value);
            }
            
            const params = paramsToObject(url.searchParams);
            router.push({ 
                path: route.fullPath, 
                query: params
            });

            e.preventDefault();
            return false;
        };
</script>
<template>
    <nav class="topbar navbar navbar-light bg-dark">
        <div class="container-fluid">
            <div>
                <span>r-Ticket</span>
                <div class="version">v1.0</div>
            </div>
            <form class="d-flex search-form" v-on:submit="onSubmit">
                <input class="form-control me-2 flex-1" type="text" placeholder="Search"
                    v-model="search">
                <i class="fas fa-search"></i>
            </form>
        </div>
    </nav>
    <Topmenu />
</template>
<style lang="scss">

    .version {
        font-size: 0.70rem;
    }

    .search-form {
        .form-control {
            padding-right: 35px;
        }

        i {
            position: absolute;
            right: 28px;
            top: 15px;
            font-size: 24px;
        }
    }

    .topbar {
        margin: -12px;
        padding-bottom: 12px;
    }

    .title {
        padding-left: 8px;
        color: #c5c2c2;
    }

    .bars-btn  {
        color: #fff;
        text-decoration: none;
        
        &:hover {
            color: #c5c2c2;
            transition: 0.2s;
        }

        &:active {
            color: #c5c2c2!important;
        }
    }
</style>