<script setup lang="ts">
    import { computed, ref, watch } from 'vue';
    import { useRoute } from 'vue-router';
    import moment from 'moment';

    const route = useRoute(),
        tickets = ref<any[]>([]),
        query = computed(() => route.query.state), 
        dateTime = (value: any) => {
            return moment(value).format("YYYY-MM-DD");
        };

    watch(query, async (to) => {
        await fetch('/api/tickets/?state=' + (to || 'new')).then((success) => {
            success.json().then((data: any[]) => {
                tickets.value = data; 
            })
        });
    }, {immediate: true}); 

</script>
<template>
    <div class="col-2 w-100">
        <div class="row row-gridtable row-gridtable-header">
            <div class="col" style="max-width: 32px;"></div>
            <div class="col">Subject</div>
            <div class="col" style="max-width: 130px;">Created</div>
        </div>
        <div class="row row-gridtable" v-for="ticket in tickets">
            <div class="col" style="max-width: 32px;">
                #{{ ticket.id }}
            </div>
            <div class="col">
                <RouterLink :to="{ path: '/tickets/' + ticket.id}" replace>
                    {{ ticket.subject }}
                </RouterLink>
            </div>
            <div class="col" style="max-width: 130px;">
                {{ dateTime(ticket.created) }}
            </div>
        </div>
    </div>
</template>