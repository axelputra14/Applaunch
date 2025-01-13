import { defineStore } from "pinia";

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
        const response = await fetch("http://localhost:16850/app", {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        });

        let data = await response.json();

        this.applists = data.rows;
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = "Failed getting app list";
      }
    },
    async fetchAppById(id) {
      try {
        const response = await fetch(`http://localhost:16850/app/${id}`, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        });

        let data = await response.json();

        this.applistbyid = data.rows;
      } catch (err) {
        this.showAlert = true;
        this.alertMessage = `Failed getting app`;
      }
    },
    async addApplication(addData) {
      try {
        await fetch("http://localhost:16850/app/add", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            title: addData.title,
            developer: addData.developer,
            exeDir: addData.exeDir,
            imgDir: addData.imgDir,
          }),
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
        await fetch(`http://localhost:16850/app/edit/${editData.id}`, {
          method: "PATCH",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            title: editData.title,
            developer: editData.developer,
            exeDir: editData.exeDir,
            imgDir: editData.imgDir,
          }),
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
        await fetch(`http://localhost:16850/app/${id}`, {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json",
          },
        });

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
