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
  <div class="relative group p-20">
    <div
      class="relative group cursor-pointer overflow-hidden duration-500 text-gray-50 bg-yellow-300"
    >
      <img
        class="object-cover w-[350px] h-[475px]"
        v-bind:src="applicationdata.imgDir"
      />

      <div
        class="absolute left-0 p-3 w-full opacity-80 duration-500 group-hover:-translate-y-12 flex justify-evenly bg-gray-900 items-center"
      >
        <EyeIcon
          class="h-6 w-6 text-blue-500 duration-500 hover:fill-none hover:stroke-orange-300"
          v-on:click="getAppDetail(applicationdata.id)"
        />
        <PlayIcon
          class="h-6 w-6 text-blue-500 duration-500 hover:fill-none hover:stroke-orange-300"
        />
        <TrashIcon
          class="h-6 w-6 text-blue-500 duration-500 hover:fill-none hover:stroke-orange-300"
        />
      </div>
    </div>

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
