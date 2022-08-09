<template>
    <main>
        <h1 class="font-semibold text-shadow-sm text-center pt-10 text-8xl text-emerald-600">Find a Story</h1>

        <p class="text-center mb-10 p-5">Would you rather view all story recommendations? Go to <NuxtLink to="/library" class="text-emerald-500 hover:text-emerald-700">library</NuxtLink> instead.</p>

        <Question q="Do you like fiction or non-fiction?">
            <Answer @click="ANSWERED_FICTION = true; ANSWERS.fiction = true;" :selected="ANSWERS.fiction === true">Fiction</Answer>
            <Answer @click="ANSWERED_FICTION = true; ANSWERS.fiction = false;" :selected="ANSWERS.fiction === false">Non-Fiction</Answer>
        </Question>

        <Question q="What is your favorite genre?" v-if="ANSWERED_FICTION && ANSWERS.fiction === true">
                <Answer @click="ANSWERED_GENRE = true; ANSWERS.genre = BookGenre.Fantasy;" :selected="ANSWERS.genre === BookGenre.Fantasy">Fantasy</Answer>
                <Answer @click="ANSWERED_GENRE = true; ANSWERS.genre = BookGenre.SciFi;" :selected="ANSWERS.genre === BookGenre.SciFi">Sci-Fi</Answer>
                <Answer @click="ANSWERED_GENRE = true; ANSWERS.genre = BookGenre.Dystopia;" :selected="ANSWERS.genre === BookGenre.Dystopia">Dystopia</Answer>
                <Answer @click="ANSWERED_GENRE = true; ANSWERS.genre = BookGenre.Action;" :selected="ANSWERS.genre === BookGenre.Action">Action</Answer>
                <Answer @click="ANSWERED_GENRE = true; ANSWERS.genre = BookGenre.Mystery;" :selected="ANSWERS.genre === BookGenre.Mystery">Mystery</Answer>
        </Question>

        <Question q="What is your favorite genre?" v-if="ANSWERED_FICTION && ANSWERS.fiction === false">
            <Answer @click="ANSWERED_GENRE = true; ANSWERS.genre = BookGenre.Math;"  :selected="ANSWERS.genre === BookGenre.Math">Math</Answer>
        </Question>

        <Question q="Do you like series or standalone novels?" v-if="ANSWERED_GENRE">
            <Answer @click="ANSWERED_SERIES = true; ANSWERS.series = true;" :selected="ANSWERS.series === true">Series</Answer>
            <Answer @click="ANSWERED_SERIES = true; ANSWERS.series = false;" :selected="ANSWERS.series === false">Standalone</Answer>
        </Question>

        <Question q="What length of book do you prefer?" v-if="ANSWERED_SERIES">
            <Answer @click="ANSWERED_LENGTH = true; ANSWERS.bookLength = BookLength.Small;" :selected="ANSWERS.bookLength === BookLength.Small">Small</Answer>
            <Answer @click="ANSWERED_LENGTH = true; ANSWERS.bookLength = BookLength.Medium;" :selected="ANSWERS.bookLength === BookLength.Medium">Medium</Answer>
            <Answer @click="ANSWERED_LENGTH = true; ANSWERS.bookLength = BookLength.Large;" :selected="ANSWERS.bookLength === BookLength.Large">Long</Answer>
        </Question>

        <div v-if="ANSWERED_LENGTH">
            <p class="mt-10 text-center text-3xl text-gray-900">You should try reading...</p>

            <div class="flex flex-wrap mx-auto w-3/4">
                <BookPreview v-for="book in suggestedBooks.books" :info="book" />
            </div>

            <p class="m-5 text-sm text-center">These suggestions match <span class="text-emerald-500">{{ suggestedBooks.hits }}</span> out of 4 questions.</p>
        </div>

        <p class="m-10 text-md text-center">Site made by <NuxtLink to="https://replit.com/@BD103" class="text-emerald-500 hover:text-emerald-700">BD103</NuxtLink></p>
    </main>
</template>

<script setup lang="ts">
import { BookGenre, BookLength, IAnswer, suggestBooks } from "../books";

const ANSWERED_FICTION = ref(false);
const ANSWERED_GENRE = ref(false);
const ANSWERED_SERIES = ref(false);
const ANSWERED_LENGTH = ref(false);

const ANSWERS = ref<IAnswer>({
    fiction: null,
    genre: null,
    series: null,
    bookLength: null,
});

const suggestedBooks = computed(() => {
    return suggestBooks(ANSWERS.value);
});
</script>
