<script setup lang="ts">
import { useDiagramStore } from '../stores/diagram'
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

const store = useDiagramStore()
const showFileMenu = ref(false)
const showEditMenu = ref(false)
const showViewMenu = ref(false)

function closeMenus() {
  showFileMenu.value = false
  showEditMenu.value = false
  showViewMenu.value = false
}

function toggleMenu(menu: 'file' | 'edit' | 'view') {
  const was = { file: showFileMenu.value, edit: showEditMenu.value, view: showViewMenu.value }
  closeMenus()
  if (menu === 'file') showFileMenu.value = !was.file
  else if (menu === 'edit') showEditMenu.value = !was.edit
  else if (menu === 'view') showViewMenu.value = !was.view
}

async function handleNew() {
  closeMenus()
  store.newDiagram()
}

async function handleSave() {
  closeMenus()
  try {
    const data = JSON.stringify(store.toJSON(), null, 2)
    const path = store.currentFilePath ?? 'diagram.seqd.json'
    await invoke('save_diagram', { path, content: data })
    store.isDirty = false
    store.currentFilePath = path
  } catch (e) {
    console.error('Save failed:', e)
  }
}

async function handleLoad() {
  closeMenus()
  try {
    const content = await invoke<string>('load_diagram', { path: '' })
    const data = JSON.parse(content)
    store.loadFromJSON(data)
  } catch (e) {
    console.error('Load failed:', e)
  }
}

function handleUndo() {
  closeMenus()
  store.undo()
}

function handleRedo() {
  closeMenus()
  store.redo()
}

function handleDelete() {
  closeMenus()
  store.deleteSelected()
}

function toggleGrid() {
  closeMenus()
  store.viewState.gridEnabled = !store.viewState.gridEnabled
}

function resetZoom() {
  closeMenus()
  store.setZoom(1)
  store.setPan(0, 0)
}
</script>

<template>
  <div class="menu-bar" @mouseleave="closeMenus">
    <div class="menu-item-wrapper">
      <button class="menu-item" @click="toggleMenu('file')">文件</button>
      <div v-if="showFileMenu" class="dropdown">
        <button @click="handleNew">新建 <span class="shortcut">Ctrl+N</span></button>
        <button @click="handleLoad">打开... <span class="shortcut">Ctrl+O</span></button>
        <button @click="handleSave">保存 <span class="shortcut">Ctrl+S</span></button>
        <div class="separator"></div>
        <button disabled>导出 XMI... <span class="shortcut">Ctrl+E</span></button>
        <button disabled>导入 XMI...</button>
      </div>
    </div>

    <div class="menu-item-wrapper">
      <button class="menu-item" @click="toggleMenu('edit')">编辑</button>
      <div v-if="showEditMenu" class="dropdown">
        <button @click="handleUndo" :disabled="store.undoStack.length === 0">
          撤销 <span class="shortcut">Ctrl+Z</span>
        </button>
        <button @click="handleRedo" :disabled="store.redoStack.length === 0">
          重做 <span class="shortcut">Ctrl+Y</span>
        </button>
        <div class="separator"></div>
        <button @click="handleDelete" :disabled="!store.selectedElementId">
          删除 <span class="shortcut">Del</span>
        </button>
      </div>
    </div>

    <div class="menu-item-wrapper">
      <button class="menu-item" @click="toggleMenu('view')">视图</button>
      <div v-if="showViewMenu" class="dropdown">
        <button @click="toggleGrid">
          {{ store.viewState.gridEnabled ? '✓ ' : '' }}显示网格
        </button>
        <button @click="resetZoom">重置缩放 <span class="shortcut">Ctrl+0</span></button>
      </div>
    </div>

    <div class="menu-item-wrapper">
      <button class="menu-item" disabled>代码生成</button>
    </div>

    <div class="spacer"></div>

    <span class="title">{{ store.metadata.name }}{{ store.isDirty ? ' *' : '' }}</span>
  </div>
</template>

<style scoped>
.menu-bar {
  display: flex;
  align-items: center;
  height: 32px;
  background: #2b2d30;
  border-bottom: 1px solid #1e1f22;
  padding: 0 4px;
  user-select: none;
  flex-shrink: 0;
}

.menu-item-wrapper {
  position: relative;
}

.menu-item {
  background: none;
  border: none;
  color: #bbb;
  padding: 4px 10px;
  font-size: 13px;
  cursor: pointer;
  border-radius: 4px;
}

.menu-item:hover:not(:disabled) {
  background: #3c3f41;
  color: #fff;
}

.menu-item:disabled {
  color: #666;
}

.dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  background: #2b2d30;
  border: 1px solid #3c3f41;
  border-radius: 6px;
  padding: 4px 0;
  min-width: 200px;
  z-index: 1000;
  box-shadow: 0 4px 12px rgba(0,0,0,0.4);
}

.dropdown button {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  background: none;
  border: none;
  color: #bbb;
  padding: 6px 16px;
  font-size: 13px;
  cursor: pointer;
  text-align: left;
}

.dropdown button:hover:not(:disabled) {
  background: #2d5cdb;
  color: #fff;
}

.dropdown button:disabled {
  color: #555;
  cursor: default;
}

.shortcut {
  color: #777;
  font-size: 12px;
  margin-left: 24px;
}

.dropdown button:hover .shortcut {
  color: #aaa;
}

.separator {
  height: 1px;
  background: #3c3f41;
  margin: 4px 8px;
}

.spacer {
  flex: 1;
}

.title {
  color: #888;
  font-size: 12px;
  padding-right: 12px;
}
</style>
