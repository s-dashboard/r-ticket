
<script lang="ts" setup>
    const toggleBar = ($even: any) => { 
        const els = document.querySelectorAll('#sidebar');
        els[0].classList.toggle('active')
    };
</script>
<template>
    <nav class="navbar navbar-light bg-dark">
        <div class="container-fluid">
            <button type="button" id="sidebarCollapse" class="bars-btn btn btn-link" v-on:click="toggleBar($event)">
                <i class="fas fa-bars"></i>
                <span class="title">Navbar</span>
            </button>
            <form class="d-flex">
                <input class="form-control me-2" type="text" placeholder="Search">
                <button class="btn btn-light btn-sm" type="button">Search</button>
            </form>
        </div>
    </nav>
</template>
<style lang="scss">
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