<script>
import { RouterLink } from "vue-router";
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
export default {
  name: "BottomNav",
  components: {
    RouterLink,
  },
  //   methods: {
  //     ...mapActions(useFetchStore, ["logout", "setIsLogin"]),
  //     logoutHandler() {
  //       this.logout();
  //     },
  //   },
  //   computed: {
  //     ...mapState(useFetchStore, ["isLogin"]),
  //   },
  //   created() {
  //     if (localStorage.getItem("access_token")) {
  //       this.setIsLogin();
  //     }
  //   },
};
</script>
<template>
  <!-- <div class="bottom-bar-overall">
    <ul class="bottom-bar-container">
      <li class="navbar-item">

        
          <span class="navbar-icon"><i class="bi bi-view-stacked"></i></span>
          <span class="navbar-label">Apps</span>
          <div id="bottom-navbar-indicator" class="nav-indicator"></div>
        </RouterLink>

      </li>
      <li class="navbar-item">

        <RouterLink to="/app-add">
          <span class="navbar-icon"><i class="bi bi-plus-square"></i></span>
          <span class="navbar-label">Add</span>
          <div id="bottom-navbar-indicator" class="nav-indicator"></div>
        </RouterLink>

      </li>

    </ul>
  </div> -->
  <!-- component -->
  <div class="w-full h-screen">
    <!-- <section id="bottom-navigation" class="md:hidden block fixed inset-x-0 bottom-0 z-10 bg-white shadow"> // if shown only tablet/mobile-->
    <section
      id="bottom-navigation"
      class="block fixed inset-x-0 bottom-0 z-10 border-t-2 border-sky-500"
    >
      <nav class="bg-gray-800 bg-opacity-50">
        <div
          class="container flex items-center justify-evenly p-6 mx-auto text-gray-600 capitalize dark:text-gray-300"
        >
          <RouterLink
            to="/"
            class="text-gray-200 border-b-2 border-purple-500 hover:border-emerald-500 mx-4 text-xl"
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
