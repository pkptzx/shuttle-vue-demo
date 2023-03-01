import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/speechrecognition",
      name: "SpeechRecognition",
      meta: {
        title: '语音识别'
      },
      component: () => import("../views/SpeechRecognition.vue"),
    },
    {
      path: "/",
      name: "qr",
      meta: {
        title: '二维码解析'
      },
      component: () => import("../views/QRDecoder.vue"),
    },
    {
      path: "/canvasdrawgif",
      name: "canvasdrawgif",
      meta: {
        title: '绘制gif到canvas上'
      },
      component: () => import("../views/CanvasDrawGif.vue"),
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
