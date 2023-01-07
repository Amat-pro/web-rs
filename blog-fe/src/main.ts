import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";

import ElementPlus from "element-plus";
import { ElMessage } from "element-plus";
import "element-plus/dist/index.css";

import "./assets/main.css";

const app = createApp(App);

// eslint-disable-next-line @typescript-eslint/no-unused-vars
app.config.errorHandler = (err, _instance, _info) => {
  // handle error, e.g. report to a service
  console.log("==> error handler: ", err);
};
// eslint-disable-next-line @typescript-eslint/no-unused-vars
app.config.warnHandler = (msg, _instance, _trace) => {
  // `trace` is the component hierarchy trace
  console.log("==> warn handler: ", msg);
};
app.config.globalProperties.$message = ElMessage;

app.use(createPinia());
app.use(router);

app.use(ElementPlus);

app.mount("#app");
