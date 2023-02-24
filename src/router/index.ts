import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "SpeechRecognition",
      component: () => import("../views/SpeechRecognition.vue"),
    },
    {
      path: "/qr",
      name: "qr",
      component: () => import("../views/QRDecoder.vue"),
    },
  ],
});

export default router;
