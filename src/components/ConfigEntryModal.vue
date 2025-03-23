<template>
<div>
  <input type="checkbox" id="config-entry-modal" class="modal-toggle" />
  <div class="modal">
    <TermEntry class="modal-box bg-light">
      <h3 class="font-bold text-lg">{{ entry ? 'Edit' : 'Add' }} Config Entry</h3>
      <form @submit.prevent="submitForm">
      <div class="form-control">
        <P>Path</P>
        <label class="label">
        <P class="label-text"></P>
        </label>
        <div class="flex">
          <Pinput type="text" v-model="path" class="input input-bordered flex-grow" required />
          <button type="button" class="btn btn-secondary ml-2" @click="selectFile">Browse</button>
        </div>
      </div>
      <div class="form-control">
        <label class="label">
        <P class="label-text">Type</P>
        </label>
        <Pselect v-model="type" class="select select-bordered" required>
          <option value="json">JSON</option>
          <option value="yaml">YAML</option>
        </Pselect>
      </div>
      <div class="form-control">
        <label class="label">
        <P class="label-text">Description</P>
        </label>
        <Pinput type="text" v-model="description" class="input input-bordered" required />
      </div>
      <div class="form-control">
        <label class="label">
          <P class="label-text">Key</P>
        </label>
        <Pinput type="text" v-model="key" class="input input-bordered" required />
      </div>
      <div class="modal-action">
        <button type="submit" class="btn btn-primary">{{ entry ? 'Save' : 'Add' }}</button>
        <label for="config-entry-modal" class="btn">Cancel</label>
        <!-- 这里起作用是因为这个label标签的for属性指向了一个id为config-entry-modal的input元素，这个input元素是一个checkbox，当点击这个label标签时，会触发checkbox的checked状态变化，从而关闭模态框。 -->
      </div>
      </form>
    </TermEntry>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { ConfigEntry } from '../types';
import { open } from '@tauri-apps/plugin-dialog';
import { TermEntry, P, Pinput, Pselect } from 'termui-vue';

const props = defineProps<{ entry: ConfigEntry | null }>();
const emit = defineEmits<{
  (e: 'add', entry: ConfigEntry): void;
  (e: 'update', entry: ConfigEntry): void;
}>();

const path = ref('');
const type = ref<'json' | 'yaml'>('yaml');
const description = ref('');
const key = ref('');

watch(() => props.entry, (newEntry) => {
  console.log('New entry:', newEntry);
  if (newEntry) {
    path.value = newEntry.path;
    type.value = newEntry.type;
    description.value = newEntry.description;
    key.value = newEntry.key;
  } else {
    path.value = '';
    type.value = 'json';
    description.value = '';
    key.value = '';
  }
});

async function selectFile() {
  const selected = await open({
    multiple: false,
    filters: [
      { name: 'Config Files', extensions: ['json', 'yaml', 'yml'] },
    ],
  });
  if (selected && typeof selected === 'string') {
    path.value = selected;
  }
}

function submitForm() {
  const newEntry: ConfigEntry = {
    id: props.entry?.id || Date.now().toString(),
    path: path.value,
    type: type.value,
    description: description.value,
    key: key.value,
    content: {},
  };
  if (props.entry) {
    emit('update', newEntry);
  } else {
    emit('add', newEntry);
  }
  // 关闭模态框
  document.getElementById('config-entry-modal')?.click();
}
</script>

<style>
.modal {
  pointer-events: none;
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  margin: 0;
  display: grid;
  height: 100%;
  max-height: none;
  width: 100%;
  max-width: none;
  justify-items: center;
  padding: 0;
  opacity: 0;
  overscroll-behavior: contain;
  z-index: 999;
  background-color: transparent;
  color: inherit;
  transition-duration: .2s;
  transition-timing-function: cubic-bezier(0,0,.2,1);
  transition-property: transform, opacity, visibility;
  overflow-y: hidden;
}
.modal-box {
  max-height: calc(100vh - 5em);
  grid-column-start: 1;
  grid-row-start: 1;
  width: 91.666667%;
  max-width: 32rem;
  --tw-scale-x: .9;
  --tw-scale-y: .9;
  transform: translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skew(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y));
  border-bottom-right-radius: var(--rounded-box, 1rem);
  border-bottom-left-radius: var(--rounded-box, 1rem);
  border-top-left-radius: var(--rounded-box, 1rem);
  border-top-right-radius: var(--rounded-box, 1rem);
  --tw-bg-opacity: 1;
  background-color: var(--fallback-b1, oklch(var(--b1) / var(--tw-bg-opacity)));
  padding: 1.5rem;
  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, -webkit-backdrop-filter;
  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter, -webkit-backdrop-filter;
  transition-timing-function: cubic-bezier(.4,0,.2,1);
  transition-timing-function: cubic-bezier(0,0,.2,1);
  transition-duration: .2s;
  box-shadow: #00000040 0 25px 50px -12px;
  overflow-y: auto;
  overscroll-behavior: contain;
}
.modal-toggle:checked+.modal {
  pointer-events: auto;
  visibility: visible;
  opacity: 1;
}
.modal-toggle:checked+.modal .modal-box {
  --tw-translate-y: 0px;
  --tw-scale-x: 1;
  --tw-scale-y: 1;
}
</style>