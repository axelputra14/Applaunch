<script>
import BottomNav from "../components/BottomNav.vue";
import Applistcomponent from "../components/Applistcomponent.vue";
export default {
  name: "ApplistView",
  components: {
    BottomNav,
    Applistcomponent,
  },
};
</script>

<template>
  <div class="body">
    <div class="mainbody">
      <div class="contentbody h-max overflow-auto">
        <br />

        <Applistcomponent />
      </div>
      <BottomNav />
    </div>
  </div>
</template>
