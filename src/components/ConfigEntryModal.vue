<template>
<div>
  <input type="checkbox" id="config-entry-modal" class="modal-toggle" />
  <div class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">Add Config Entry</h3>
    <form @submit.prevent="submitForm">
    <div class="form-control">
      <label class="label">
      <span class="label-text">Path</span>
      </label>
      <div class="flex">
        <input type="text" v-model="path" class="input input-bordered flex-grow" required />
        <button type="button" class="btn btn-secondary ml-2" @click="selectFile">Browse</button>
      </div>
    </div>
    <div class="form-control">
      <label class="label">
      <span class="label-text">Type</span>
      </label>
      <select v-model="type" class="select select-bordered" required>
      <option value="json">JSON</option>
      <option value="yaml">YAML</option>
      </select>
    </div>
    <div class="form-control">
      <label class="label">
      <span class="label-text">Description</span>
      </label>
      <input type="text" v-model="description" class="input input-bordered" required />
    </div>
    <div class="modal-action">
      <button type="submit" class="btn btn-primary">Add</button>
      <label for="config-entry-modal" class="btn">Cancel</label>
    </div>
    </form>
  </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { ConfigEntry } from '../types';
import { open } from '@tauri-apps/plugin-dialog';

const path = ref('');
const type = ref<'json' | 'yaml'>('json');
const description = ref('');

const emit = defineEmits<{
(e: 'add', entry: ConfigEntry): void;
}>();
  
async function selectFile() {
  const selected = await open({
    multiple: false,
    filters: [
      { name: 'Config Files', extensions: ['json', 'yaml', 'yml'] },
    ],
  });
  if (selected && typeof selected === 'string') {
    path.value = selected;
  }
}

function submitForm() {
  const newEntry: ConfigEntry = {
    id: Date.now().toString(),
    path: path.value,
    type: type.value,
    description: description.value,
    content: {},
  };
  emit('add', newEntry);
  // Reset form
  path.value = '';
  type.value = 'json';
  description.value = '';
}
</script>