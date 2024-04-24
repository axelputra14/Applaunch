<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import BottomNav from "../components/BottomNav.vue";

export default {
  name: "AddappView",
  components: { BottomNav },
  computed: {
    ...mapState(useFetchStore, ["applistbyid"]),
  },
  methods: {
    ...mapActions(useFetchStore, ["addApplication", "fetchAppById"]),
    addappHandler() {
      this.addApplication(this.formData);
    },
  },
  data() {
    return {
      formData: {
        title: this.fetchAppById?.title || "",
        developer: this.fetchAppById?.developer || "",
        publisher: this.fetchAppById?.publisher || "",
        exeDir: this.fetchAppById?.exeDir || "",
        imgDir: this.fetchAppById?.imgDir || "",
        bgDir: this.fetchAppById?.bgDir || "",
        desc: this.fetchAppById?.desc || "",
        lang: this.fetchAppById?.lang || "",
        relDate: this.fetchAppById?.relDate || "",
      },
    };
  },
};
</script>

<template>
  <div class="mainbody h-screen">
    <h1 class="text-white text-2xl">Add new application</h1>
    <div class="grid grid-cols-1 gap-4">
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
            class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white focus:text-white/"
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
            >Description:</label
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
            >Language:</label
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
            >Release Date:</label
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
        <div class="flex flex-row justify-evenly">
          <button
            class="mt-10 py-2 px-4 text-xl bg-green-600 text-white hover:bg-green-500 active:bg-green-400 ring-2 ring-offset-2 ring-offset-slate-900 ring-emerald-500 rounded-lg"
            type="submit"
          >
            Add
          </button>

          <button
            class="mt-10 py-2 px-4 text-xl bg-gray-600 text-white hover:bg-gray-500 active:bg-gray-400 ring-2 ring-offset-2 ring-offset-slate-900 ring-slate-500 rounded-lg"
            type="reset"
          >
            Reset
          </button>
        </div>
      </form>
    </div>
    <BottomNav />
  </div>
</template>
