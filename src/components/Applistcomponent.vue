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
      </div>
    </div>

    <!-- </div> -->
  </div>
</template>
