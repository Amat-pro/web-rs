import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "app",
      component: () => import("../App.vue"),
    },
    {
      path: "/blog-home",
      name: "blog-home",
      component: () => import("../views/BlogHome.vue"),
    },
    {
      path: "/login",
      name: "login",
      component: () => import("../views/LoginView.vue"),
    },
    {
      path: "/register",
      name: "register",
      component: () => import("../views/RegisterView.vue"),
    },
    {
      path: "/article",
      name: "article",
      component: () => import("../views/ArticleView.vue"),
    },
  ],
});

export default router;
