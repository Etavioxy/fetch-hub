<template>
  <div class="card shadow-lg compact bg-base-100">
    <div class="card-body">
      <h3 class="card-title">{{ entry.path }}</h3>
      <p>Type: {{ entry.type }}</p>
      <p>Description: {{ entry.description }}</p>
      <pre class="bg-gray-100 p-2 rounded">{{ entry.content }}</pre>
      <div class="card-actions justify-end">
        <button class="btn btn-error" @click="removeEntry">Remove</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ConfigEntry } from '../types';

const props = defineProps<{ entry: ConfigEntry }>();
const emits = defineEmits(['remove']);

function removeEntry() {
  emits('remove', props.entry.id);
}
</script>