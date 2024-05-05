<script>
import {
  EyeIcon,
  PencilSquareIcon,
  PlayIcon,
  TrashIcon,
} from "@heroicons/vue/24/solid";
import ModalDelete from "./ModalDelete.vue";
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import { invoke } from "@tauri-apps/api/tauri";

export default {
  name: "Appcard",
  components: { EyeIcon, PencilSquareIcon, PlayIcon, TrashIcon, ModalDelete },
  props: ["applicationdata"],
  methods: {
    ...mapActions(useFetchStore, ["deleteApplication"]),
    getAppDetail(id) {
      this.$router.push({ name: "appDetail", params: { id } });
    },
    launchApp(exeDir) {
      invoke("launch_app", { exeDir });
    },
    openDeleteModal(appid, button, content) {
      this.appId = appid;
      this.showModal = true;
      this.type = button;
      this.content = content;
    },
    async deleteAppById(appid) {
      this.showModal = false;
      await this.deleteApplication(appid);
    },
    editApp(id) {
      this.$router.push({ name: "editapppage", params: { id } });
    },
  },
  data() {
    return {
      appid: "",
      type: "",
      content: "",
      showModal: false,
    };
  },
};
</script>

<template>
  <ModalDelete
    v-if="showModal"
    @close="showModal = false"
    v-bind:applicationid="this.appId"
    @deleteApp="deleteAppById"
  >
    <template v-slot:header>
      <h1>{{ type }}</h1>
    </template>
    <template v-slot:content>
      <p>{{ content }}</p>
    </template>
  </ModalDelete>
  <div class="relative m-5">
    <div
      class="relative group cursor-pointer rounded-lg overflow-hidden duration-500 text-gray-50 mx-auto hover:bg-slate-50/5"
    >
      <img
        class="object-scale-down max-w-[400px] h-[650px] items-center justify-center mx-auto"
        v-bind:src="'http://localhost:25850/cover/' + applicationdata.imgDir"
      />

      <div
        class="absolute left-0 w-full bg-opacity-60 duration-500 group-hover:-translate-y-12 flex justify-evenly bg-gray-900 items-center py-3"
      >
        <EyeIcon
          class="h-6 w-6 text-sky-700 duration-500 hover:fill-sky-600 active:fill-sky-400"
          v-on:click="getAppDetail(applicationdata.id)"
        />
        <PencilSquareIcon
          class="h-6 w-6 text-orange-700 duration-500 hover:fill-orange-600 active:fill-orange-400"
          v-on:click="editApp(applicationdata.id)"
        />
        <PlayIcon
          class="h-6 w-6 text-sky-700 duration-500 hover:fill-sky-600 active:fill-sky-400"
          v-on:click="launchApp(applicationdata.exeDir)"
        />
        <TrashIcon
          class="h-6 w-6 text-red-700 duration-500 hover:fill-red-600 active:fill-red-400"
          v-on:click="
            openDeleteModal(
              applicationdata.id,
              'Delete',
              'Are you sure you want to delete this?'
            )
          "
        />
      </div>
    </div>
    <p class="text-white text-3xl text-center">{{ applicationdata.title }}</p>
  </div>
</template>
