<script>
export default {
  name: "Appcard",
  props: ["applicationdata"],
  methods: {
    getAppDetail(id) {
      this.$router.push({ name: "appDetail", params: { id } });
    },
  },
};
</script>

<template>
  <div class="relative group">
    <div
      class="relative group cursor-pointer overflow-hidden duration-500 w-64 h-80 bg-zinc-800 text-gray-50"
    >
      <img v-bind:src="applicationdata.imgDir" />
      <div class="">
        <div
          class="group-hover:scale-110 w-full h-60 bg-blue-400 duration-500"
        ></div>
      </div>
      <div
        class="absolute w-56 left-0 p-5 -bottom-16 duration-500 group-hover:-translate-y-12"
      >
        <div
          class="absolute -z-10 left-0 w-64 h-28 opacity-0 duration-500 group-hover:opacity-0 group-hover:bg-blue-900"
        ></div>
        <span class="text-xl font-bold">Hover me!</span>
        <p class="group-hover:opacity-100 w-56 duration-500 opacity-0">
          Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        </p>
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
