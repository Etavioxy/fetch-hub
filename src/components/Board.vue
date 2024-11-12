<template>
  <main class="container mx-auto p-4">
    <h1 class="text-2xl font-bold mb-4">Configuration Manager</h1>
    <div v-for="group in configGroups" :key="group.id" class="config-group mb-6">
      <h2 class="text-xl font-semibold mb-2">{{ group.name }}</h2>
      <ul class="list-disc pl-5">
        <li v-for="entry in group.entries" :key="entry.id" class="mb-2">
          <ConfigEntryView :entry="entry" @remove="removeEntry(group.id, $event)" />
        </li>
      </ul>
      <button class="btn btn-primary mt-2" @click="addEntry(group.id)">Add Entry</button>
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useConfigStore } from '../configStore';
import { ConfigEntry } from '../types';
import ConfigEntryView from './ConfigEntryView.vue';

const { configGroups, addConfigEntry, removeConfigEntry } = useConfigStore();

function addEntry(groupId: string) {
  const newEntry: ConfigEntry = {
    id: Date.now().toString(),
    path: 'path/to/config.yaml',
    type: 'yaml',
    description: 'New config entry',
    content: {},
  };
  addConfigEntry(groupId, newEntry);
}

function removeEntry(groupId: string, entryId: string) {
  removeConfigEntry(groupId, entryId);
}

// Example data
configGroups.value = [
  {
    id: 'group1',
    name: 'Group 1',
    entries: [
      {
        id: 'entry1',
        path: 'path/to/config.json',
        type: 'json',
        description: 'Example JSON config',
        content: {},
      },
    ],
  },
];
</script>

<style scoped>
.container {
  padding: 20px;
}

.config-group {
  margin-bottom: 20px;
}

button {
  margin-left: 10px;
}
</style>