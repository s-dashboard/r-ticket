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
            ajax.get<PropertyInfo[]>(`/api/properties/${dataFromOwner.ownerType}/${dataFromOwner.id}`).then(
            (data: PropertyInfo[]) => {
                properties.value = data.map((p: any) => { 
                    return {
                        id: p.id, 
                        data_type: p.data_type, 
                        label: p.label, 
                        value:  p.value.value
                    }; 
                });
            });
        }
    }, {immediate: true}); 
</script>
<template>
    <p>PROPERTIES COMES HERE</p>
    <div v-for="prop in properties" class="form-group">
        <label :for="'property-field_' + prop.id">{{ prop.label }}</label>
        <template v-if="prop.data_type === 'int'">
            <input :name="'property_' + prop.id" type="number" :id="'property-field_' + prop.id" class="form-control" :value="prop.value" />
        </template>
    </div>
</template>