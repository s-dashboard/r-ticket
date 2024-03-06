<script setup lang="ts">
    import type { Ticket } from '@/types';
    // import type { PropType } from 'vue';
    import moment from 'moment';

    const dateTime = (value: any) => {
        return moment(value).format("YYYY-MM-DD, HH:mm");
    }, props = defineProps<{
        ticket?: Ticket
    }>();
</script>
<template>
        <div class="container-fluid" style="background-color: #e3f2fd;">
            <div class="row p-2">
                <div class="col flex">
                    #{{ props.ticket?.id }} - {{ props.ticket?.subject }}
                </div>
                <div>
                    <span class="badge bg-secondary">{{ props.ticket?.state_title }}</span>
                </div>
            </div>
        </div>
</template>

<!-- <span class="badge bg-secondary">{{ dateTime(props.ticket?.created) }}</span>
<span class="badge bg-secondary">{{ dateTime(props.ticket?.changed) }}</span> -->