<template>
    <main v-if="book !== null">
        <div class="flex">
            <img :src="'/img/' + book.id + '.webp'" class="p-20 basis-1/2" />
        
            <div class="p-20 basis-1/2">
                <h1 class="font-semibold text-shadow-sm text-center pt-10 text-4xl text-emerald-600">{{ book.name }}</h1>

                <p class="text-center text-xl py-5">Written by {{ book.author }}</p>

               <p class="p-10">A <span class="text-emerald-700">{{ book.fiction ? "fiction" : "non-fiction" }}</span> book with a/an <span class="text-emerald-700">{{ book.genre }}</span> genre. It is a <span class="text-emerald-700">{{ book.bookLength }}</span> book that <span class="text-emerald-700">{{ book.series ? "is" : "is not" }}</span> part of a series.</p>

                <h2 class="font-semibold px-10 text-2xl">BD103's Review</h2>

                <blockquote class="px-10 pt-5 italic">
                    "{{ book.review }}"
                </blockquote>

                <p class="text-center pt-10">For more information, please go to <NuxtLink :to="book.link.toString()" class="text-emerald-500 hover:text-emerald-700">Barnes & Noble</NuxtLink>.</p>

               <p class="text-center pt-5">Alternatives, go back to <NuxtLink to="/library" class="text-emerald-500 hover:text-emerald-700">the Library</NuxtLink>.</p>
            </div>
        </div>

        <p class="m-10 text-md text-center">Site made by <NuxtLink to="https://replit.com/@BD103" class="text-emerald-500 hover:text-emerald-700">BD103</NuxtLink></p>
    </main>

    <main v-else>
        <h1 class="text-center p-10 text-3xl">Sorry, we don't know which book you're talking about :(</h1>

        <p class="text-center text-lg"><NuxtLink to="/" class="text-emerald-500 hover:text-emerald-700">Go home?</NuxtLink></p>

        <p class="m-10 text-md text-center">Site made by <NuxtLink to="https://replit.com/@BD103" class="text-emerald-500 hover:text-emerald-700">BD103</NuxtLink></p>
    </main>
</template>

<script setup lang="ts">
import { BOOKS, IBook } from '../../books';

const route = useRoute();

const book = computed<IBook | null>(() => {
    for (const bookInfo of BOOKS) {
        if (bookInfo.id === route.params.id) {
            return bookInfo;
        }
    }

    return null;
});
</script>
