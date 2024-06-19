<script>
import { InformationCircleIcon, XMarkIcon } from "@heroicons/vue/24/solid";

export default {
  name: "AlertInfo",
  props: ["alertContent"],
  components: {
    InformationCircleIcon,
    XMarkIcon,
  },
  data() {
    return {
      countdown: 5,
    };
  },
  methods: {
    close() {
      this.$emit("closeAlert");
    },
    startCountdown() {
      const interval = setInterval(() => {
        this.countdown -= 1;
        if (this.countdown <= 0) {
          clearInterval(interval);
          this.close();
        }
      }, 1000);
    },
  },
  mounted() {
    this.startCountdown();
  },
};
</script>

<template>
  <div
    class="mt-5 flex items-center w-1/2 p-4 mb-4 text-sm border border-t-4 rounded-lg bg-gray-800 text-blue-400 border-blue-800 fixed top-5 z-50 bg-opacity-75"
  >
    <InformationCircleIcon
      class="flex-shrink-0 inline w-6 h-6 me-3 text-blue-400"
    />
    <div>
      <span class="font-medium">{{ alertContent }}</span>
    </div>

    <span class="ms-auto font-bold">{{ countdown }}</span>

    <button
      type="button"
      class="ms-auto -mx-1.5 -my-1.5 rounded-lg focus:ring-2 focus:ring-blue-400 p-1.5 inline-flex items-center justify-center h-8 w-8 bg-gray-800 text-blue-400 hover:bg-gray-700"
      aria-label="Close"
      v-on:click="close()"
    >
      <XMarkIcon class="w-12 h-12" />
    </button>
  </div>
</template>
