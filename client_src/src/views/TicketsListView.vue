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
    <div class="col-2">
        <table class="table table-striped">
            <thead>
                <th>#</th>
                <th>Subject</th>
                <th>Received</th>
            </thead>
            <tbody>
                <tr v-for="ticket in tickets">
                    <td>{{ ticket.id }}</td>
                    <td>
                        <RouterLink :to="{ path: '/tickets/' + ticket.id}" replace>
                        {{ ticket.subject }}
                        </RouterLink>
                    </td>
                    <td>{{ dateTime(ticket.created) }}</td>
                </tr>
            </tbody>
        </table>
    </div>
</template>