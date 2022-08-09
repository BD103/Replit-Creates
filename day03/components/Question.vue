<template>
    <div>
        <p class="text-lg text-center text-gray-900">{{ q }}</p>

        <div class="flex mx-auto w-1/2">
            <slot />
        </div>
    </div>
</template>

<script setup lang="ts">
defineProps<{q?: string}>();
</script>
