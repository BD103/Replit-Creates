<template>
    <button class="rounded flex-auto m-2 p-2 text-emerald-50" :class="[selected ? ['bg-emerald-500', 'hover:bg-emerald-600'] : ['bg-emerald-700', 'hover:bg-emerald-800']]">
        <slot />
    </button>
</template>

<script setup lang="ts">
const props = defineProps<{selected?: boolean}>();
</script>
