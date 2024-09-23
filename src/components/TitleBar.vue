<script>
import { appWindow } from "@tauri-apps/api/window";
import {
  MinusIcon,
  Square2StackIcon,
  WindowIcon,
  XMarkIcon,
} from "@heroicons/vue/24/solid";
export default {
  data() {
    return {
      maximized: false,
    };
  },
  created() {
    window.addEventListener("resize", this.resizeEventHandler);
  },
  destroyed() {
    window.removeEventListener("resize", this.resizeEventHandler);
  },
  components: {
    MinusIcon,
    Square2StackIcon,
    WindowIcon,
    XMarkIcon,
  },
  methods: {
    minimize() {
      appWindow.minimize();
    },
    toggleMaximize() {
      appWindow.isMaximized().then((result) => {
        result ? appWindow.unmaximize() : appWindow.maximize();
        this.maximized = result;
      });
    },
    close() {
      appWindow.close();
    },
    resizeEventHandler() {
      appWindow.isMaximized().then((result) => {
        this.maximized = result;
      });
    },
  },
};
</script>

<template>
  <div
    @dblclick="toggleMaximize"
    data-tauri-drag-region
    class="titlebar flex flex-row h-6"
    id="titlebar"
  >
    <div class="py-[2px] left-4 select-none gap-x-3 mt-2 pt-2 h-8 fixed z-50">
      <p class="font-normal text-white">Applaunch</p>
    </div>
    <div
      class="bg-slate-800/50 rounded-2xl py-[2px] justify-end select-none gap-x-3 mt-2 pt-2 h-8 fixed right-4 z-50"
    >
      <div
        @click="minimize"
        class="titlebar-button inline-flex justify-center items-center hover:cursor-pointer mx-2"
        id="titlebar-minimize"
      >
        <MinusIcon
          class="text-white w-5 h-5 basis-10 stroke-2 stroke-white drop-shadow-lg"
        ></MinusIcon>
      </div>
      <div
        v-if="maximized"
        @click="toggleMaximize"
        class="titlebar-button inline-flex justify-center items-center hover:cursor-pointer mx-2"
        id="titlebar-maximize"
      >
        <Square2StackIcon
          class="text-white w-5 h-5 basis-10 drop-shadow-lg"
        ></Square2StackIcon>
      </div>
      <div
        v-else
        @click="toggleMaximize"
        class="titlebar-button inline-flex justify-center items-center hover:cursor-pointer mx-2"
        id="titlebar-restore"
      >
        <WindowIcon
          class="text-white w-5 h-5 basis-10 drop-shadow-lg"
        ></WindowIcon>
      </div>
      <div
        @click="close"
        class="titlebar-button inline-flex justify-center items-center hover:cursor-pointer mx-2"
        id="titlebar-close"
      >
        <XMarkIcon
          class="text-red w-5 h-5 basis-10 stroke-2 stroke-white drop-shadow-lg"
        ></XMarkIcon>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
