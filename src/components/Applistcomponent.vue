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
    ...mapState(useFetchStore, ["applists"]),
  },
  async created() {
    await this.fetchApplists();
  },
};
</script>

<template>
  <div class="container mx-auto px-8 py-12 min-h-screen">
    <section
      class="flex justify-center items-center"
      v-if="this.applists.length == 0"
    >
      <div class="py-8 px-4 mx-auto max-w-screen-xl lg:py-16 lg:px-6">
        <div class="mx-auto max-w-screen-sm text-center">
          <h1
            class="mb-4 text-7xl tracking-tight font-bold lg:text-9xl text-white"
          >
            Hello!
          </h1>
          <p class="mb-4 text-3xl tracking-tight font-bold text-gray-100">
            Welcome to Applaunch!
          </p>
          <p class="mb-4 text-lg font-medium text-gray-200">
            Begin by adding application.
          </p>
        </div>
      </div>
    </section>
    <div
      class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 gap-8"
    >
      <!-- Iterate over applists and render Appcard component -->
      <Appcard
        v-for="app in applists"
        :applicationdata="app"
        :key="app.id"
        v-if="this.applists.length > 0"
      />
    </div>
  </div>

  <!-- </div> -->
  <!-- </div> -->
</template>
