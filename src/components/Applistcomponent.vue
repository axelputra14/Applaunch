<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";

import { MagnifyingGlassIcon } from "@heroicons/vue/24/solid";
import Appcard from "./Appcard.vue";
import AlertComponent from "./AlertComponent.vue";

export default {
  name: "Applistcomponent",
  data() {
    return {
      searchTitleQuery: "",
      searchDeveloperQuery: "",
    };
  },
  components: {
    Appcard,
    MagnifyingGlassIcon,
    AlertComponent,
  },
  methods: {
    ...mapActions(useFetchStore, ["fetchApplists", "closeAlert"]),
    closeAlertFunction() {
      this.closeAlert();
    },
  },
  computed: {
    ...mapState(useFetchStore, ["applists", "showAlert", "alertMessage"]),
    filterNameFn() {
      return this.applists.filter((app) =>
        app.title.toLowerCase().includes(this.searchTitleQuery.toLowerCase())
      );
    },
    filterDeveloperFn() {
      return this.applists.filter((app) => {
        app.developer === this.searchDeveloperQuery;
      });
    },
  },
  async created() {
    await this.fetchApplists();
  },
};
</script>

<template>
  <div class="flex justify-center">
    <Transition>
      <AlertComponent
        v-if="this.showAlert"
        v-bind:alertContent="alertMessage"
        @closeAlert="closeAlertFunction"
      />
    </Transition>
  </div>
  <div class="container mx-auto px-8 py-12 min-h-screen">
    <div class="grid grid-cols-2 grid-flow-row">
      <div
        class="relative w-1/2 translate-x-1/2"
        v-if="this.applists.length > 0"
      >
        <div
          class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none bottom-1"
        >
          <MagnifyingGlassIcon
            class="w-4 h-4 text-gray-500 dark:text-gray-400"
          ></MagnifyingGlassIcon>
        </div>
        <input
          type="text"
          v-model="searchTitleQuery"
          class="border text-sm rounded-lg block w-full ps-10 p-2 bg-gray-700/75 border-cyan-500 placeholder-sky-300 text-sky-300 focus:border-yellow-400"
          placeholder="Search app name..."
        />
      </div>
      <div
        class="relative w-1/2 translate-x-1/2"
        v-if="this.applists.length > 0"
      >
        <div
          class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none bottom-1"
        >
          <MagnifyingGlassIcon
            class="w-4 h-4 text-gray-500 dark:text-gray-400"
          ></MagnifyingGlassIcon>
        </div>
        <!-- di sini by brand name-->

        <div class="max-w-sm mx-auto">
          <select
            id="countries"
            class="border text-sm rounded-lg block w-full ps-10 p-2 bg-gray-700/75 border-yellow-500 placeholder-yellow-300 text-yellow-300 focus:border-pink-400"
          >
            <option disabled hidden selected>Search Developer</option>
            <option
              class="bg-slate-800"
              v-for="app in applists"
              :value="app.developer"
            >
              {{ app.developer }}
            </option>
          </select>
        </div>

        <!-- <input
          type="text"
          v-model="searchTitleQuery"
          class="border text-sm rounded-lg block w-full ps-10 p-2.5 bg-gray-700/75 border-cyan-500 placeholder-sky-300 text-sky-300"
          placeholder="By brand name work in progress..."
        /> -->
      </div>
    </div>

    <p
      class="text-center mt-1 text-sky-100"
      v-if="this.applists.length > 0 && filterNameFn.length < 1"
    >
      No results found. Try with different keyword.
    </p>

    <div
      class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 gap-8"
      v-if="this.applists.length > 0"
    >
      <!-- Iterate over applists and render Appcard component -->
      <Appcard
        v-for="(app, index) in filterNameFn"
        :applicationdata="app"
        :key="app.id"
      />
    </div>
    <div v-else class="flex justify-center items-center">
      <div class="py-8 px-4 mx-auto max-w-screen-xl lg:py-16 lg:px-6">
        <div class="mx-auto max-w-screen-sm text-center">
          <h1
            class="mb-4 text-6xl tracking-tight font-bold lg:text-9xl text-sky-400"
          >
            Hello!
          </h1>
          <p class="mb-4 text-3xl tracking-tight font-medium text-cyan-200">
            Welcome to Applaunch!
          </p>
          <p class="mb-4 text-lg font-normal text-cyan-100">
            Begin by adding application.
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped>
.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
