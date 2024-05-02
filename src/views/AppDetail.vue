<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import BottomNav from "../components/BottomNav.vue";
import {
  ArrowLeftIcon,
  EyeIcon,
  PencilSquareIcon,
  PlayIcon,
  TrashIcon,
} from "@heroicons/vue/24/solid";
import ModalDelete from "../components/ModalDelete.vue";
import { invoke } from "@tauri-apps/api/tauri";

export default {
  name: "AppDetail",
  components: {
    ArrowLeftIcon,
    EyeIcon,
    PencilSquareIcon,
    PlayIcon,
    TrashIcon,
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
    editApp(id) {
      this.$router.push({ name: "editapppage", params: { id } });
    },
    launchApp(exeDir) {
      invoke("launch_app", { exeDir });
    },
  },
  computed: {
    ...mapState(useFetchStore, ["applistbyid"]),
  },
  async created() {
    await this.fetchAppById(this.$route.params.id);
    let date = new Date(this.applistbyid.relDate);

    let year = date.getFullYear();
    let month = date.getMonth() + 1;
    let dt = date.getDate();

    if (dt < 10) {
      dt = "0" + dt;
    }
    if (month < 10) {
      month = "0" + month;
    }
    this.finalDate = year + "-" + month + "-" + dt;
  },
  data() {
    return {
      finalDate: "",
    };
  },
};
</script>

<template>
  <div
    class="relative max-w-full overflow-hidden bg-cover bg-[50%] bg-no-repeat max-h-screen"
  >
    <img class="bg-cover w-screen h-screen" :src="applistbyid.bgDir" />
    <div
      class="absolute bottom-0 left-0 right-0 top-0 h-full w-full overflow-hidden bg-gradient-to-b from-transparent to-black/75"
    >
      <div
        class="mt-10 mb-20 bg-gray-100 bg-opacity-10 mx-10 backdrop-blur-sm shadow-lg rounded-xl bg-clip-content"
      >
        <div class="grid grid-cols-3 gap-10 p-5">
          <div class="col-span-1 flex justify-center items-center">
            <img v-bind:src="applistbyid.imgDir" class="max-h-[700px]" />
          </div>
          <div class="p-8 col-span-2">
            <p class="text-5xl mb-8">{{ applistbyid.title }}</p>
            <p class="mt-3 font-medium text-gray-900">Developer</p>
            <p class="text-xl text-gray-800">{{ applistbyid.developer }}</p>
            <p class="mt-3 font-medium text-gray-900">Publisher</p>
            <p class="text-xl text-gray-800">{{ applistbyid.publisher }}</p>
            <p class="mt-3 font-medium text-gray-900">Description</p>
            <p class="text-xl text-gray-800">{{ applistbyid.desc }}</p>
            <p class="mt-3 font-medium text-gray-900">Language</p>
            <p class="text-xl text-gray-800">{{ applistbyid.lang }}</p>
            <p class="mt-3 font-medium text-gray-900">Release Date</p>
            <p class="text-xl text-gray-800">{{ finalDate }}</p>
          </div>
        </div>
        <div class="flex justify-end py-6 my-4">
          <ArrowLeftIcon
            class="h-12 w-12 text-blue-700 cursor-pointer duration-500 hover:scale-125 hover:fill-blue-600 active:fill-blue-400 active:scale-100 mx-12"
            v-on:click="goBack"
          />
          <PencilSquareIcon
            class="h-12 w-12 text-orange-700 cursor-pointer duration-500 hover:scale-125 active:scale-100 hover:fill-orange-600 active:fill-orange-400 mx-12"
            v-on:click="editApp(applistbyid.id)"
          />
          <PlayIcon
            class="h-12 w-12 text-green-700 cursor-pointer duration-500 hover:scale-125 active:scale-100 hover:fill-green-600 active:fill-green-400 mx-12"
            v-on:click="launchApp(applistbyid.exeDir)"
          />
          <TrashIcon
            class="h-12 w-12 text-red-700 cursor-pointer duration-500 hover:scale-125 active:scale-100 hover:fill-red-600 active:fill-red-400 mx-12"
            v-on:click="
              openDeleteModal(
                applistbyid.id,
                'Delete',
                'Are you sure you want to delete this?'
              )
            "
          />
        </div>
      </div>
    </div>
  </div>

  <BottomNav />
</template>
