<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import BottomNav from "../components/BottomNav.vue";
import { ArrowLeftIcon } from "@heroicons/vue/24/solid";

export default {
  name: "FormappView",
  components: { BottomNav, ArrowLeftIcon },
  computed: {
    ...mapState(useFetchStore, ["applistbyid"]),
  },
  methods: {
    ...mapActions(useFetchStore, [
      "addApplication",
      "editApplication",
      "fetchAppById",
    ]),
    async addappHandler() {
      await this.addApplication(this.formData);
    },
    async editappHandler() {
      await this.editApplication(this.formData);
    },
    goBack() {
      this.$router.go(-1);
    },
  },
  data() {
    return {
      formData: {
        id: this.applistbyid?.id || "",
        title: this.applistbyid?.title || "",
        developer: this.applistbyid?.developer || "",
        publisher: this.applistbyid?.publisher || "",
        exeDir: this.applistbyid?.exeDir || "",
        imgDir: this.applistbyid?.imgDir || "",
        bgDir: this.applistbyid?.bgDir || "",
        desc: this.applistbyid?.desc || "",
        lang: this.applistbyid?.lang || "",
        relDate: this.applistbyid?.relDate || "",
      },
    };
  },
  async created() {
    if (this.$route.params.id) {
      await this.fetchAppById(this.$route.params.id);
      this.formData.id = this.applistbyid.id;
      this.formData.title = this.applistbyid.title;
      this.formData.developer = this.applistbyid.developer;
      this.formData.publisher = this.applistbyid.publisher;
      this.formData.exeDir = this.applistbyid.exeDir;
      this.formData.imgDir = this.applistbyid.imgDir;
      this.formData.bgDir = this.applistbyid.bgDir;
      this.formData.desc = this.applistbyid.desc;
      this.formData.lang = this.applistbyid.lang;
      this.formData.relDate = this.applistbyid.relDate;
    }
  },
};
</script>

<template>
  <div class="mainbody h-screen">
    <h1 class="text-white text-2xl">
      {{ !$route.params.id ? "Add new" : "Edit" }} application
    </h1>
    <div class="grid grid-cols-1 gap-4">
      <!-- Add form -->
      <form
        class="form-add-app"
        v-on:submit.prevent="addappHandler"
        v-if="!$route.params.id"
      >
        <div class="input flex flex-col w-fit static">
          <label
            for="title"
            class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
            >Title:</label
          >
          <input
            id="title"
            type="text"
            placeholder="Application name"
            name="title"
            v-model="formData.title"
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
            v-model="formData.developer"
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
            v-model="formData.publisher"
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
            v-model="formData.exeDir"
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
            v-model="formData.imgDir"
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
            v-model="formData.bgDir"
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
            v-model="formData.desc"
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
            v-model="formData.lang"
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
            v-model="formData.relDate"
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

      <!-- Edit form -->
      <form
        class="form-add-app"
        v-on:submit.prevent="editappHandler"
        v-if="$route.params.id"
      >
        <div class="input flex flex-col w-fit static">
          <label
            for="title"
            class="text-blue-500 text-xs font-semibold relative top-2 ml-[7px] px-[3px] bg-[#e8e8e8] w-fit"
            >Title:</label
          >
          <input
            id="title"
            type="text"
            placeholder="Application name"
            name="title"
            v-model="formData.title"
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
            v-model="formData.developer"
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
            v-model="formData.publisher"
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
            v-model="formData.exeDir"
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
            v-model="formData.imgDir"
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
            v-model="formData.bgDir"
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
            v-model="formData.desc"
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
            v-model="formData.lang"
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
            v-model="formData.relDate"
            class="border-blue-500 input px-[10px] py-[11px] text-base bg-[#252525] border-2 rounded-[5px] w-[410px] focus:outline-none placeholder:text-white/25 text-white"
          />
        </div>
        <div class="flex flex-row justify-evenly">
          <button
            class="mt-10 py-2 px-4 text-xl bg-cyan-600 text-white hover:bg-cyan-500 active:bg-cyan-400 ring-2 ring-offset-2 ring-offset-slate-900 ring-blue-500 rounded-lg"
            v-on:click="goBack"
          >
            < Back
          </button>
          <button
            class="mt-10 py-2 px-4 text-xl bg-green-600 text-white hover:bg-green-500 active:bg-green-400 ring-2 ring-offset-2 ring-offset-slate-900 ring-emerald-500 rounded-lg"
            type="submit"
          >
            Edit
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
