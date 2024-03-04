<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import BottomNav from "../components/BottomNav.vue";
import { FwbInput, FwbButton, FwbTextarea } from "flowbite-vue";

export default {
  name: "AddappView",
  components: { BottomNav, FwbInput, FwbButton, FwbTextarea },
  data() {
    return {
      addData: {
        title: "",
        developer: "",
        publisher: "",
        exeDir: "",
        imgDir: "",
        desc: "",
        lang: "",
        relDate: "",
      },
    };
  },
  methods: {
    ...mapActions(useFetchStore, ["addApplication"]),
    addappHandler() {
      this.addApplication(this.addData);
    },
  },
};
</script>

<template>
  <div class="mainbody">
    <h1>Ini add app view</h1>
    <form class="form-add-app" v-on:submit.prevent="addappHandler">
      <fwb-input v-model="addData.title" label="Title" placeholder="App name">
      </fwb-input>
      <fwb-input
        v-model="addData.developer"
        label="Developer"
        placeholder="App developer"
      >
        <template #helper> We'll never share your details. Read our</template>
      </fwb-input>
      <fwb-input
        v-model="addData.publisher"
        label="Publisher"
        placeholder="App publisher"
      >
        <template #helper> We'll never share your details. Read our</template>
      </fwb-input>
      <fwb-input
        v-model="addData.exeDir"
        label="Executable Directory"
        placeholder="Location of the executable of the app"
      >
        <template #helper>.exe on Windows</template>
      </fwb-input>
      <fwb-input
        v-model="addData.imgDir"
        label="Image Directory"
        placeholder="Image to show on the launcher"
      >
        <template #helper>.jpg, .png, .jpeg, .webp</template>
      </fwb-input>
      <fwb-textarea
        v-model="addData.desc"
        :rows="5"
        label="Description"
        placeholder="App description"
      />
      <fwb-input
        v-model="addData.lang"
        label="Language"
        placeholder="Language (or main language) of the app"
      >
      </fwb-input>
      <!--date-->
      <fwb-input
        v-model="addData.relDate"
        label="Release Date"
        placeholder="Release date in YYYY-mm-DD (e.g. 2015-06-14) format"
      >
      </fwb-input>

      <fwb-button color="green" type="submit">Add</fwb-button>
    </form>
    <BottomNav />
  </div>
</template>
