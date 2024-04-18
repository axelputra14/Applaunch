<script>
export default {
  name: "ModalDelete",
  methods: {
    close() {
      this.$emit("close");
    },
  },
};
</script>

<template>
  <main class="modal-container">
    <div class="modal">
      <div class="modal-header">
        <slot name="header"></slot>
      </div>
      <div class="modal-content">
        <slot name="content"></slot>
      </div>
      <div class="modal-footer">
        <button v-on:click="buttonClicked()">Yes</button>
        <button v-on:click="close">No</button>
      </div>
    </div>
  </main>
</template>
