<script setup lang="ts">
import { ref } from "vue";
import { readFile } from "./utils/fileUtils";

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
  <form class="row" @submit.prevent="readSubmit">
    <input id="read-input" v-model="path" placeholder="Enter a file path..." />
    <button type="submit">Read</button>
  </form>
  <p>{{ readMsg }}</p>
</template>