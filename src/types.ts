export interface ConfigEntry {
  id: string;
  path: string;
  type: 'json' | 'yaml';
  description: string;
  content: any;
}

export interface ConfigGroup {
  id: string;
  name: string;
  entries: ConfigEntry[];
}