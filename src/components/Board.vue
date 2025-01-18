<template>
  <main>
    <TermEntry>
      <div class="container">
        <div class="sidebar">
          <div v-for="(group, index) in configGroups" :key="group.id" :class="index % 2 === 0 ? 'dark' : ''">
            <h2>{{ group.name }}</h2>
            <Pa v-for="entry in group.entries" :key="entry.id" :execute="() => setNow(entry, group.id)">
             {{ entry.path }}
            </Pa>
            <Pa :execute="() => setCurrentGroup(group.id)">Add Entry</Pa>
          </div>
        </div>
        <div class="content">
          <ConfigEntryView v-if="nowEntry && nowGroupId" :entry="nowEntry" @remove="removeEntry(nowGroupId, $event)" @edit="editEntry(nowGroupId, nowEntry)" />
          <ConfigEntryModal :entry="currentEntry" @add="addEntry" @update="updateEntry"  />
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

const nowGroupId = ref<string | null>(null);
const nowEntry = ref<ConfigEntry | null>(null);

function setNow(entry: ConfigEntry, groupid: string) {
  nowGroupId.value = groupid;
  nowEntry.value = entry;
}

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