import { ref } from 'vue';
import { ConfigGroup, ConfigEntry } from './types';
import { Store } from '@tauri-apps/plugin-store';

const store = await Store.load('.settings.dat');
console.log(store);

const configGroups = ref<ConfigGroup[]>([]);

async function loadConfigGroups() {
  const storedGroups = await store.get('configGroups');
  if (storedGroups) {
    configGroups.value = storedGroups as ConfigGroup[];
  }
}

async function saveConfigGroups() {
  await store.set('configGroups', configGroups.value);
  await store.save();
}

export function useConfigStore() {
  function addConfigEntry(groupId: string, entry: ConfigEntry) {
    const group = configGroups.value.find(g => g.id === groupId);
    if (group) {
      group.entries.push(entry);
      saveConfigGroups();
    }
  }

  function removeConfigEntry(groupId: string, entryId: string) {
    const group = configGroups.value.find(g => g.id === groupId);
    if (group) {
      group.entries = group.entries.filter(e => e.id !== entryId);
      saveConfigGroups();
    }
  }
  
  function updateConfigEntry(groupId: string, updatedEntry: ConfigEntry) {
    const group = configGroups.value.find(g => g.id === groupId);
    if (group) {
      const index = group.entries.findIndex(e => e.id === updatedEntry.id);
      if (index !== -1) {
        group.entries[index] = updatedEntry;
        saveConfigGroups();
      }
    }
  }

  return {
    configGroups,
    addConfigEntry,
    removeConfigEntry,
    updateConfigEntry,
    loadConfigGroups,
  };
}

// Load config groups when the store is initialized
loadConfigGroups();