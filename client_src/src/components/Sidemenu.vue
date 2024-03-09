<script lang="ts" setup>
    import { useRoute, useRouter } from 'vue-router';
    const router = useRouter(), 
        activeRoute = useRoute();
    
    const routes = router.options.routes.filter(p => p.meta?.sideMenu);
</script> 
<template>
    <nav id="sidebar">
        <div class="sidebar-header"></div>
        <ul class="list-unstyled components">
            <li class="list-nav-item" v-for="route in routes">
                <a class="nav-link" v-bind:href="route.path" data-toggle="collapse" aria-expanded="false" :title="route.meta?.title">
                    <i :class="'fas ' + route.meta?.iconCls"></i>
                </a>
            </li>
        </ul>
    </nav>
</template>
<style lang="scss">

    .list-nav-item {
        text-align: center;
        padding: 5px;
    }

    .nav {
        &-level-1 {
            margin-left: 15px;
            font-size: 0.8em;
        }
    }

    #sidebar {
        min-width: 80px;
        max-width: 80px;
        background: #7386D5;
        color: #fff;
        transition: all 0.3s;
    }

    #sidebar.active {
        margin-left: -80px;
    }

    #sidebar .sidebar-header {
        padding: 20px;
        background: #6d7fcc;
    }

    #sidebar ul.components {
        padding: 20px 0;
        border-bottom: 1px solid #47748b;
    }

    #sidebar ul p {
        color: #fff;
        padding: 10px;
    }

    #sidebar ul li a {
    }

    #sidebar ul li a:hover {
        // color: #7386D5;
        // background: #fff;
    }

    // #sidebar ul li.active>a,
    // a[aria-expanded="true"] {
    //     color: #fff;
    //     background: #6d7fcc;
    // }

    // ul ul a {
    //     font-size: 0.9em !important;
    //     padding-left: 30px !important;
    //     background: #6d7fcc;
    // }

    
    @media (max-width: 768px) {
        #sidebar {
            margin-left: -250px;
        }
        #sidebar.active {
            margin-left: 0;
        }
        #sidebarCollapse span {
            display: none;
        }
    }
</style>