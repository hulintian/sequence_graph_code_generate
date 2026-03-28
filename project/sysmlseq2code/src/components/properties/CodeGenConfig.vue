<script setup lang="ts">
import { useDiagramStore } from '../../stores/diagram'

const store = useDiagramStore()

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

    <button class="gen-btn" disabled>
      生成代码
    </button>
    <button class="preview-btn" disabled>
      预览
    </button>
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
</style>
