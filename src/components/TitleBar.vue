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
    console.log(this.maximized);
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
    class="titlebar flex flex-row justify-end select-none gap-x-3 pr-4 pt-2 h-8 bg-transparent fixed right-0 pl-[1600px] z-50"
    id="titlebar"
  >
    <div
      @click="minimize"
      class="titlebar-button inline-flex justify-center items-center hover:cursor-pointer"
      id="titlebar-minimize"
    >
      <MinusIcon
        class="text-emerald-600 w-5 h-5 basis-10 stroke-2 stroke-emerald-600 drop-shadow-lg"
      ></MinusIcon>
    </div>
    <div
      v-if="maximized"
      @click="toggleMaximize"
      class="titlebar-button inline-flex justify-center items-center hover:cursor-pointer"
      id="titlebar-maximize"
    >
      <Square2StackIcon
        class="text-amber-500 w-5 h-5 basis-10 drop-shadow-lg"
      ></Square2StackIcon>
    </div>
    <div
      v-else
      @click="toggleMaximize"
      class="titlebar-button inline-flex justify-center items-center hover:cursor-pointer"
      id="titlebar-restore"
    >
      <WindowIcon
        class="text-amber-500 w-5 h-5 basis-10 drop-shadow-lg"
      ></WindowIcon>
    </div>
    <div
      @click="close"
      class="titlebar-button inline-flex justify-center items-center hover:cursor-pointer"
      id="titlebar-close"
    >
      <XMarkIcon
        class="text-red w-5 h-5 basis-10 stroke-2 stroke-red-500 drop-shadow-lg"
      ></XMarkIcon>
    </div>
  </div>
</template>

<style scoped></style>
