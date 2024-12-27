<script setup lang="ts">
import { ref } from "vue";
import { readFile } from "./utils/fileUtils";

import Board from "./components/Board.vue";

// 检查 `$APPDATA/avatar.png` 文件是否存在
//await exists('avatar.png', { baseDir: BaseDirectory.AppData });

const path = ref("");
const readMsg = ref("");

path.value = 'C:\\Users\\z7z_Eta\\AppData\\Roaming\\Rime\\';
//path.value = 'N:\\.vimrc';

async function readSubmit(){
  readMsg.value = await readFile(path.value) as string;
}
</script>

<template>
  <main class="container">
    <Board></Board>
    
    <form class="row" @submit.prevent="readSubmit">
      <input id="read-input" v-model="path" placeholder="Enter a file path..." />
      <button type="submit">Read</button>
    </form>
    <p>{{ readMsg }}</p>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}
</style>