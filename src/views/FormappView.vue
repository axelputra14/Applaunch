<script>
import { mapActions, mapState } from "pinia";
import { useFetchStore } from "../stores/fetch";
import BottomNav from "../components/BottomNav.vue";
import { ArrowLeftIcon, FolderOpenIcon } from "@heroicons/vue/24/solid";
import { invoke } from "@tauri-apps/api/tauri";
export default {
  name: "FormappView",
  components: { BottomNav, ArrowLeftIcon, FolderOpenIcon },
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
    async selectExeFile() {
      try {
        const exe_path = await invoke("select_exe");

        this.exePath = exe_path;
        this.formData.exeDir = this.exePath;
        this.showExeDirErr = false;
      } catch (error) {
        this.exeErrMsg = error;
        this.showExeDirErr = true;
      }
    },
    async selectImgFile() {
      try {
        const img_path = await invoke("select_img");
        this.imgPath = img_path;
        this.formData.imgDir = this.imgPath;
        this.showImgDirErr = false;
      } catch (error) {
        this.imgErrMsg = error;
        this.showImgDirErr = true;
      }
    },
    setFormType(type) {
      if (type === "add") {
        this.addappHandler();
      } else if (type === "edit") {
        this.editappHandler();
      }
    },
  },
  data() {
    return {
      formData: {
        id: this.applistbyid?.id || "",
        title: this.applistbyid?.title || "",
        developer: this.applistbyid?.developer || "",
        exeDir: this.applistbyid?.exeDir || "",
        imgDir: this.applistbyid?.imgDir || "",
      },
      showExeDirErr: false,
      showImgDirErr: false,
      exePath: "",
      imgPath: "",
      exeErrMsg: "",
      imgErrMsg: "",
      formActionType: "",
    };
  },
  async created() {
    if (this.$route.params.id) {
      await this.fetchAppById(this.$route.params.id);
      this.formData.id = this.applistbyid.id;
      this.formData.title = this.applistbyid.title;
      this.formData.developer = this.applistbyid.developer;
      this.formData.exeDir = this.applistbyid.exeDir;
      this.formData.imgDir = this.applistbyid.imgDir;
    }
  },
};
</script>

<template>
  <div class="mainbody h-screen">
    <div class="relative p-4 w-screen h-full">
      <!-- Modal header -->
      <div
        class="flex justify-between items-center pb-4 rounded-t border-b mb-5 border-lime-600"
      >
        <h3 class="text-lg font-semibold text-white">
          {{ !$route.params.id ? "Add new" : "Edit" }} application
        </h3>
        <button
          type="button"
          class="text-gray-200 bg-transparent hover:rounded-lg text-sm p-1.5 ml-auto inline-flex items-center hover:bg-gray-400 hover:text-white"
          v-on:click="goBack"
        >
          <!-- Change to heroicon later. TBA remove, keep, or change to left arrow icon-->
          <svg
            aria-hidden="true"
            class="w-5 h-5"
            fill="currentColor"
            viewBox="0 0 20 20"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              fill-rule="evenodd"
              d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
              clip-rule="evenodd"
            ></path>
          </svg>
        </button>
      </div>

      <form autocomplete="off" v-on:submit.prevent="">
        <div class="grid gap-4 mb-4 grid-cols-2">
          <div>
            <label for="title" class="block mb-2 text-sm font-medium text-white"
              >Title</label
            >
            <input
              type="text"
              name="title"
              id="title"
              class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-red-800 ring-red-800 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-red-800 focus:border-red-800 active:ring-red-800 active:border-red-800"
              placeholder="Application title"
              v-model="formData.title"
            />
          </div>
          <div>
            <label
              for="developer"
              class="block mb-2 text-sm font-medium text-white"
              >Developer</label
            >
            <input
              type="text"
              name="developer"
              id="developer"
              class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-orange-500 ring-orange-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-orange-500 focus:border-orange-500 active:ring-orange-500 active:border-orange-500"
              placeholder="Developer"
              v-model="formData.developer"
            />
          </div>

          <div>
            <label
              for="exedir"
              class="block mb-2 text-sm font-medium text-white"
              >Executable Directory</label
            >
            <div>
              <div class="grid grid-flow-col grid-cols-12 gap-2">
                <input
                  type="text"
                  name="exedir"
                  id="exedir"
                  class="bg-[#160b3b] col-span-11 border text-sm rounded-lg block w-full p-2.5 border-green-500 ring-green-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-green-500 focus:border-green-500 active:ring-green-500 active:border-green-500"
                  placeholder="Absolute directory of the executable"
                  v-model="formData.exeDir"
                />
                <button
                  class="flex justify-center items-center text-sm bg-[#160b3b] text-white hover:bg-green-500 active:bg-green-400 ring-1 ring-offset-0 ring-offset-slate-900 ring-green-600 rounded-lg"
                  v-on:click="selectExeFile"
                >
                  <FolderOpenIcon class="h-7 w-7" />
                </button>
              </div>

              <span v-if="showExeDirErr" class="text-red-600 text-xs">
                {{ exeErrMsg }}
              </span>
            </div>
          </div>

          <div>
            <label
              for="imgdir"
              class="block mb-2 text-sm font-medium text-white"
              >Cover Image</label
            >
            <div class="grid grid-flow-col grid-cols-12 gap-2">
              <input
                type="text"
                name="imgdir"
                id="imgdir"
                class="bg-[#160b3b] col-span-11 border text-sm rounded-lg block w-full p-2.5 border-cyan-500 ring-cyan-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-cyan-500 focus:border-cyan-500 active:ring-cyan-500 active:border-cyan-500"
                placeholder="Image file name for the cover"
                v-model="formData.imgDir"
              />
              <button
                class="flex justify-center items-center text-sm bg-[#160b3b] text-white hover:bg-cyan-500 active:bg-cyan-400 ring-1 ring-offset-0 ring-offset-slate-900 ring-cyan-500 rounded-lg"
                v-on:click="selectImgFile"
              >
                <FolderOpenIcon class="h-7 w-7" />
              </button>
            </div>
            <span v-if="showImgDirErr" class="text-red-600 text-xs">
              {{ imgErrMsg }}
            </span>
          </div>
        </div>
        <div class="flex flex-row justify-evenly">
          <button
            class="mt-10 py-2 px-4 text-xl bg-green-600 text-white hover:bg-green-500 active:bg-green-400 ring-2 ring-offset-2 ring-offset-slate-900 ring-emerald-500 rounded-lg"
            type="submit"
            v-if="!$route.params.id"
            v-on:click="setFormType('add')"
          >
            Add
          </button>

          <button
            class="mt-10 py-2 px-4 text-xl bg-cyan-600 text-white hover:bg-cyan-500 active:bg-cyan-400 ring-2 ring-offset-2 ring-offset-slate-900 ring-blue-500 rounded-lg"
            v-on:click="goBack"
            v-if="$route.params.id"
          >
            < Back
          </button>
          <button
            class="mt-10 py-2 px-4 text-xl bg-green-600 text-white hover:bg-green-500 active:bg-green-400 ring-2 ring-offset-2 ring-offset-slate-900 ring-emerald-500 rounded-lg"
            type="submit"
            v-if="$route.params.id"
            v-on:click="setFormType('edit')"
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
