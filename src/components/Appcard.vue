<script>
import { EyeIcon, PlayIcon, TrashIcon } from "@heroicons/vue/24/solid";
export default {
  name: "Appcard",
  components: { EyeIcon, PlayIcon, TrashIcon },
  props: ["applicationdata"],
  methods: {
    getAppDetail(id) {
      this.$router.push({ name: "appDetail", params: { id } });
    },
  },
};
</script>

<template>
  <div class="relative group p-20">
    <div
      class="relative group cursor-pointer overflow-hidden duration-500 max-w-[300px] max-h-[500px] text-gray-50 bg-yellow-300"
    >
      <img class="object-cover" v-bind:src="applicationdata.imgDir" />

      <div class="">
        <div class="group-hover:scale-110 w-full h-60 duration-500"></div>
      </div>
      <div
        class="absolute left-0 p-5 w-full -bottom-16 duration-500 group-hover:-translate-y-12 flex justify-evenly justify-items-stretch bg-green-600"
      >
        <EyeIcon
          class="h-6 w-6 text-blue-500"
          v-on:click="getAppDetail(applicationdata.id)"
        />
        <PlayIcon class="h-6 w-6 text-blue-500" />
        <TrashIcon class="h-6 w-6 text-blue-500" />
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
