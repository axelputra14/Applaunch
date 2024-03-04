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
  <div class="bottom-bar-overall">
    <ul class="bottom-bar-container">
      <li class="navbar-item">
        <!-- <a href="#"> -->
        <RouterLink to="/">
          <span class="navbar-icon"><i class="bi bi-view-stacked"></i></span>
          <span class="navbar-label">Apps</span>
          <div id="bottom-navbar-indicator" class="nav-indicator"></div>
        </RouterLink>
        <!-- </a> -->
      </li>
      <li class="navbar-item">
        <!-- <a href="#"> -->
        <RouterLink to="/app-add">
          <span class="navbar-icon"><i class="bi bi-plus-square"></i></span>
          <span class="navbar-label">Add</span>
          <div id="bottom-navbar-indicator" class="nav-indicator"></div>
        </RouterLink>
        <!-- </a> -->
      </li>
      <!-- indicator -->
      <!-- <div id="bottom-navbar-indicator" class="nav-indicator"></div> -->
    </ul>
  </div>
</template>
