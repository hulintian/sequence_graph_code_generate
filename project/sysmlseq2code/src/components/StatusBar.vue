<script setup lang="ts">
import { useDiagramStore } from '../stores/diagram'

const store = useDiagramStore()
</script>

<template>
  <div class="status-bar">
    <span class="status-item">{{ store.metadata.name }}</span>
    <span class="status-sep">|</span>
    <span class="status-item" :class="{ dirty: store.isDirty }">
      {{ store.isDirty ? '已修改' : '已保存' }}
    </span>
    <span class="status-sep">|</span>
    <span class="status-item">画布: {{ Math.round(store.viewState.zoom * 100) }}%</span>
    <span class="status-sep">|</span>
    <span class="status-item">全局: {{ Math.round(store.appZoom * 100) }}%</span>
    <span class="status-sep">|</span>
    <span class="status-item">元素: {{ store.elementCount }}</span>
    <span class="spacer"></span>
    <span class="status-item">工具: {{ store.activeTool }}</span>
  </div>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  height: 24px;
  background: #2b2d30;
  border-top: 1px solid #1e1f22;
  padding: 0 12px;
  font-size: 12px;
  color: #888;
  user-select: none;
  flex-shrink: 0;
}

.status-item {
  white-space: nowrap;
}

.status-item.dirty {
  color: #e8a838;
}

.status-sep {
  margin: 0 8px;
  color: #444;
}

.spacer {
  flex: 1;
}
</style>
