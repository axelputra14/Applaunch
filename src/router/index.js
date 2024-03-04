import { createRouter, createWebHistory } from "vue-router";
import ApplistView from "../views/ApplistView.vue";
import AddappView from "../views/AddappView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/:pathMatch(.*)*",
      name: "NotFound",
      component: ApplistView,
    },
    {
      path: "/",
      name: "applistpage",
      component: ApplistView,
    },
    {
      path: "/app-add",
      name: "addapppage",
      component: AddappView,
    },
  ],
});

router.beforeEach((to, from, next) => {
  if (to.name == "NotFound") {
    next("/");
  } else {
    next();
  }
});

export default router;
