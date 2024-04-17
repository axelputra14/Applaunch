<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import BottomNav from "../components/BottomNav.vue";
import { ArrowLeftIcon } from "@heroicons/vue/24/solid";
export default {
  name: "AppDetail",
  components: {
    ArrowLeftIcon,
    BottomNav,
  },
  methods: {
    ...mapActions(useFetchStore, ["fetchAppById", "deleteApplication"]),
    goBack() {
      this.$router.go(-1);
    },
    deleteApp(id) {
      this.deleteApplication(id);
    },
  },
  computed: {
    ...mapState(useFetchStore, ["applistbyid"]),
  },
  created() {
    this.fetchAppById(this.$route.params.id);
  },
};
</script>

<template>
  <div class="body">
    <div class="mainbody">
      <ArrowLeftIcon
        class="h-12 w-12 text-blue-500 cursor-pointer duration-200 hover:scale-125 active:scale-100"
        v-on:click="goBack"
      />
      <!-- <button
        class="cursor-pointer duration-200 hover:scale-125 active:scale-100"
        title="Go Back"
        v-on:click="goBack"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="50px"
          height="50px"
          viewBox="0 0 24 24"
          class="stroke-blue-300"
        >
          <path
            stroke-linejoin="round"
            stroke-linecap="round"
            stroke-width="1.5"
            d="M11 6L5 12M5 12L11 18M5 12H19"
          ></path>
        </svg>
      </button> -->
      <div class="contentbody">
        <h2 class="text-3xl font-bold underline">Ini halaman detail app</h2>
        <br />
        <h1 class="text-4xl text-white">{{ applistbyid.title }}</h1>
      </div>
      <div class="temp">
        <button class="text-red-400" v-on:click="deleteApp(applistbyid.id)">
          Delete
        </button>
      </div>
      <BottomNav />
    </div>
  </div>
</template>
