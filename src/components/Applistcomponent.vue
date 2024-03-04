<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import Appcard from "./Appcard.vue";
export default {
  name: "Applistcomponent",
  components: {
    Appcard,
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
  <div class="applist-body">
    <h1>Ini app list component</h1>
    <div class="list-grid-wrapper">
      <Appcard v-for="app in applists" :applicationdata="app" :key="app.id" />
      <!-- <div class="app-card">
        <h3>Amairo Chocolata</h3>
        <br />
        <img src="../assets/covercatalog/Amachoco.jpg" />
      </div>
      <div class="app-card">
        <h3>Amairo Chocolata 2</h3>
        <img src="../assets/covercatalog/Amachoco2.jpg" />
      </div>
      <div class="app-card">
        <h3>Amairo Chocolata 3</h3>
        <img src="../assets/covercatalog/Amachoco3.jpg" />
      </div>
      <div class="app-card">
        <h3>D.C.4</h3>
        <img src="../assets/covercatalog/DC4.jpg" />
      </div>
      <div class="app-card">
        <h3>D.C.4 -Fortunate Departures-</h3>
        <img src="../assets/covercatalog/DC4FD.jpg" />
      </div>
      <div class="app-card">
        <h3>D.C.5</h3>
        <img src="../assets/covercatalog/DC5.jpg" />
      </div>
      <div class="app-card">
        <h3>D.C.5 -Future Link-</h3>
        <img src="../assets/covercatalog/DC5FL.jpeg" />
      </div>
      <div class="app-card">
        <h3>D.S. -Dal Segno-</h3>
        <img src="../assets/covercatalog/DS.jpg" />
      </div>
      <div class="app-card">
        <h3>D.S.i.F. -Dal Segno in Future-</h3>
        <img src="../assets/covercatalog/DSIF.jpg" />
      </div> -->
    </div>
  </div>
</template>
