<script setup lang="ts">
    import type { Client, Ticket } from '@/types';
    import moment from 'moment';

    const dateTime = (value: any) => {
        return moment(value).startOf('hour')
            .fromNow();
    }, props = defineProps<{
        ticket?: Ticket | null,
        client?: Client | null
    }>();
</script>
<template>
        <div class="container-fluid ticket-header p-2">
            <div class="row">
                <div class="col ticket-header-title">
                    #{{ props.ticket?.id }} - {{ props.ticket?.subject }}
                </div>
                <div class="col-auto">
                    <span class="badge bg-secondary">{{ props.ticket?.state_title }}</span>
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