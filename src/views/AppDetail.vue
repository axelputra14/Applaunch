<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import BottomNav from "../components/BottomNav.vue";
import { ArrowLeftIcon } from "@heroicons/vue/24/solid";
import ModalDelete from "../components/ModalDelete.vue";
export default {
  name: "AppDetail",
  components: {
    ArrowLeftIcon,
    BottomNav,
    ModalDelete,
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
    <ArrowLeftIcon
      class="h-12 w-12 text-blue-500 cursor-pointer duration-200 hover:scale-125 active:scale-100"
      v-on:click="goBack"
    />
    <!-- <div class="contentbody"> -->
    <!-- <h2 class="text-3xl font-bold underline">Ini halaman detail app</h2>
        <br />
        <h1 class="text-4xl text-white">{{ applistbyid.title }}</h1>
      </div>
      <div class="temp">
        <button class="text-red-400" v-on:click="deleteApp(applistbyid.id)">
          Delete
        </button> JANGAN HAPUS -->
    <div class="bg-gray-100">
      <!-- Header -->
      <header class="bg-gray-900 text-white py-4">
        <div class="container mx-auto">
          <h1 class="text-2xl font-bold">Game Title</h1>
        </div>
      </header>

      <!-- Main Content -->
      <main class="container mx-auto mt-8">
        <!-- Game Banner -->
        <section>
          <img
            v-bind:src="applistbyid.bgDir"
            alt="Game Banner"
            class="w-full h-auto"
          />
        </section>

        <!-- Game Details -->
        <section class="mt-8">
          <!-- Game Logo -->
          <div class="flex items-center">
            <!-- <img src="game_logo.png" alt="Game Logo" class="w-16 h-16 mr-4" /> -->
            <h2 class="text-xl font-bold">Game Title</h2>
          </div>
          <!-- Playtime and Other Information -->
          <div class="mt-4">
            <p><strong>Playtime:</strong> 10 hours</p>
            <p><strong>Achievements:</strong> 20</p>
            <!-- Add more game details here -->
          </div>
        </section>
      </main>
    </div>
    <BottomNav />
  </div>
</template>
