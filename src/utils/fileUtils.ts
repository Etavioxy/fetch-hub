import { readTextFile } from '@tauri-apps/plugin-fs';
import yaml from 'js-yaml';

export async function readConfigFile(path: string, type: 'json' | 'yaml'): Promise<any> {
  const content = await readTextFile(path);
  if (type === 'json') {
    return JSON.parse(content);
  } else if (type === 'yaml') {
    return yaml.load(content);
  }
}