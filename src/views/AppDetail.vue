<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import BottomNav from "../components/BottomNav.vue";
import { ArrowLeftIcon } from "@heroicons/vue/24/solid";
import ModalDelete from "../components/ModalDelete.vue";
import applist from "../apis/applist";
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
  <div
    class="body bg-contain"
    :style="{ backgroundImage: 'url(' + applistbyid.bgDir + ')' }"
  >
    <!-- Button Strip Section -->
    <div class="flex justify-evenly py-4 bg-black bg-opacity-15">
      <ArrowLeftIcon
        class="h-12 w-12 text-blue-500 cursor-pointer duration-200 hover:scale-125 active:scale-100"
        v-on:click="goBack"
      />
      <button
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
      >
        Play
      </button>
      <button
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
      >
        Edit
      </button>
      <button
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
      >
        Delete
      </button>
    </div>

    <!-- Rest of the Content -->
    <div class="p-8">
      <!-- Lorem Ipsum paragraphs or any other content -->
      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit...</p>
      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit...</p>
      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit...</p>
      <!-- Add more paragraphs or content as needed -->
    </div>

    <BottomNav />
  </div>
</template>
