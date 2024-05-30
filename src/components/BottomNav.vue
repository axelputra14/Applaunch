<script>
import { RouterLink } from "vue-router";
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
export default {
  name: "BottomNav",
  components: {
    RouterLink,
  },
};
</script>
<template>
  <div class="w-full">
    <section
      id="bottom-navigation"
      class="block fixed inset-x-0 bottom-0 z-10 border-t-2 border-sky-500"
    >
      <nav class="bg-gray-800 bg-opacity-50">
        <div
          class="grid grid-cols-2 items-center text-center py-2 mx-auto text-gray-600 dark:text-gray-300"
        >
          <RouterLink
            to="/"
            class="text-gray-200 border-b-2 border-transparent hover:border-emerald-500 mx-4 text-xl"
            >Home</RouterLink
          >

          <RouterLink
            to="/app-add"
            class="text-gray-200 border-b-2 border-transparent hover:border-emerald-500 mx-4 text-xl"
            >Add</RouterLink
          >
        </div>
      </nav>
    </section>
  </div>
</template>
