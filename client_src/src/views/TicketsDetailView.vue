<script setup lang="ts">
    import { computed, ref, watch } from 'vue';
    import { useRoute } from 'vue-router';
    import TicketHeader from '@/components/TicketHeader.vue';
    import PropertyList from '@/components/PropertyList.vue';
    import Editablefield from '@/components/Editablefield.vue';
    import type { Client, Ticket } from '@/types';
    import ajax from '@/ajax';

    const route = useRoute(),
        ticket = ref<Ticket|null>(null),
        client = ref<Client|null>(null),
        form = ref(),
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

    const onSave = (source: any) => {
        const formData = new FormData(source), 
            formObj = Object.fromEntries(formData);
            console.log(formObj);

        // ajax.put<Ticket>(`/api/tickets/${ticket_id}`, formObj).then(
        // );
    }, onSumbitForm = (e: any | SubmitEvent) => {
        onSave(e.target);
    }, onValueChanged = (e: any) => {
        if(ticket.value) {
            const data: any = ticket.value;
            data[e[0]] = e[1];
        }
    }

</script>
<template>
    <form name="tickets_form" v-on:submit.prevent="onSumbitForm" ref="form">
        <div class="container-fluid">
            <div class="row">
                <TicketHeader :ticket="ticket" :client="client" />
            </div>
            <div class="row mt-4">
                <div class="col flex">
                    <Editablefield :value="ticket?.content" name="content" :multiline="true" @value:changed="onValueChanged($event)" />
                </div>
                <div class="col" style="max-width: 250px;">
                    <PropertyList :owner="{
                        ownerType: 'tickets',
                        id: ticket?.id
                    }" @save="onSave" />
                </div>
            </div>
        </div>
    </form>
</template>