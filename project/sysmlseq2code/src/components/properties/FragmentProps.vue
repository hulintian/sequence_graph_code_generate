<script setup lang="ts">
import { useDiagramStore } from '../../stores/diagram'
import { computed } from 'vue'

const store = useDiagramStore()
const fragment = computed(() => store.selectedFragment)

function updateType(e: Event) {
  const val = (e.target as HTMLSelectElement).value as any
  if (fragment.value) {
    store.updateFragment(fragment.value.id, { type: val })
  }
}

function updateOperandGuard(operandIdx: number, e: Event) {
  const val = (e.target as HTMLInputElement).value
  if (!fragment.value) return
  const newOperands = [...fragment.value.operands]
  newOperands[operandIdx] = { ...newOperands[operandIdx], guard: val }
  store.updateFragment(fragment.value.id, { operands: newOperands })
}
</script>

<template>
  <div v-if="fragment" class="props-section">
    <h3>组合片段属性</h3>

    <label>
      <span>片段类型</span>
      <select :value="fragment.type" @change="updateType">
        <option value="alt">alt (条件分支)</option>
        <option value="loop">loop (循环)</option>
        <option value="opt">opt (可选)</option>
        <option value="par">par (并行)</option>
        <option value="break">break (中断)</option>
      </select>
    </label>

    <div class="operands-section">
      <span class="operands-label">操作数 (Operands)</span>
      <div v-for="(op, idx) in fragment.operands" :key="op.id" class="operand-item">
        <label>
          <span>守卫条件 #{{ idx + 1 }}</span>
          <input type="text" :value="op.guard" @change="updateOperandGuard(idx, $event)"
                 placeholder="条件表达式" />
        </label>
        <div class="operand-info">
          包含 {{ op.messageIds.length }} 条消息
        </div>
      </div>
    </div>

    <div class="info-row">
      <span>ID</span>
      <span class="info-value id-text">{{ fragment.id }}</span>
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

.operands-section {
  margin-bottom: 12px;
}

.operands-label {
  font-size: 11px;
  color: #888;
  display: block;
  margin-bottom: 8px;
}

.operand-item {
  background: #1e1f22;
  border-radius: 4px;
  padding: 8px;
  margin-bottom: 6px;
}

.operand-info {
  font-size: 11px;
  color: #666;
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
