<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useDiagramStore } from '../../stores/diagram'

const store = useDiagramStore()
const generating = ref(false)
const statusMsg = ref('')
const showPreview = ref(false)
const previewFiles = ref<{ path: string; content: string }[]>([])
const previewIndex = ref(0)

function updateOutputDir(e: Event) {
  store.metadata.codeGenConfig.outputDir = (e.target as HTMLInputElement).value
  store.isDirty = true
}

function updateLanguage(e: Event) {
  store.metadata.codeGenConfig.language = (e.target as HTMLSelectElement).value as any
  store.isDirty = true
}

function updateTemplateSet(e: Event) {
  store.metadata.codeGenConfig.templateSet = (e.target as HTMLInputElement).value
  store.isDirty = true
}

function updateNamespace(e: Event) {
  store.metadata.codeGenConfig.namespace = (e.target as HTMLInputElement).value
  store.isDirty = true
}

function updateOldVersionDir(e: Event) {
  const val = (e.target as HTMLInputElement).value
  store.metadata.codeGenConfig.oldVersionDir = val || null
  store.isDirty = true
}

async function handleGenerate() {
  generating.value = true
  statusMsg.value = ''
  try {
    const diagramJson = JSON.stringify(store.toJSON())
    const config = store.metadata.codeGenConfig
    const result = await invoke<{
      success: boolean
      files: { path: string; action: string }[]
      warnings: string[]
    }>('generate_code', {
      diagramJson,
      outputDir: config.outputDir,
      oldVersionDir: config.oldVersionDir,
    })

    const created = result.files.filter(f => f.action === 'created').length
    const updated = result.files.filter(f => f.action === 'updated' || f.action === 'merged_user_code').length
    const unchanged = result.files.filter(f => f.action === 'unchanged').length
    statusMsg.value = `done: ${created} created, ${updated} updated, ${unchanged} unchanged`
    if (result.warnings.length > 0) {
      statusMsg.value += ` (${result.warnings.length} warnings)`
    }
  } catch (err: any) {
    statusMsg.value = `error: ${err}`
  } finally {
    generating.value = false
  }
}

async function handlePreview() {
  generating.value = true
  statusMsg.value = ''
  try {
    const diagramJson = JSON.stringify(store.toJSON())
    const files = await invoke<{ path: string; content: string }[]>('preview_code', {
      diagramJson,
    })
    previewFiles.value = files
    previewIndex.value = 0
    showPreview.value = true
  } catch (err: any) {
    statusMsg.value = `error: ${err}`
  } finally {
    generating.value = false
  }
}
</script>

<template>
  <div class="props-section">
    <h3>代码生成配置</h3>

    <label>
      <span>输出路径</span>
      <input type="text"
             :value="store.metadata.codeGenConfig.outputDir"
             @change="updateOutputDir"
             placeholder="./generated" />
    </label>

    <label>
      <span>目标语言</span>
      <select :value="store.metadata.codeGenConfig.language" @change="updateLanguage">
        <option value="cpp">C++</option>
        <option value="java">Java</option>
        <option value="python">Python</option>
      </select>
    </label>

    <label>
      <span>模板集</span>
      <input type="text"
             :value="store.metadata.codeGenConfig.templateSet"
             @change="updateTemplateSet"
             placeholder="default" />
    </label>

    <label>
      <span>命名空间</span>
      <input type="text"
             :value="store.metadata.codeGenConfig.namespace"
             @change="updateNamespace"
             placeholder="如 auth" />
    </label>

    <label>
      <span>旧版本路径</span>
      <input type="text"
             :value="store.metadata.codeGenConfig.oldVersionDir ?? ''"
             @change="updateOldVersionDir"
             placeholder="/tmp (默认)" />
    </label>

    <button class="gen-btn" :disabled="generating" @click="handleGenerate">
      {{ generating ? '生成中...' : '生成代码' }}
    </button>
    <button class="preview-btn" :disabled="generating" @click="handlePreview">
      预览
    </button>

    <div v-if="statusMsg" class="status-msg" :class="{ error: statusMsg.startsWith('error') }">
      {{ statusMsg }}
    </div>

    <!-- Preview modal -->
    <Teleport to="body">
      <div v-if="showPreview" class="preview-overlay" @click.self="showPreview = false">
        <div class="preview-modal">
          <div class="preview-header">
            <span>代码预览 ({{ previewFiles.length }} files)</span>
            <button class="close-btn" @click="showPreview = false">X</button>
          </div>
          <div class="preview-body">
            <div class="file-list">
              <div v-for="(file, idx) in previewFiles" :key="file.path"
                   class="file-item" :class="{ active: idx === previewIndex }"
                   @click="previewIndex = idx">
                {{ file.path }}
              </div>
            </div>
            <pre class="file-content">{{ previewFiles[previewIndex]?.content ?? '' }}</pre>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.props-section h3 {
  font-size: 13px;
  color: #ccc;
  margin: 0 0 12px 0;
  padding-bottom: 8px;
  border-bottom: 1px solid #3c3f41;
}

label {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: 10px;
}

label span {
  font-size: 11px;
  color: #888;
}

input, select {
  background: #1e1f22;
  border: 1px solid #3c3f41;
  border-radius: 4px;
  padding: 5px 8px;
  color: #ddd;
  font-size: 13px;
  outline: none;
}

input:focus, select:focus {
  border-color: #4b7bff;
}

.gen-btn, .preview-btn {
  width: 100%;
  padding: 7px;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
  margin-bottom: 6px;
}

.gen-btn {
  background: #2d5cdb;
  color: #fff;
}

.gen-btn:hover:not(:disabled) {
  background: #3a6bef;
}

.gen-btn:disabled, .preview-btn:disabled {
  opacity: 0.5;
  cursor: default;
}

.preview-btn {
  background: #3c3f41;
  color: #bbb;
}

.preview-btn:hover:not(:disabled) {
  background: #4c4f51;
}

.status-msg {
  font-size: 11px;
  color: #8c8;
  margin-top: 4px;
  word-break: break-all;
}

.status-msg.error {
  color: #e88;
}

.preview-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.preview-modal {
  width: 80vw;
  height: 70vh;
  background: #2b2d30;
  border: 1px solid #3c3f41;
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 16px;
  background: #1e1f22;
  border-bottom: 1px solid #3c3f41;
  color: #ccc;
  font-size: 14px;
}

.close-btn {
  background: none;
  border: none;
  color: #888;
  cursor: pointer;
  font-size: 16px;
  padding: 2px 8px;
}

.close-btn:hover {
  color: #fff;
}

.preview-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.file-list {
  width: 200px;
  border-right: 1px solid #3c3f41;
  overflow-y: auto;
  flex-shrink: 0;
}

.file-item {
  padding: 8px 12px;
  font-size: 12px;
  color: #aaa;
  cursor: pointer;
  border-bottom: 1px solid #3c3f41;
}

.file-item:hover {
  background: #3c3f41;
}

.file-item.active {
  background: #2d5cdb;
  color: #fff;
}

.file-content {
  flex: 1;
  padding: 16px;
  overflow: auto;
  font-size: 13px;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  color: #ddd;
  line-height: 1.5;
  margin: 0;
  white-space: pre;
  background: #1e1f22;
}
</style>
