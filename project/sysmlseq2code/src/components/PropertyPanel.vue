<script setup lang="ts">
import { useDiagramStore } from '../stores/diagram'
import LifelineProps from './properties/LifelineProps.vue'
import MessageProps from './properties/MessageProps.vue'
import FragmentProps from './properties/FragmentProps.vue'
import CodeGenConfig from './properties/CodeGenConfig.vue'

const store = useDiagramStore()
</script>

<template>
  <div class="property-panel">
    <template v-if="store.selectedElementType === 'lifeline'">
      <LifelineProps />
    </template>
    <template v-else-if="store.selectedElementType === 'message'">
      <MessageProps />
    </template>
    <template v-else-if="store.selectedElementType === 'fragment'">
      <FragmentProps />
    </template>
    <template v-else>
      <div class="empty-hint">
        <p>选择画布上的元素以编辑其属性</p>
      </div>
    </template>

    <div class="config-divider"></div>
    <CodeGenConfig />
  </div>
</template>

<style scoped>
.property-panel {
  width: 240px;
  background: #2b2d30;
  border-left: 1px solid #1e1f22;
  padding: 12px;
  overflow-y: auto;
  flex-shrink: 0;
}

.empty-hint {
  text-align: center;
  padding: 20px 0;
}

.empty-hint p {
  color: #555;
  font-size: 12px;
}

.config-divider {
  height: 1px;
  background: #3c3f41;
  margin: 16px 0;
}
</style>
