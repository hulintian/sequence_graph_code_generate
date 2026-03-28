<script setup lang="ts">
import { useDiagramStore } from '../../stores/diagram'
import { computed } from 'vue'

const store = useDiagramStore()
const fragment = computed(() => store.selectedFragment)

// Messages available to move into an operand (belong to this fragment but not in the target operand)
function availableMessagesForOperand(opIdx: number) {
  if (!fragment.value) return []
  const opMsgIds = new Set(fragment.value.operands[opIdx].messageIds)
  // Include messages in this fragment's other operands, plus unassigned messages with parentFragmentId == this fragment
  return store.messages.filter(m =>
    (m.parentFragmentId === fragment.value!.id && !opMsgIds.has(m.id))
  )
}

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

function moveMessageToOperand(msgId: string, targetOpIdx: number) {
  if (!fragment.value) return
  const newOperands = fragment.value.operands.map((op, idx) => ({
    ...op,
    messageIds: idx === targetOpIdx
      ? [...op.messageIds.filter(id => id !== msgId), msgId]
      : op.messageIds.filter(id => id !== msgId)
  }))
  store.updateFragment(fragment.value.id, { operands: newOperands })
}

function getMessageName(msgId: string) {
  const msg = store.messages.find(m => m.id === msgId)
  return msg ? `${msg.name}()` : msgId
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
          <span>{{ idx === 0 ? '条件 → 生成代码中的 if/while(...)' : '否则 (else)' }}</span>
          <input type="text" :value="op.guard" @change="updateOperandGuard(idx, $event)"
                 :placeholder="fragment.type === 'loop' ? 'i < n' : 'isValid'" />
        </label>
        <div class="operand-msgs">
          <div v-for="mid in op.messageIds" :key="mid" class="msg-tag">
            {{ getMessageName(mid) }}
          </div>
          <div v-if="op.messageIds.length === 0" class="operand-empty">暂无消息</div>
        </div>
        <div v-if="availableMessagesForOperand(idx).length > 0" class="move-section">
          <select @change="(e: Event) => { const v = (e.target as HTMLSelectElement).value; if (v) { moveMessageToOperand(v, idx); (e.target as HTMLSelectElement).value = ''; } }">
            <option value="">移入消息...</option>
            <option v-for="m in availableMessagesForOperand(idx)" :key="m.id" :value="m.id">
              {{ m.name }}()
            </option>
          </select>
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

.operand-msgs {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin: 4px 0;
}

.msg-tag {
  font-size: 11px;
  background: #2d5cdb33;
  color: #8ab4f8;
  padding: 2px 6px;
  border-radius: 3px;
}

.operand-empty {
  font-size: 11px;
  color: #555;
  font-style: italic;
}

.move-section select {
  width: 100%;
  margin-top: 4px;
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
