<template>
<div>
  <input type="checkbox" id="config-entry-modal" class="modal-toggle" />
  <div class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">{{ entry ? 'Edit' : 'Add' }} Config Entry</h3>
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
      <div class="form-control">
        <label class="label">
          <span class="label-text">Key</span>
        </label>
        <input type="text" v-model="key" class="input input-bordered" required />
      </div>
      <div class="modal-action">
        <button type="submit" class="btn btn-primary">{{ entry ? 'Save' : 'Add' }}</button>
        <label for="config-entry-modal" class="btn">Cancel</label>
      </div>
      </form>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { ConfigEntry } from '../types';
import { open } from '@tauri-apps/plugin-dialog';

const props = defineProps<{ entry: ConfigEntry | null }>();
const emit = defineEmits<{
  (e: 'add', entry: ConfigEntry): void;
  (e: 'update', entry: ConfigEntry): void;
}>();

const path = ref('');
const type = ref<'json' | 'yaml'>('json');
const description = ref('');
const key = ref('');

watch(() => props.entry, (newEntry) => {
  console.log('New entry:', newEntry);
  if (newEntry) {
    path.value = newEntry.path;
    type.value = newEntry.type;
    description.value = newEntry.description;
    key.value = newEntry.key;
  } else {
    path.value = '';
    type.value = 'json';
    description.value = '';
    key.value = '';
  }
});

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
    id: props.entry?.id || Date.now().toString(),
    path: path.value,
    type: type.value,
    description: description.value,
    key: key.value,
    content: {},
  };
  if (props.entry) {
    emit('update', newEntry);
  } else {
    emit('add', newEntry);
  }
  // 关闭模态框
  document.getElementById('config-entry-modal')?.click();
}
</script>