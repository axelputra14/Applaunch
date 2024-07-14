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
      <div class="contentbody w-11/12 h-max overflow-auto">
        <Applistcomponent />
      </div>
      <BottomNav />
    </div>
  </div>
</template>
