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
  <div class="card">
    <h3 class="text-white">{{ applicationdata.title }}</h3>
    <br />
    <img v-bind:src="applicationdata.imgDir" />
    <button class="text-white" v-on:click="getAppDetail(applicationdata.id)">
      Detail
    </button>
  </div>
  <!-- <div class="card">
    <div class="card2"></div>
  </div> -->
</template>
