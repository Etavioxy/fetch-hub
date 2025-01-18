<template>
  <div class="card shadow-lg compact bg-base-100">
    <div class="card-body">
      <h3 class="card-title">{{ entry.path }}</h3>
      <p>Type: {{ entry.type }}</p>
      <p>Description: {{ entry.description }}</p>
      {{ "KEY: " + entry.key }}
      <div v-if="showContent" class="mt-2">
        <pre class="bg-gray-800 p-2 rounded max-h-48 overflow-auto whitespace-pre text-left">{{ content }}</pre>
      </div>
      <div class="card-actions justify-end">
        <div v-if="testResult !== null" class="mt-2">
          <p class="text-sm" :class="testResult ? 'text-green-500' : 'text-red-500'">
            {{ testResult ? 'Path exists' : 'Path does not exist' }}
          </p>
        </div>
        <button v-if="testResult" class="btn btn-secondary" @click="toggleContent">
          {{ showContent ? 'Hide' : 'Show' }} Content
        </button>
        <button class="btn btn-warning" @click="editEntry">Edit</button>
        <button class="btn btn-error" @click="removeEntry">Remove</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { ConfigEntry } from '../types';
import { invoke } from "@tauri-apps/api/core";
import { testFilePath, readTextFileObjectCached } from '../utils/fileUtils';

const props = defineProps<{ entry: ConfigEntry }>();
const emits = defineEmits(['remove', 'edit']);

async function human_json_stringify(json: string): Promise<string> {
  return await invoke('human_read', { serialized: JSON.stringify(json), indent: 4, level: 1, linewidth: 90});
}

const showContent = ref(false);
const content = ref('');
const testResult = ref<boolean | null>(null);

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
    const fileContent = await readTextFileObjectCached(props.entry.path, props.entry.type);
    if (fileContent) {
      content.value = await human_json_stringify(getObjectByKey(fileContent, props.entry.key));
    }
  }
}

async function testPath() {
  try {
    testResult.value = await testFilePath(props.entry.path);
  } catch (error) {
    console.error('Error testing file path:', error);
    testResult.value = false;
  }
}

onMounted(() => {
  testPath();
  toggleContent();
});

function removeEntry() {
  emits('remove', props.entry.id);
}

function editEntry() {
  emits('edit');
}
</script>