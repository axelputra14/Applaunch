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

async function launchApp(exeDir) {
  await invoke("launch_app", { exeDir });
}

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
    class="relative max-w-full max-h-screen bg-[50%] bg-no-repeat overflow-hidden"
  >
    <img
      class="min-w-screen min-h-screen"
      :src="'http://localhost:25850/bg/' + applistbyid.bgDir"
    />
    <div
      class="absolute bottom-0 left-0 right-0 top-0 h-full w-full bg-gradient-to-b from-transparent to-black/75"
    >
      <div
        class="mt-5 mb-20 bg-gray-100 bg-opacity-30 mx-10 backdrop-blur-sm shadow-lg rounded-xl bg-clip-content overflow-auto"
      >
        <div class="grid grid-cols-3 gap-10 p-5">
          <div class="col-span-1 flex justify-center items-center">
            <img
              v-bind:src="'http://localhost:25850/cover/' + applistbyid.imgDir"
              class="max-h-[700px]"
            />
          </div>
          <div class="col-span-2 drop-shadow-sm">
            <p class="text-5xl mb-5">{{ applistbyid.title }}</p>
            <p class="mt-2 text-lg font-semibold text-gray-800">Developer</p>
            <p class="text-lg text-gray-900">{{ applistbyid.developer }}</p>
            <p class="mt-2 text-lg font-semibold text-gray-800">Publisher</p>
            <p class="text-lg text-gray-900">{{ applistbyid.publisher }}</p>
            <p class="mt-2 text-lg font-semibold text-gray-800">Description</p>
            <p class="text-lg text-gray-900">{{ applistbyid.desc }}</p>
            <p class="mt-2 text-lg font-semibold text-gray-800">Language</p>
            <p class="text-lg text-gray-900">{{ applistbyid.lang }}</p>
            <p class="mt-2 text-lg font-semibold text-gray-800">Release Date</p>
            <p class="text-lg text-gray-900">{{ finalDate }}</p>
          </div>
        </div>
      </div>
    </div>
    <div
      class="flex justify-center py-6 ml-2 my-4 sticky bottom-12 overflow-hidden"
    >
      <ArrowLeftIcon
        class="h-12 w-12 text-blue-500 cursor-pointer duration-500 hover:scale-125 hover:fill-blue-400 active:fill-blue-300 active:scale-100 mx-12"
        v-on:click="goBack"
      />
      <PlayIcon
        class="h-12 w-12 text-emerald-500 cursor-pointer duration-500 hover:scale-125 active:scale-100 hover:fill-emerald-400 active:fill-emerald-300 mx-12"
        v-on:click="launchApp(applistbyid.exeDir)"
      />
      <PencilSquareIcon
        class="h-12 w-12 text-amber-500 cursor-pointer duration-500 hover:scale-125 active:scale-100 hover:fill-amber-400 active:fill-amber-300 mx-12"
        v-on:click="editApp(applistbyid.id)"
      />
      <TrashIcon
        class="h-12 w-12 text-red-500 cursor-pointer duration-500 hover:scale-125 active:scale-100 hover:fill-red-400 active:fill-red-300 mx-12"
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

  <BottomNav />
</template>
