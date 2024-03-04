<script>
export default {
  name: "Appcard",
  props: ["applicationdata"],
};
</script>

<template>
  <div class="app-card">
    <h3>{{ applicationdata.title }}</h3>
    <br />
    <img v-bind:src="applicationdata.imgDir" />
  </div>
</template>
