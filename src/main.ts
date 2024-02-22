import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import routers from "./routers";
import 'virtual:uno.css'


createApp(App)
    .use(routers)
    .mount("#app");
