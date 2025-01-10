<script>
import BottomNav from "../components/BottomNav.vue";
import Applistcomponent from "../components/Applistcomponent.vue";
import Bubbles from "../components/Bubbles.vue";

export default {
  name: "ApplistView",
  components: {
    BottomNav,
    Applistcomponent,
    Bubbles,
  },
};
</script>

<template>
  <div class="body">
    <Bubbles />
    <div class="mainbody">
      <div class="contentbody w-11/12 h-max overflow-auto">
        <Applistcomponent />
      </div>
      <BottomNav />
    </div>
  </div>
</template>
