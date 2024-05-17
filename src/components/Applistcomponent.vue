<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";

import { MagnifyingGlassIcon } from "@heroicons/vue/24/solid";
import Appcard from "./Appcard.vue";
export default {
  name: "Applistcomponent",
  data() {
    return {
      searchTitleQuery: "",
    };
  },
  components: {
    Appcard,
    MagnifyingGlassIcon,
  },
  methods: {
    ...mapActions(useFetchStore, ["fetchApplists"]),
  },
  computed: {
    ...mapState(useFetchStore, ["applists"]),
    filterFn() {
      return this.applists.filter((app) =>
        app.title.toLowerCase().includes(this.searchTitleQuery.toLowerCase())
      );
    },
  },
  async created() {
    await this.fetchApplists();
  },
};
</script>

<template>
  <div class="container mx-auto px-8 py-12 min-h-screen">
    <div class="relative w-full" v-if="this.applists.length > 0">
      <!-- <div class="relative w-full"> -->
      <div
        class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none"
      >
        <MagnifyingGlassIcon
          class="w-4 h-4 text-gray-500 dark:text-gray-400"
        ></MagnifyingGlassIcon>
      </div>
      <input
        type="text"
        v-model="searchTitleQuery"
        class="border text-sm rounded-lg block w-full ps-10 p-2.5 bg-gray-900 border-cyan-500 placeholder-yellow-400 text-yellow-400"
        placeholder="Search app name..."
      />
    </div>
    <p
      class="text-center mt-1 text-sky-100"
      v-if="this.applists.length > 0 && filterFn.length < 1"
    >
      No results found. Try with different keyword.
    </p>
    <section
      class="flex justify-center items-center"
      v-if="this.applists.length == 0"
    >
      <div class="py-8 px-4 mx-auto max-w-screen-xl lg:py-16 lg:px-6">
        <div class="mx-auto max-w-screen-sm text-center">
          <h1
            class="mb-4 text-6xl tracking-tight font-bold lg:text-9xl text-sky-400"
          >
            Hello!
          </h1>
          <p class="mb-4 text-3xl tracking-tight font-medium text-sky-200">
            Welcome to Applaunch!
          </p>
          <p class="mb-4 text-lg font-normal text-cyan-100">
            Begin by adding application.
          </p>
        </div>
      </div>
    </section>
    <!-- v-if applists.length > 0 dan filteredItems.length (?) < 1 -->

    <div
      class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 gap-8"
      v-if="this.applists.length > 0"
    >
      <!-- Iterate over applists and render Appcard component -->
      <Appcard
        v-for="app in filterFn"
        :applicationdata="app"
        :key="app.id"
        v-if="this.applists.length > 0"
      />
    </div>
  </div>

  <!-- </div> -->
  <!-- </div> -->
</template>
