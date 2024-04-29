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
  <div class="container mx-auto px-8 py-12">
    <div
      class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 gap-8"
    >
      <!-- Iterate over applists and render Appcard component -->
      <Appcard v-for="app in applists" :applicationdata="app" :key="app.id" />
    </div>
  </div>

  <!-- </div> -->
  <!-- </div> -->
</template>
