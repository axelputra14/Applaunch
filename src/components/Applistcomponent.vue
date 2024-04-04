<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import Appcard from "./Appcard.vue";
export default {
  name: "Applistcomponent",
  components: {
    Appcard,
  },
  methods: {
    ...mapActions(useFetchStore, ["fetchApplists"]),
  },
  computed: {
    ...mapState(useFetchStore, ["tesvar", "applists"]),
  },
  created() {
    this.fetchApplists();
  },
};
</script>

<template>
  <div class="applist-body">
    <h1 class="text-orange-600">Ini app list component</h1>
    <!-- <div class="list-grid-wrapper"> -->

    <div
      class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 gap-8"
    >
      <!-- Image 1 -->
      <div class="relative group">
        <Appcard v-for="app in applists" :applicationdata="app" :key="app.id" />
        <!-- <img
          class="w-full h-auto rounded-lg shadow-lg transition duration-300 transform group-hover:scale-105"
          src="https://via.placeholder.com/200x300"
          alt="Image 1"
        /> -->
        <!-- <div
          class="absolute bottom-0 left-0 right-0 bg-black bg-opacity-50 opacity-0 flex justify-center items-center transition duration-300 group-hover:opacity-100"
        > -->
        <!-- <a href="#" class="text-white text-lg font-bold">View</a> -->
        <!-- </div> -->
      </div>
    </div>

    <!-- </div> -->
  </div>
</template>
