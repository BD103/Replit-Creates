<template>
    <div class="p-10 mx-10 my-10 drop-shadow-xl rounded-xl bg-slate-900">
        <iframe v-bind:src="replUrl" class="w-4/5 h-[700px] mx-auto rounded"></iframe>
    </div>
</template>

<script setup>
const props = defineProps({
    creator: String,
    name: String,
});

const replUrl = computed(() => {
    return `https://replit.com/@${props.creator}/${props.name}?embed=true`;
});
</script>
