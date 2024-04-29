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
        class="my-10 bg-gray-100 bg-opacity-10 mx-10 backdrop-blur-sm shadow-lg rounded-xl bg-clip-content"
      >
        <div class="">
          <div class="p-8">
            <p class="text-5xl">{{ applistbyid.title }}</p>
            <p>{{ applistbyid.developer }}</p>
            <p>{{ applistbyid.publisher }}</p>
            <p>{{ applistbyid.exeDir }}</p>
            <p>{{ applistbyid.imgDir }}</p>
            <p>{{ applistbyid.desc }}</p>
            <p>{{ applistbyid.lang }}</p>
            <p>{{ finalDate }}</p>
          </div>

          <div class="flex justify-end py-6 my-4">
            <ArrowLeftIcon
              class="h-12 w-12 text-emerald-400 cursor-pointer duration-500 hover:scale-125 active:scale-100 mx-12"
              v-on:click="goBack"
            />
            <PencilSquareIcon
              class="h-12 w-12 text-blue-500 cursor-pointer duration-500 hover:scale-125 active:scale-100 hover:fill-orange-300 active:fill-green-400 mx-12"
            />
            <PlayIcon
              class="h-12 w-12 text-blue-500 cursor-pointer duration-500 hover:scale-125 active:scale-100 hover:fill-orange-300 active:fill-green-400 mx-12"
            />
            <TrashIcon
              class="h-12 w-12 text-blue-500 cursor-pointer duration-500 hover:scale-125 active:scale-100 hover:fill-orange-300 active:fill-green-400 mx-12"
            />
          </div>
        </div>
      </div>
    </div>
  </div>

  <BottomNav />
</template>
