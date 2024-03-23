<script setup lang="ts">
    import ajax from '@/ajax';
    import type { Client, Ticket } from '@/types';
    import moment from 'moment';
import { useRouter } from 'vue-router';


    const dateTime = (value: any) => {
        return moment(value).startOf('hour')
            .fromNow();
    }, props = defineProps<{
        ticket?: Ticket | null,
        client?: Client | null
    }>();

    const router = useRouter(),
        onDelete = () => {
        ajax.delete(`/api/tickets/${props.ticket?.id}`).then((result) => {
            router.push('/tickets');
        });
    }
</script>
<template>
    <div class="container-fluid ticket-header p-2">
        <div class="row">
            <div class="col ticket-header-title">
                #{{ props.ticket?.id }} - 
                <span class="clickable-field" data-name="subject">{{ props.ticket?.subject }}</span>
            </div>
            <div class="col-auto">
                <span class="badge bg-secondary">{{ props.ticket?.state_title }}</span>
            </div>
            <div class="col-auto">
                <div class="dropdown">
                    <button class="btn btn-sm btn-secondary dropdown-toggle" type="button" id="dropdownMenuButton1"
                        data-bs-toggle="dropdown" aria-expanded="false">
                        Actions
                    </button>
                    <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                        <li><a class="dropdown-item" href="#" v-on:click="onDelete">
                            <i class="fas fa-trash"></i> Remove
                        </a></li>
                    </ul>
                </div>
            </div>
        </div>
        <div class="row">
            <div class="col">
                {{ dateTime(props.ticket?.created) }}
                <span class="bull-divider">&bull;</span>
                {{ props.client?.name }}
            </div>
        </div>
    </div>
</template>
<style lang="scss">
.ticket-header {
    background-color: $default-ligher-background;

    &-title {
        font-weight: 600;
        font-size: 1.2em;
    }
}
</style>