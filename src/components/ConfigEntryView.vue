<template>
  <div class="card shadow-lg compact bg-base-100">
    <div class="card-body">
      <h3 class="card-title">{{ entry.path }}</h3>
      <p>Type: {{ entry.type }}</p>
      <p>Description: {{ entry.description }}</p>
      <div v-if="showContent" class="mt-2">
        <pre class="bg-gray-100 p-2 rounded">{{ content }}</pre>
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
        <button class="btn btn-error" @click="removeEntry">Remove</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { ConfigEntry } from '../types';
import { testFilePath, readConfigFile } from '../utils/fileUtils';

const props = defineProps<{ entry: ConfigEntry }>();
const emits = defineEmits(['remove']);

const showContent = ref(false);
const content = ref('');
const testResult = ref<boolean | null>(null);

async function toggleContent() {
  showContent.value = !showContent.value;
  if (showContent.value && !content.value) {
    try {
      content.value = await readConfigFile(props.entry.path, props.entry.type);
    } catch (error) {
      console.error('Error reading file content:', error);
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
});

function removeEntry() {
  emits('remove', props.entry.id);
}
</script>