<template>
  <main class="container mx-auto p-4">
    <h1 class="text-2xl font-bold mb-4">Configuration Manager</h1>
    <div v-for="group in configGroups" :key="group.id" class="config-group mb-6">
      <h2 class="text-xl font-semibold mb-2">{{ group.name }}</h2>
      <ul class="list-disc pl-5">
        <li v-for="entry in group.entries" :key="entry.id" class="mb-2">
          <ConfigEntryView :entry="entry" @remove="removeEntry(group.id, $event)" @edit="editEntry(group.id, entry)" />
        </li>
      </ul>
      <label for="config-entry-modal" class="btn btn-primary mt-2" @click="setCurrentGroup(group.id)">Add Entry</label>
    </div>
    <ConfigEntryModal :entry="currentEntry" @add="addEntry" @update="updateEntry"  />
  </main>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useConfigStore } from '../configStore';
import { ConfigEntry } from '../types';
import ConfigEntryView from './ConfigEntryView.vue';
import ConfigEntryModal from './ConfigEntryModal.vue';

const { configGroups, addConfigEntry, removeConfigEntry, updateConfigEntry } = useConfigStore();
const currentGroupId = ref<string | null>(null);
const currentEntry = ref<ConfigEntry | null>(null);

function setCurrentGroup(groupId: string) {
  currentGroupId.value = groupId;
  currentEntry.value = null;
}

function addEntry(entry: ConfigEntry) {
  if (currentGroupId.value) {
    addConfigEntry(currentGroupId.value, entry);
  }
}

function updateEntry(entry: ConfigEntry) {
  if (currentGroupId.value) {
    updateConfigEntry(currentGroupId.value, entry);
  }
}

function removeEntry(groupId: string, entryId: string) {
  removeConfigEntry(groupId, entryId);
}

function editEntry(groupId: string, entry: ConfigEntry) {
  currentGroupId.value = groupId;
  currentEntry.value = entry;
  document.getElementById('config-entry-modal')?.click();
}

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