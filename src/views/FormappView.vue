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
    <!-- <h1 class="text-white text-2xl">
      {{ !$route.params.id ? "Add new" : "Edit" }} application
    </h1> -->

    <div class="relative p-4 w-screen h-full">
      <!-- Modal content -->
      <div class="relative p-4 rounded-lg shadow">
        <!-- Modal header -->
        <div
          class="flex justify-between items-center pb-4 mb-4 rounded-t border-b sm:mb-5 dark:border-gray-600"
        >
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {{ !$route.params.id ? "Add new" : "Edit" }} application
          </h3>
          <button
            type="button"
            class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-600 dark:hover:text-white"
            data-modal-toggle="defaultModal"
          >
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
        <!-- Modal body -->
        <form action="#">
          <div class="grid gap-4 mb-4 sm:grid-cols-2">
            <div>
              <label
                for="name"
                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                >Name</label
              >
              <input
                type="text"
                name="name"
                id="name"
                class="bg-[#160b3b] border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-cyan-500 dark:focus:border-cyan-500"
                placeholder="Type product name"
                required=""
              />
            </div>
            <div>
              <label
                for="brand"
                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                >Brand</label
              >
              <input
                type="text"
                name="brand"
                id="brand"
                class="bg-[#160b3b] border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-cyan-500 dark:focus:border-cyan-500"
                placeholder="Product brand"
                required=""
              />
            </div>

            <div class="sm:col-span-2">
              <label
                for="description"
                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                >Description</label
              >
              <textarea
                id="description"
                rows="4"
                class="block p-2.5 w-full text-sm text-gray-900 bg-[#160b3b] rounded-lg border border-gray-300 focus:ring-cyan-500 focus:border-cyan-500 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-cyan-500 dark:focus:border-cyan-500"
                placeholder="Write product description here"
              ></textarea>
            </div>
          </div>
          <button
            type="submit"
            class="text-white inline-flex items-center bg-cyan-700 hover:bg-cyan-800 focus:ring-4 focus:outline-none focus:ring-cyan-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-cyan-600 dark:hover:bg-cyan-700 dark:focus:ring-cyan-800"
          >
            <svg
              class="mr-1 -ml-1 w-6 h-6"
              fill="currentColor"
              viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                fill-rule="evenodd"
                d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z"
                clip-rule="evenodd"
              ></path>
            </svg>
            Add new product
          </button>
        </form>
      </div>
    </div>

    <div class="grid grid-cols-1 gap-4">
      <!-- Add form -->
      <!-- <form
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
      </form> -->

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
