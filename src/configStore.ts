import { ref } from 'vue';
import { ConfigGroup, ConfigEntry } from './types';

const configGroups = ref<ConfigGroup[]>([]);

export function useConfigStore() {
  function addConfigEntry(groupId: string, entry: ConfigEntry) {
    const group = configGroups.value.find(g => g.id === groupId);
    if (group) {
      group.entries.push(entry);
    }
  }

  function removeConfigEntry(groupId: string, entryId: string) {
    const group = configGroups.value.find(g => g.id === groupId);
    if (group) {
      group.entries = group.entries.filter(e => e.id !== entryId);
    }
  }

  return {
    configGroups,
    addConfigEntry,
    removeConfigEntry,
  };
}