<script>
import { EyeIcon, PlayIcon, TrashIcon } from "@heroicons/vue/24/solid";
import ModalDelete from "./ModalDelete.vue";

export default {
  name: "Appcard",
  components: { EyeIcon, PlayIcon, TrashIcon, ModalDelete },
  props: ["applicationdata"],
  methods: {
    getAppDetail(id) {
      this.$router.push({ name: "appDetail", params: { id } });
    },
  },
};
</script>
<!-- https://webcraft-notes.com/blog/mastering-reusable-modals-in-vuejs-enhancing-ui -->
<template>
  <div class="relative m-5">
    <div
      class="relative group cursor-pointer overflow-hidden duration-500 text-gray-50 mx-auto"
    >
      <img
        class="object-scale-down w-max-[400px] h-[650px] items-center justify-center mx-auto"
        v-bind:src="applicationdata.imgDir"
      />

      <div
        class="absolute left-0 w-full opacity-60 duration-500 group-hover:-translate-y-12 flex justify-evenly bg-gray-900 items-center py-3"
      >
        <EyeIcon
          class="h-6 w-6 text-blue-500 duration-500 hover:fill-orange-300"
          v-on:click="getAppDetail(applicationdata.id)"
        />
        <PlayIcon
          class="h-6 w-6 text-blue-500 duration-500 hover:fill-orange-300"
        />
        <TrashIcon
          class="h-6 w-6 text-blue-500 duration-500 hover:fill-orange-300"
        />
      </div>
    </div>
    <p class="text-white text-3xl text-center">{{ applicationdata.title }}</p>

    <!-- <h3 class="text-white">{{ applicationdata.title }}</h3>
    <br />
    <img v-bind:src="applicationdata.imgDir" />
    <button class="text-white" v-on:click="getAppDetail(applicationdata.id)">
      Detail
    </button> -->
  </div>
  <!-- <div class="card">
    <div class="card2"></div>
  </div> -->
</template>
