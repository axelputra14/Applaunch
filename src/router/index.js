import { createRouter, createWebHistory } from "vue-router";
import ApplistView from "../views/ApplistView.vue";
import FormappView from "../views/FormappView.vue";
import AppDetail from "../views/AppDetail.vue";

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
      component: FormappView,
    },
    {
      path: "/app-edit/:id",
      name: "editapppage",
      component: FormappView,
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
