<script>
export default {
  name: "ModalDelete",
  props: ["applicationid"],
  methods: {
    close() {
      this.$emit("close");
    },
    deleteApp() {
      this.$emit("deleteApp", this.applicationid);
    },
  },
};
</script>

<template>
  <main class="modal-container z-50 bg-white opacity-25">
    <div class="modal">
      <div class="modal-header">
        <slot name="header"></slot>
      </div>
      <div class="modal-content">
        <slot name="content"></slot>
      </div>
      <div class="modal-footer">
        <button v-on:click="deleteApp(this.applicationid)">Yes</button>
        <button v-on:click="close()">No</button>
      </div>
    </div>
  </main>
</template>
