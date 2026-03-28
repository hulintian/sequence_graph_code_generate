<script setup lang="ts">
import { useDiagramStore } from '../stores/diagram'
import type { ToolType } from '../types/diagram'

const store = useDiagramStore()

interface ToolItem {
  id: ToolType
  label: string
  icon: string
  group: 'element' | 'fragment' | 'action'
}

const tools: ToolItem[] = [
  { id: 'select', label: '选择', icon: '⇢', group: 'action' },
  { id: 'lifeline', label: '生命线', icon: '▯', group: 'element' },
  { id: 'sync-message', label: '同步调用', icon: '→', group: 'element' },
  { id: 'async-message', label: '异步调用', icon: '⇢', group: 'element' },
  { id: 'return-message', label: '返回', icon: '⇠', group: 'element' },
  { id: 'alt', label: 'alt', icon: '⎕', group: 'fragment' },
  { id: 'loop', label: 'loop', icon: '↻', group: 'fragment' },
  { id: 'opt', label: 'opt', icon: '?', group: 'fragment' },
  { id: 'par', label: 'par', icon: '‖', group: 'fragment' },
  { id: 'delete', label: '删除', icon: '✕', group: 'action' },
]

function selectTool(tool: ToolType) {
  if (tool === 'delete') {
    store.deleteSelected()
  } else {
    store.setTool(tool)
  }
}
</script>

<template>
  <div class="tool-panel">
    <div class="tool-section">
      <div class="section-label">操作</div>
      <template v-for="tool in tools.filter(t => t.group === 'action')" :key="tool.id">
        <button
          class="tool-btn"
          :class="{ active: store.activeTool === tool.id && tool.id !== 'delete' }"
          :title="tool.label"
          @click="selectTool(tool.id)"
        >
          <span class="tool-icon">{{ tool.icon }}</span>
          <span class="tool-label">{{ tool.label }}</span>
        </button>
      </template>
    </div>

    <div class="tool-section">
      <div class="section-label">元素</div>
      <template v-for="tool in tools.filter(t => t.group === 'element')" :key="tool.id">
        <button
          class="tool-btn"
          :class="{ active: store.activeTool === tool.id }"
          :title="tool.label"
          @click="selectTool(tool.id)"
        >
          <span class="tool-icon">{{ tool.icon }}</span>
          <span class="tool-label">{{ tool.label }}</span>
        </button>
      </template>
    </div>

    <div class="tool-section">
      <div class="section-label">片段</div>
      <template v-for="tool in tools.filter(t => t.group === 'fragment')" :key="tool.id">
        <button
          class="tool-btn"
          :class="{ active: store.activeTool === tool.id }"
          :title="tool.label"
          @click="selectTool(tool.id)"
        >
          <span class="tool-icon">{{ tool.icon }}</span>
          <span class="tool-label">{{ tool.label }}</span>
        </button>
      </template>
    </div>

    <div class="tool-section bottom-actions">
      <button class="tool-btn" title="撤销 (Ctrl+Z)" @click="store.undo()"
              :disabled="store.undoStack.length === 0">
        <span class="tool-icon">↶</span>
        <span class="tool-label">撤销</span>
      </button>
      <button class="tool-btn" title="重做 (Ctrl+Y)" @click="store.redo()"
              :disabled="store.redoStack.length === 0">
        <span class="tool-icon">↷</span>
        <span class="tool-label">重做</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.tool-panel {
  width: 72px;
  background: #2b2d30;
  border-right: 1px solid #1e1f22;
  display: flex;
  flex-direction: column;
  padding: 4px;
  gap: 2px;
  overflow-y: auto;
  flex-shrink: 0;
}

.tool-section {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.section-label {
  font-size: 10px;
  color: #666;
  text-align: center;
  padding: 6px 0 2px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.tool-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1px;
  padding: 6px 4px;
  background: none;
  border: 1px solid transparent;
  border-radius: 6px;
  cursor: pointer;
  color: #aaa;
  transition: all 0.15s;
}

.tool-btn:hover:not(:disabled) {
  background: #3c3f41;
  color: #fff;
}

.tool-btn.active {
  background: #2d5cdb;
  color: #fff;
  border-color: #4b7bff;
}

.tool-btn:disabled {
  opacity: 0.35;
  cursor: default;
}

.tool-icon {
  font-size: 18px;
  line-height: 1;
}

.tool-label {
  font-size: 9px;
  line-height: 1;
  white-space: nowrap;
}

.bottom-actions {
  margin-top: auto;
  border-top: 1px solid #3c3f41;
  padding-top: 6px;
}
</style>
