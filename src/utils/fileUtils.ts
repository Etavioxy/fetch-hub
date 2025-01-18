import { exists, BaseDirectory, readTextFile, readDir } from '@tauri-apps/plugin-fs';
import yaml from 'js-yaml';

export {
  testFilePath,
  readFile,
  readTextFileObject,
  readTextFileObjectCached
};

async function testFilePath(path: string): Promise<boolean> {
  return await exists(path);
}

async function readFile(path: string) {
  try {
    if( path.endsWith('\\') || path.endsWith('/') ){
      const files = await readDir(path);
      return JSON.stringify(files);
    }

    return await readTextFile(path/*, { dir: BaseDirectory.AppData }*/);
  } catch (error) {
    alert('Error reading file');
    console.error('Error reading file:', error);
  }
}

async function readTextFileObject(path: string, type: 'json' | 'yaml'): Promise<any> {
  if (type === 'json' && !path.endsWith('.json')) {
    console.warn('File is not JSON:', path);
  } else if (type === 'yaml' && !path.endsWith('.yaml')) {
    console.warn('File is not YAML:', path);
  }
  
  try {
    const content = await readFile(path) as string;

    if (type === 'json') {
      return JSON.parse(content);
    } else if (type === 'yaml') {
      return yaml.load(content);
    }
  } catch (error) {
    console.error('Error reading file content:', error);
  }
}

const pathContentMap = new Map<string, string>();

async function readTextFileObjectCached(path: string, type: 'json' | 'yaml'): Promise<any> {
  if (pathContentMap.has(path)) {
    return pathContentMap.get(path) || '';
  } else {
    const fileContent = await readTextFileObject(path, type);
    pathContentMap.set(path, fileContent);
    return fileContent;
  }
}