<script setup lang="ts">
import { watch } from 'vue'
import MenuBar from './components/MenuBar.vue'
import ToolPanel from './components/ToolPanel.vue'
import CanvasArea from './components/CanvasArea.vue'
import PropertyPanel from './components/PropertyPanel.vue'
import StatusBar from './components/StatusBar.vue'
import { useDiagramStore } from './stores/diagram'

const store = useDiagramStore()

// 使用 Tauri 原生 webview 缩放（不影响鼠标坐标），web 环境回退到 CSS zoom
const isTauri = '__TAURI_INTERNALS__' in window
watch(() => store.appZoom, async (zoom) => {
  if (isTauri) {
    const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow')
    await getCurrentWebviewWindow().setZoom(zoom)
  } else {
    document.documentElement.style.zoom = `${zoom}`
  }
})
</script>

<template>
  <div class="app-layout">
    <MenuBar />
    <div class="main-area">
      <ToolPanel />
      <CanvasArea />
      <PropertyPanel />
    </div>
    <StatusBar />
  </div>
</template>

<style>
/* Global reset */
*, *::before, *::after {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100%;
  height: 100%;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #1e1f22;
  color: #ddd;
}
</style>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100vh;
}

.main-area {
  display: flex;
  flex: 1;
  overflow: hidden;
}
</style>
