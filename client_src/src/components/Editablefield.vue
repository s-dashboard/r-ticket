<script setup lang="ts">
    import { ref } from 'vue';

    const props = defineProps(["label", "name", "value","multiline"]),
        editing = ref(false),
        emit = defineEmits(['value:changed']),
        onInput = (e: any) =>  {
            emit('value:changed', [props.name, e.target.value]);
        };
</script>
<template>     
    <div>
        <span @click="editing=true" v-show="!editing" v-nl2br>
            {{ props.value }}
        </span>
        <span v-show="editing">
            <template v-if="props.multiline">
                <textarea
                    @input="onInput"
                    @blur="editing=false" 
                    class="form-control"
                    v-bind:name="props.name"
                >{{ props.value }}</textarea>
            </template>
            <template v-if="!props.multiline">
                <input :value="props.value"
                    @input="onInput"
                    @keydown.enter="editing=false"
                    @blur="editing=false"
                    type="text" 
                    class="form-control"
                    v-bind:name="props.name"
                >
            </template>
        </span>
    </div>
</template>