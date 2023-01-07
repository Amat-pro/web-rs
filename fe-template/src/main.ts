import { createApp } from 'vue'
import App from '@App.vue'
import router from "./router";
import ElementPlus from 'element-plus'
import { ElMessage } from 'element-plus'
import 'element-plus/dist/index.css'

const app = createApp(App)

app.config.errorHandler = (err, _instance, _info) => {
    // handle error, e.g. report to a service
    console.log("==> error handler: ", err);
};
app.config.warnHandler = (msg, _instance, _trace) => {
    // `trace` is the component hierarchy trace
    console.log("==> warn handler: ", msg);
}
app.config.globalProperties.$message = ElMessage;

app.use(ElementPlus)
app.use(router)
app.mount('#app');
