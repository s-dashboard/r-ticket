<script setup lang="ts">
    import { computed, ref, watch } from 'vue';
    import { useRoute } from 'vue-router';
    import TicketHeader from '@/components/TicketHeader.vue';
    import PropertyList from '@/components/PropertyList.vue';
    import type { Client, Ticket } from '@/types';
    import ajax from '@/ajax';
    
    const route = useRoute(),
        ticket = ref<Ticket|null>(null),
        client = ref<Client|null>(null),
        query = computed(() => route.params.id);

    watch(query, async (ticket_id) => {
        ajax.get<Ticket>(`/api/tickets/${ticket_id}`).then(
            (data: Ticket) => {

            ajax.get<Client>(`/api/clients/${data?.client_id}`).then(
                (clientData: Client) => {
                    ticket.value = data;
                    client.value = clientData;
                });
        });
    }, {immediate: true}); 
</script>
<template>
    <div class="container-fluid">
        <div class="row">
            <TicketHeader :ticket="ticket" :client="client" />
        </div>
        <div class="row mt-4">
            <div class="col flex">
                {{ ticket?.content }}
            </div>
            <div class="col" style="max-width: 250px;">
                <PropertyList :owner="{
                    ownerType: 'tickets',
                    id: ticket?.id
                }" />
            </div>
        </div>
    </div>
</template>