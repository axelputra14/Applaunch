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
        this.alertMessage = "Failed getting app list";
      }
    },
    async fetchAppById(id) {
      try {
        const response = await applist.get(`/app/${id}`);

        this.applistbyid = response.data.result;
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = `Failed getting app`;
      }
    },
    async addApplication(addData) {
      try {
        await applist.post("/app/add", {
          title: addData.title,
          developer: addData.developer,
          exeDir: addData.exeDir,
          imgDir: addData.imgDir,
        });
        router.push({ name: "applistpage" });
        this.showAlert = true;
        this.alertMessage = "New application has been added";
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = "Error in adding data, check some fields";
      }
    },
    async editApplication(editData) {
      try {
        await applist.patch(`/app/edit/${editData.id}`, {
          title: editData.title,
          developer: editData.developer,
          exeDir: editData.exeDir,
          imgDir: editData.imgDir,
        });
        router.push({ name: "applistpage" });
        this.showAlert = true;
        this.alertMessage = "Selected application has been edited";
      } catch (err) {
        this.showAlert = true;
        this.alertMessage =
          "Failed editing data of application, check some fields";
      }
    },
    async deleteApplication(id) {
      try {
        await applist.delete(`/app/${id}`);

        this.showAlert = true;
        this.alertMessage = "Selected application has been deleted";

        router.push({ name: "applistpage" });
        router.go(0);
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = "Failed deleting app";
      }
    },
    async closeAlert() {
      this.showAlert = false;
      this.alertMessage = "";
    },
  },
});
