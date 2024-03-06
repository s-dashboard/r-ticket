<script setup lang="ts">
    import { computed, ref, watch } from 'vue';
    import { useRoute } from 'vue-router';
    import TicketHeader from '@/components/TicketHeader.vue';
    
    const route = useRoute(),
        tickets = ref<any>(null),
        query = computed(() => route.params.id);

    watch(query, async (to) => {
        await fetch(`/api/tickets/${to}`).then((success) => {
            success.json().then((data: any[]) => {
                tickets.value = data; 
            })
        });
    }, {immediate: true}); 
</script>
<template>
    <div class="container-fluid">
        <div class="row">
            <TicketHeader :ticket="tickets" />
        </div>
        <div class="row mt-4">
            <div class="col flex">
                {{ tickets?.content }}
            </div>
            <div class="col">PROPERTIES</div>
        </div>
    </div>
</template>