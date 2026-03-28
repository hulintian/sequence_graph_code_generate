<script setup lang="ts">
import { useDiagramStore } from '../../stores/diagram'
import { computed } from 'vue'

const store = useDiagramStore()
const lifeline = computed(() => store.selectedLifeline)

function updateName(e: Event) {
  const val = (e.target as HTMLInputElement).value
  if (lifeline.value) {
    store.updateLifeline(lifeline.value.id, { name: val })
  }
}

function updateType(e: Event) {
  const val = (e.target as HTMLSelectElement).value as any
  if (lifeline.value) {
    store.updateLifeline(lifeline.value.id, { type: val })
  }
}

function updateStereotype(e: Event) {
  const val = (e.target as HTMLInputElement).value
  if (lifeline.value) {
    store.updateLifeline(lifeline.value.id, {
      properties: { ...lifeline.value.properties, stereotype: val }
    })
  }
}

function updateNamespace(e: Event) {
  const val = (e.target as HTMLInputElement).value
  if (lifeline.value) {
    store.updateLifeline(lifeline.value.id, {
      properties: { ...lifeline.value.properties, namespace: val }
    })
  }
}
</script>

<template>
  <div v-if="lifeline" class="props-section">
    <h3>生命线属性</h3>

    <label>
      <span>名称</span>
      <input type="text" :value="lifeline.name" @change="updateName" />
    </label>

    <label>
      <span>类型</span>
      <select :value="lifeline.type" @change="updateType">
        <option value="class">class</option>
        <option value="actor">actor</option>
        <option value="component">component</option>
        <option value="interface">interface</option>
      </select>
    </label>

    <label>
      <span>构造型</span>
      <input type="text" :value="lifeline.properties.stereotype" @change="updateStereotype"
             placeholder="如 &lt;&lt;boundary&gt;&gt;" />
    </label>

    <label>
      <span>命名空间</span>
      <input type="text" :value="lifeline.properties.namespace" @change="updateNamespace"
             placeholder="如 auth" />
    </label>

    <div class="info-row">
      <span>位置 X</span>
      <span class="info-value">{{ lifeline.position.x }}</span>
    </div>

    <div class="info-row">
      <span>ID</span>
      <span class="info-value id-text">{{ lifeline.id }}</span>
    </div>
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

.info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 6px;
  font-size: 12px;
}

.info-row span:first-child {
  color: #888;
}

.info-value {
  color: #aaa;
}

.id-text {
  font-size: 10px;
  font-family: monospace;
  opacity: 0.6;
}
</style>
