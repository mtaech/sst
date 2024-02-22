import {createRouter, createWebHistory} from "vue-router";
import Home from "../views/Home.vue";

const routes = [
    {
        path: "/",
        name: "Home",
        component: Home
    }
]


const routers = createRouter({
    history: createWebHistory(),
    routes: routes,
})

export default routers;