<template>
    <main>
        <h1 class="font-semibold text-shadow-sm text-center py-10 text-8xl text-emerald-600">Library</h1>

        <p class="text-center mb-5">You can go home <NuxtLink to="/" class="text-emerald-500 hover:text-emerald-700">here</NuxtLink>.</p>

        <div class="flex flex-wrap mx-auto w-3/4">
            <BookPreview v-for="book in BOOKS" :info="book" />
        </div>

        <p class="m-10 text-md text-center">Site made by <NuxtLink to="https://replit.com/@BD103" class="text-emerald-500 hover:text-emerald-700">BD103</NuxtLink></p>
    </main>
</template>

<script setup lang="ts">
import { BOOKS } from "../books";
</script>
