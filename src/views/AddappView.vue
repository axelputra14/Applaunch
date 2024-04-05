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
        bgDir: "",
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
      <div class="input flex flex-col w-fit static">
        <label
          for="title"
          class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
          >Title:</label
        >
        <input
          id="title"
          type="text"
          placeholder="App name"
          name="title"
          v-model="addData.title"
          class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
        />

        <label
          for="developer"
          class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
          >Developer:</label
        >
        <input
          id="developer"
          type="text"
          placeholder="Developer name"
          name="developer"
          v-model="addData.developer"
          class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
        />

        <label
          for="publisher"
          class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
          >Publisher:</label
        >
        <input
          id="publisher"
          type="text"
          placeholder="Publisher name"
          name="publisher"
          v-model="addData.publisher"
          class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
        />

        <label
          for="exedir"
          class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
          >Executable Directory:</label
        >
        <input
          id="exedir"
          type="text"
          placeholder="Executable directory"
          name="exedir"
          v-model="addData.exeDir"
          class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
        />

        <label
          for="imgdir"
          class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
          >Image Directory:</label
        >
        <input
          id="imgdir"
          type="text"
          placeholder="Image directory"
          name="imgdir"
          v-model="addData.imgDir"
          class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
        />

        <label
          for="bgdir"
          class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
          >Background Image Directory:</label
        >
        <input
          id="bgdir"
          type="text"
          placeholder="Background image directory"
          name="bgdir"
          v-model="addData.bgDir"
          class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
        />

        <label
          for="description"
          class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
          >Executable Directory:</label
        >
        <textarea
          id="description"
          type="text"
          placeholder="Description"
          name="description"
          v-model="addData.desc"
          class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
        ></textarea>
        <label
          for="bgdir"
          class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
          >Background Image Directory:</label
        >
        <input
          id="lang"
          type="text"
          placeholder="Main language of the app"
          name="lang"
          v-model="addData.lang"
          class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
        />

        <label
          for="relDate"
          class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
          >Date:</label
        >
        <input
          id="relDate"
          type="text"
          placeholder="Release date in YYYY-MM-DD (e.g. 2015-06-14) format"
          name="relDate"
          v-model="addData.relDate"
          class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
        />
      </div>

      <fwb-button color="green" type="submit">Add</fwb-button>
    </form>
    <BottomNav />
  </div>
</template>
