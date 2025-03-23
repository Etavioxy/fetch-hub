<template>
  <div class="card shadow-lg compact bg-base-100">
    <div class="card-body">
      <h3 class="card-title">{{ entry.path }}</h3>
      <p>Type: {{ entry.type }}</p>
      <p>Description: {{ entry.description }}</p>
      <p>KEY: {{ entry.key }}</p>
      <div v-if="testResult !== null" class="flex">
        <p :class="testResult ? 'green' : 'red'">
          {{ testResult ? 'Path exists' : 'Path does not exist' }}
        </p>
      </div>
      <CodePreview
        v-if="testResult"
        :path="entry.path"
        :type="entry.type"
        :keyprop="entry.key"
      />
      <div class="card-actions justify-end">
        <button class="btn btn-warning" @click="editEntry">Edit</button>
        <button class="btn btn-error" @click="removeEntry">Remove</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import CodePreview from './CodePreview.vue';

import { ref, onMounted } from 'vue';
import { ConfigEntry } from '../types';
import { testFilePath } from '../utils/fileUtils';

const props = defineProps<{ entry: ConfigEntry }>();
const emits = defineEmits(['remove', 'edit']);

const testResult = ref<boolean | null>(null);

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

function editEntry() {
  emits('edit');
}
</script>