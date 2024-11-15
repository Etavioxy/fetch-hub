import { exists, BaseDirectory, readTextFile, readDir } from '@tauri-apps/plugin-fs';
import yaml from 'js-yaml';

export async function testFilePath(path: string): Promise<boolean> {
  return await exists(path);
}

export async function readFile(path: string) {
  try {
    console.log(await exists(path));
    if( path.endsWith('\\') || path.endsWith('/') ){
      const files = await readDir(path);
      console.log(files);
      return JSON.stringify(files);
    }

    return await readTextFile(path/*, { dir: BaseDirectory.AppData }*/);
  } catch (error) {
    alert('Error reading file');
    console.error('Error reading file:', error);
  }
}

export async function readConfigFile(path: string, type: 'json' | 'yaml'): Promise<any> {
  const content = await readFile(path) as string;
  if (type === 'json') {
    return JSON.parse(content);
  } else if (type === 'yaml') {
    return yaml.load(content);
  }
}