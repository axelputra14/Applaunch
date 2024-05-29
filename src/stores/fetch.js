import { ref, computed } from "vue";
import { defineStore } from "pinia";
import applist from "../apis/applist";

import router from "../router";

export const useFetchStore = defineStore({
  id: "fetch",
  state() {
    return {
      applists: [],
      applistbyid: {},
      showAlert: false,
      alertMessage: "",
    };
  },
  actions: {
    async fetchApplists() {
      try {
        const response = await applist.get("/app");

        this.applists = response.data.rows;
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = err.response.data.message;
      }
    },
    async fetchAppById(id) {
      try {
        const response = await applist.get(`/app/${id}`);

        this.applistbyid = response.data.result;
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = err.response.data.message;
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
        this.showAlert = true;
        this.alertMessage = "Selected application has been added";
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = err.response.data.message;
      }
    },
    async editApplication(editData) {
      try {
        const response = await applist.patch(`/app/edit/${editData.id}`, {
          title: editData.title,
          developer: editData.developer,
          publisher: editData.publisher,
          exeDir: editData.exeDir,
          imgDir: editData.imgDir,
          bgDir: editData.bgDir,
          desc: editData.desc,
          lang: editData.lang,
          relDate: editData.relDate,
        });
        router.push({ name: "applistpage" });
        this.showAlert = true;
        this.alertMessage = "Selected application has been edited";
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = err.response.data.message;
      }
    },
    async deleteApplication(id) {
      try {
        await applist.delete(`/app/${id}`);

        router.go(0);

        this.showAlert = true;
        this.alertMessage = "Selected application has been deleted";
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = err.response.data.message;
      }
    },
    closeAlert() {
      this.showAlert = false;
      this.alertMessage = "";
    },
  },
});
