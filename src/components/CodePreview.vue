<script setup lang="ts">
import { ref } from 'vue';
import { readTextFileObjectCached } from '../utils/fileUtils';
import { invoke } from '@tauri-apps/api/core';

// 定义从父组件接收的属性
const props = defineProps<{
  path: string;
  type: 'json' | 'yaml';
  keyprop: string;
}>();

const showContent = ref(false);
const content = ref('');

function getObjectByKey(obj: any, key: string): any {
  return key.split('.').reduce((o, k) => {
    if (k.includes('[')) {
      const [prop, index] = k.split(/\[|\]/).filter(Boolean);
      return o[prop][parseInt(index, 10)];
    }
    return o[k];
  }, obj);
}

async function toggleContent() {
  showContent.value = !showContent.value;
  if (showContent.value && !content.value) {
    const fileObject = await readTextFileObjectCached(props.path, props.type);
    if (fileObject) {
      content.value = await human_json_stringify(getObjectByKey(fileObject, props.keyprop));
    }
  }
}

async function human_json_stringify(json: any): Promise<string> {
  return await invoke('human_read', {
    serialized: JSON.stringify(json),
    indent: 4,
    level: 1,
    linewidth: 90,
  });
}

toggleContent();
</script>

<template>
  <button class="btn btn-secondary" @click="toggleContent">
    {{ showContent ? 'Hide' : 'Show' }} Content
  </button>
  <section v-if="showContent">
    <pre class="overflow-auto">{{ content }}</pre>
  </section>
</template>

<style>
overflow-auto {
  overflow: auto;
}
</style>