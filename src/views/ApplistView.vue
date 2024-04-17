<script>
import BottomNav from "../components/BottomNav.vue";
import Applistcomponent from "../components/Applistcomponent.vue";
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
export default {
  name: "ApplistView",
  components: {
    BottomNav,
    Applistcomponent,
  },
  methods: {
    ...mapActions(useFetchStore, ["fetchApplists"]),
  },
  computed: {
    ...mapState(useFetchStore, ["tesvar", "applists"]),
  },
  created() {
    this.fetchApplists();
  },
};
</script>

<template>
  <div class="body">
    <div class="mainbody">
      <div class="contentbody">
        <br />

        <Applistcomponent />

        <!-- <p>Tes data & {{ tesvar }}</p>
        <br />
        <p>{{ applists }}</p> -->
      </div>
      <BottomNav />
    </div>
  </div>
</template>
