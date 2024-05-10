<script>
import { RouterView } from "vue-router";
import TitleBar from "./components/TitleBar.vue";
export default {
  name: "App",
  components: { TitleBar },
};
</script>

<template>
  <TitleBar />
  <RouterView />
</template>

<style></style>
