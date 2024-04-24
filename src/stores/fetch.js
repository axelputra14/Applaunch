import { ref, computed } from "vue";
import { defineStore } from "pinia";
import applist from "../apis/applist";

import router from "../router";

export const useFetchStore = defineStore({
  id: "fetch",
  state() {
    return {
      tesvar: "Hello dunia",
      applists: [],
      applistbyid: {},
    };
  },
  actions: {
    async fetchApplists() {
      try {
        const response = await applist.get("/app");

        this.applists = response.data.rows;
      } catch (err) {
        console.log("Gagal");
        console.log(err);
      }
    },
    async fetchAppById(id) {
      try {
        const response = await applist.get(`/app/${id}`);

        this.applistbyid = response.data.result;
      } catch (err) {
        console.log("Gagal");
        console.log(err);
      }
    },
    async addApplication(addData) {
      try {
        const response = await applist.post("/app/add", {
          title: addData.title,
          developer: addData.developer,
          publisher: addData.publisher,
          exeDir: addData.exeDir,
          imgDir: addData.imgDir,
          bgDir: addData.bgDir,
          desc: addData.desc,
          lang: addData.lang,
          relDate: addData.relDate,
        });
        router.push({ name: "applistpage" });
        // swal fire something
      } catch (err) {
        console.log(err.response.data.message);
      }
    },
    async deleteApplication(id) {
      try {
        await applist.delete(`/app/${id}`);
        // router.push({ name: "applistpage" });
        router.go(0);
        // swal fire something
      } catch (err) {
        console.log(err.response.data.message);
      }
    },
  },
});
