<script setup lang="ts">
    import ajax from '@/ajax';
import type { PropertyInfo } from '@/types';
    import { computed, ref, watch } from 'vue';

    const props = defineProps<{
        owner?: {
            ownerType: string | null, 
            id?: number | null
        }
    }>(), 
        properties = ref<any[]|null>(null),
        ownerData = computed(() => props.owner);

    watch(ownerData, async (dataFromOwner) => {     
        if(dataFromOwner?.id) {
            ajax.get<PropertyInfo[]>(`/api/properties/`, dataFromOwner).then(
            (data: PropertyInfo[]) => {
                properties.value = data;
            });
        }
    }, {immediate: true}); 
</script>
<template>
    <p>PROPERTIES COMES HERE {{ properties?.length }}</p>
    <p>{{ props.owner?.ownerType }} / {{ props.owner?.id }}</p>
</template>