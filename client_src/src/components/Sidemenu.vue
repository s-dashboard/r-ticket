<script lang="ts" setup>
    import { useRoute, useRouter } from 'vue-router';
    const router = useRouter(), 
        activeRoute = useRoute(),
        isMatch = (path: string) => {
            return false;
        }, 
        routes = router.options.routes.filter(p => p.meta?.sideMenu);

        const signOut = (e: any) => {
            
             // await ajax.post<LoginResponse>('/api/login', {
            //     username: email.value, 
            //     password: password.value
            // }).then((data: LoginResponse) => {
            //     if(data.success) {
            //         localStorage.setItem('TICKETS_TOKEN', data.token); 
            // router.push({
            //     path: '/'
            // });
            
            // location.reload();
            //     } else {
            //         error.value = 'Invalid email or password';    
            //     }
            // })

            localStorage.removeItem('TICKETS_TOKEN');
            router.push({
                path: '/'
            });
            location.reload();
            return false;
        }; 
</script> 
<template>
    <nav id="sidebar">
        <div class="sidebar-header"></div>
        <ul class="list-unstyled components">
            <li class="list-nav-item" :class="{ 'active': isMatch(route.path)}" v-for="route in routes">
                <a class="nav-link" v-bind:href="route.path" data-toggle="collapse" aria-expanded="false" :title="route.meta?.title">
                    <i :class="'fas ' + route.meta?.iconCls"></i>
                </a>
            </li>
            <li class="list-nav-item">
                <a href="#" class="nav-link" @click="signOut($event)">
                    <i class="fas fa-sign-out-alt"></i>
                </a>
            </li>
        </ul>
    </nav>
</template>
<style lang="scss">
    $sidebar-width: 54px;

    .list-nav-item {
        text-align: center;
        font-size: 28px;
        padding: 8px;
        width: 53px;
        height: 53px;
        border-bottom: 1px solid #47748b;

        &:hover, &.active {
            color: #7386D5;
            background: #fff;
        }
    }

    #sidebar {
        min-width: $sidebar-width;
        max-width: $sidebar-width;
        background: #7386D5;
        color: #fff;
        transition: all 0.3s;
        padding: 0;
    }

    #sidebar.active {
        margin-left: -$sidebar-width;
    }

    #sidebar .sidebar-header {
        padding: 20px;
        background: #6d7fcc;
    }

    #sidebar ul.components {
        padding: 0;
    }

    #sidebar ul p {
        color: #fff;
        padding: 10px;
    }
    
    @media (max-width: 768px) {
        #sidebar {
            margin-left: -$sidebar-width;
        }
        #sidebar.active {
            margin-left: 0;
        }
        #sidebarCollapse span {
            display: none;
        }
    }
</style>