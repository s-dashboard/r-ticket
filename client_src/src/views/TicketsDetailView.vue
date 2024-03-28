<script setup lang="ts">
    import { computed, inject, onUpdated, ref, watch } from 'vue';
    import { useRoute } from 'vue-router';
    import TicketHeader from '@/components/TicketHeader.vue';
    import PropertyList from '@/components/PropertyList.vue';
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
    }

    const clickable: any = inject('clickablefield'), 
    validators = {
        'subject': (value: string) => {
            return value !== '' && value.length <= 50;
        },

        'content': (value: string) => {
            return value !== '';
        }
    };

    onUpdated(() => {
        clickable.init(form.value, ticket.value, onSave, validators);
    });
</script>
<template>
    <form name="tickets_form" v-on:submit.prevent="onSumbitForm" ref="form">
        <div class="container-fluid">
            <div class="row">
                <TicketHeader :ticket="ticket" :client="client" />
            </div>
            <div class="row mt-4">
                <div class="col flex clickable-field" data-name="content" data-type="textarea">
                    {{ ticket?.content }}
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