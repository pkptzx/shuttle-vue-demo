import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "SpeechRecognition",
      meta: {
        title: '语音识别'
      },
      component: () => import("../views/SpeechRecognition.vue"),
    },
    {
      path: "/qr",
      name: "qr",
      meta: {
        title: '二维码解析'
      },
      component: () => import("../views/QRDecoder.vue"),
    },
  ],
});
router.beforeEach((to, from, next) => {
  if (to.meta.title) {
      document.title = to.meta.title as string;
  }
  next();
});
export default router;
