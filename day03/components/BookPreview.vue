<template>
    <div class="flex-auto p-5 w-1/3">
        <NuxtLink :to="bookPath">
            <img :src="imagePath" />
        </NuxtLink>
    </div>
</template>

<script setup lang="ts">
// Prevents website for changing???
import { IBook } from "../books";

const props = defineProps<{info?: IBook}>();

const imagePath = computed(() => {
    return "/img/" + props.info.id + ".webp";
});

const bookPath = computed(() => {
    return "/book/" + props.info.id;
});
</script>
