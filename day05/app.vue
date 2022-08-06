<template>
    <main>
        <h1 class="p-5 mt-10 text-5xl font-bold text-center text-slate-900">Don't just <span class="text-blue-900 text-shadow">make</span> random, <span class="text-blue-900 text-shadow">find</span> random!</h1>

        <p class="text-center"><span class="text-slate-900">Find Something Random</span>, a website that randomly showcases epic repls from week 1 of Replit Creates.</p>

        <h2 class="p-5 mt-10 text-2xl text-center"><span class="text-slate-900">{{ data.name }}</span>, by <NuxtLink class="text-blue-600" v-bind:to="`https://replit.com/@${data.creator}`" target="_blank">{{ data.creator }}</NuxtLink></h2>

        <Repl v-bind:creator="data.creator" v-bind:name="data.name" />

        <p class="mb-5 text-center">Site by <NuxtLink class="text-blue-600" to="https://replit.com/@BD103" target="_blank">BD103</NuxtLink></p>
    </main>
</template>

<script setup>
import "./assets/css/main.css";

const { data } = await useFetch("/api/random_repl");
</script>

<style>
.text-shadow {
    text-shadow: 0 1px 2px rgb(0 0 0 / 0.1);
}
</style>
