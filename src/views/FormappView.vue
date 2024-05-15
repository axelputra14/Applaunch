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
        <!-- Add form -->
        <form
          autocomplete="off"
          v-on:submit.prevent="addappHandler"
          v-if="!$route.params.id"
        >
          <div class="grid gap-4 mb-4 grid-cols-2">
            <div>
              <label
                for="title"
                class="block mb-2 text-sm font-medium text-white"
                >Title</label
              >
              <input
                type="text"
                name="title"
                id="title"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-red-800 ring-red-800 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-red-800 focus:border-red-800"
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
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-orange-500 ring-orange-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-orange-500 focus:border-orange-500"
                placeholder="Developer"
                v-model="formData.developer"
              />
            </div>

            <div>
              <label
                for="publisher"
                class="block mb-2 text-sm font-medium text-white"
                >Publisher</label
              >
              <input
                type="text"
                name="publisher"
                id="publisher"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-yellow-400 ring-yellow-400 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-yellow-400 focus:border-yellow-400"
                placeholder="Publisher"
                v-model="formData.publisher"
              />
            </div>

            <div>
              <label
                for="exedir"
                class="block mb-2 text-sm font-medium text-white"
                >Executable Directory</label
              >
              <input
                type="text"
                name="exedir"
                id="exedir"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-green-500 ring-green-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-green-500 focus:border-green-500"
                placeholder="Absolute directory of the executable"
                v-model="formData.exeDir"
              />
            </div>

            <div>
              <label
                for="imgdir"
                class="block mb-2 text-sm font-medium text-white"
                >Cover Image</label
              >
              <input
                type="text"
                name="imgdir"
                id="imgdir"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-cyan-500 ring-cyan-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-cyan-500 focus:border-cyan-500"
                placeholder="Image file name for the cover"
                v-model="formData.imgDir"
              />
            </div>

            <div>
              <label
                for="bgdir"
                class="block mb-2 text-sm font-medium text-white"
                >Background Image</label
              >
              <input
                type="text"
                name="bgdir"
                id="bgdir"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-blue-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-blue-500 focus:border-blue-500"
                placeholder="Image file name for the background in details"
                v-model="formData.bgDir"
              />
            </div>

            <div class="col-span-2">
              <label
                for="description"
                class="block mb-2 text-sm font-medium text-white"
                >Description</label
              >
              <textarea
                id="description"
                rows="8"
                class="block p-2.5 w-full text-sm bg-[#160b3b] rounded-lg border border-indigo-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-indigo-500 focus:border-indigo-500"
                placeholder="Write product description here"
                v-model="formData.desc"
              ></textarea>
            </div>

            <div>
              <label
                for="lang"
                class="block mb-2 text-sm font-medium text-white"
                >Language</label
              >
              <input
                type="text"
                name="lang"
                id="lang"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-purple-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-purple-500 focus:border-purple-500"
                placeholder="Main language of the application"
                v-model="formData.lang"
              />
            </div>
            <div>
              <label
                for="reldate"
                class="block mb-2 text-sm font-medium text-white"
                >Release date</label
              >
              <input
                type="text"
                name="reldate"
                id="reldate"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-pink-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-pink-500 focus:border-pink-500"
                placeholder="Release date in YYYY-MM-DD"
                v-model="formData.relDate"
              />
            </div>
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
          autocomplete="off"
          v-on:submit.prevent="editappHandler"
          v-if="$route.params.id"
        >
          <div class="grid gap-4 mb-4 grid-cols-2">
            <div>
              <label
                for="title"
                class="block mb-2 text-sm font-medium text-white"
                >Title</label
              >
              <input
                type="text"
                name="title"
                id="title"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-red-800 ring-red-800 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-red-800 focus:border-red-800"
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
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-orange-500 ring-orange-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-orange-500 focus:border-orange-500"
                placeholder="Developer"
                v-model="formData.developer"
              />
            </div>

            <div>
              <label
                for="publisher"
                class="block mb-2 text-sm font-medium text-white"
                >Publisher</label
              >
              <input
                type="text"
                name="publisher"
                id="publisher"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-yellow-400 ring-yellow-400 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-yellow-400 focus:border-yellow-400"
                placeholder="Publisher"
                v-model="formData.publisher"
              />
            </div>

            <div>
              <label
                for="exedir"
                class="block mb-2 text-sm font-medium text-white"
                >Executable Directory</label
              >
              <input
                type="text"
                name="exedir"
                id="exedir"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-green-500 ring-green-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-green-500 focus:border-green-500"
                placeholder="Absolute directory of the executable"
                v-model="formData.exeDir"
              />
            </div>

            <div>
              <label
                for="imgdir"
                class="block mb-2 text-sm font-medium text-white"
                >Cover Image</label
              >
              <input
                type="text"
                name="imgdir"
                id="imgdir"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-cyan-500 ring-cyan-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-cyan-500 focus:border-cyan-500"
                placeholder="Image file name for the cover"
                v-model="formData.imgDir"
              />
            </div>

            <div>
              <label
                for="bgdir"
                class="block mb-2 text-sm font-medium text-white"
                >Background Image</label
              >
              <input
                type="text"
                name="bgdir"
                id="bgdir"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-blue-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-blue-500 focus:border-blue-500"
                placeholder="Image file name for the background in details"
                v-model="formData.bgDir"
              />
            </div>

            <div class="col-span-2">
              <label
                for="description"
                class="block mb-2 text-sm font-medium text-white"
                >Description</label
              >
              <textarea
                id="description"
                rows="8"
                class="block p-2.5 w-full text-sm bg-[#160b3b] rounded-lg border border-indigo-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-indigo-500 focus:border-indigo-500"
                placeholder="Write product description here"
                v-model="formData.desc"
              ></textarea>
            </div>

            <div>
              <label
                for="lang"
                class="block mb-2 text-sm font-medium text-white"
                >Language</label
              >
              <input
                type="text"
                name="lang"
                id="lang"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-purple-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-purple-500 focus:border-purple-500"
                placeholder="Main language of the application"
                v-model="formData.lang"
              />
            </div>
            <div>
              <label
                for="reldate"
                class="block mb-2 text-sm font-medium text-white"
                >Release date</label
              >
              <input
                type="text"
                name="reldate"
                id="reldate"
                class="bg-[#160b3b] border text-sm rounded-lg block w-full p-2.5 border-pink-500 placeholder-gray-400 text-white focus:bg-gray-900 focus:ring-pink-500 focus:border-pink-500"
                placeholder="Release date in YYYY-MM-DD"
                v-model="formData.relDate"
              />
            </div>
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
    </div>

    <BottomNav />
  </div>
</template>
