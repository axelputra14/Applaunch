<script>
import { HomeIcon, PlusIcon } from "@heroicons/vue/24/solid";
import { RouterLink } from "vue-router";
export default {
  name: "BottomNav",
  components: {
    RouterLink,
    HomeIcon,
    PlusIcon,
  },
};
</script>
<template>
  <div
    class="fixed z-50 w-full h-12 max-w-64 -translate-x-1/2 border-2 border-opacity-50 rounded-full bottom-4 left-1/2 bg-gray-700/25 border-gray-600 shadow-md shadow-purple-400/75"
  >
    <div class="grid h-full max-w-lg grid-cols-2 mx-auto">
      <RouterLink
        to="/"
        class="inline-flex cursor-pointer flex-col items-center justify-center px-5 rounded-s-full hover:bg-gray-700/50 group hover:shadow-2xl hover:shadow-sky-400/75"
      >
        <HomeIcon
          class="w-6 h-6 mb-1 text-neutral-500 group-hover:text-sky-400"
        />
      </RouterLink>

      <RouterLink
        to="/app-add"
        class="inline-flex cursor-pointer flex-col items-center justify-center px-5 rounded-e-full hover:bg-gray-700/50 group hover:shadow-2xl hover:shadow-lime-400/75"
      >
        <PlusIcon
          class="w-6 h-6 mb-1 text-neutral-500 group-hover:text-green-400"
        />
      </RouterLink>
    </div>
  </div>

  <!-- <div class="w-full">
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
  </div> -->
</template>
