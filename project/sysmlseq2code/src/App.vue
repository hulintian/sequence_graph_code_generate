<script setup lang="ts">
import MenuBar from './components/MenuBar.vue'
import ToolPanel from './components/ToolPanel.vue'
import CanvasArea from './components/CanvasArea.vue'
import PropertyPanel from './components/PropertyPanel.vue'
import StatusBar from './components/StatusBar.vue'
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
