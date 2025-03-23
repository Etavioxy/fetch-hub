<template>
  <main>
    <TermEntry>
      <div class="container">
        <div class="sidebar">
          <div
            v-for="(group, index) in configGroups"
            :key="group.id"
            :class="index % 2 === 0 ? 'dark' : ''"
          >
            <h2>{{ group.name }}</h2>
            <Pa
              v-for="entry in group.entries"
              :key="entry.id"
              :execute="() => setCurrent(entry, group.id)"
            >
              {{ entry.path }}
            </Pa>
            <Pa :execute="() => setCurrentGroupForAddNew(group.id)">Add Entry</Pa>
          </div>
        </div>
        <div class="content">
          <ConfigEntryView
            v-if="currentEntry && currentGroupId"
            :entry="currentEntry"
            @remove="removeEntry(currentGroupId, $event)"
            @edit="editEntry(currentGroupId, currentEntry)"
          />
          <ConfigEntryModal
            :entry="currentEntry"
            @add="addEntry"
            @update="updateEntry"
          />
        </div>
      </div>
    </TermEntry>
  </main>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useConfigStore } from '../configStore';
import { ConfigEntry } from '../types';
import ConfigEntryView from './ConfigEntryView.vue';
import ConfigEntryModal from './ConfigEntryModal.vue';
import { TermEntry, Pa } from 'termui-vue';

const currentGroupId = ref<string | null>(null);
const currentEntry = ref<ConfigEntry | null>(null);

function setCurrent(entry: ConfigEntry, groupId: string) {
  currentGroupId.value = groupId;
  currentEntry.value = entry;
}

function setCurrentGroupForAddNew(groupId: string) {
  currentGroupId.value = groupId;
  currentEntry.value = null;
  document.getElementById('config-entry-modal')?.click();
}

const { configGroups, addConfigEntry, removeConfigEntry, updateConfigEntry } = useConfigStore();

function addEntry(entry: ConfigEntry) {
  if (currentGroupId.value) {
    console.log('Adding entry:', entry);
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
  display: flex;
  height: 100vh;
}

.sidebar {
  flex: 2;
  border-right: 1px solid var(--subtle-color);
  padding: 1em;
  overflow-y: auto;
}

.content {
  flex: 3;
  padding: 1em;
  overflow-y: auto;
}
</style>