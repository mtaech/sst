<template>
  <div class="server-container">
    <ul class="server-list" v-if="profile?.sub">
      <li :class="{
        'server-item':true,
        'is-active':server.name === activeName
      }" v-for="server in profile?.sub?.servers" @click="exe_server(server)">
        <span >{{server.name}}</span>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Profile, Resp, Server} from "src/models/index.ts";

const profile = ref<Profile>();
const activeName = ref('')
function getList() {
  invoke<Resp<Profile[]>>("get_profile_list").then(resp=>{
    console.log("log",resp)
    if (resp && resp.data) {
      profile.value  = resp.data[0]
    }
  })
}
// function addProfile() {
//   url.value = "https://api.fanss.vip/link/hVnsnzkzql24t9nA?list=shadowrocket";
//   if (url.value) {
//     invoke("add_profile",{url:url.value}).then(resp=>{
//       console.log('add pro',resp)
//       if (resp && resp.data) {
//         profile.value  = resp.data[0]
//       }
//     })
//   }
// }
function exe_server(server:Server) {
  activeName.value = server.name
  invoke("exe_server",{server:server}).then(resp=>{
    console.log('resp',resp)
  })

}
getList();
</script>

<style scoped>
.server-list{
  --uno: flex flex-wrap justify-around;
}
.server-item{
  cursor: pointer;
  width: 30%;
  border:1px solid darkgray;
  --uno: rounded mb-2 py-4 bg-zinc-200 shadow-lg mb-2
}
.is-active{
  --uno:  color-blue-600
}
</style>