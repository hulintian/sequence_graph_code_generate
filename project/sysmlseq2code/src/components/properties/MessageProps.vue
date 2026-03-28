<script setup lang="ts">
import { useDiagramStore } from '../../stores/diagram'
import { computed } from 'vue'

const store = useDiagramStore()
const message = computed(() => store.selectedMessage)

const sourceLifelineName = computed(() =>
  store.lifelines.find(l => l.id === message.value?.sourceLifelineId)?.name ?? '?'
)
const targetLifelineName = computed(() =>
  store.lifelines.find(l => l.id === message.value?.targetLifelineId)?.name ?? '?'
)

function updateName(e: Event) {
  const val = (e.target as HTMLInputElement).value
  if (message.value) {
    store.updateMessage(message.value.id, { name: val })
  }
}

function updateType(e: Event) {
  const val = (e.target as HTMLSelectElement).value as any
  if (message.value) {
    store.updateMessage(message.value.id, { type: val })
  }
}

function updateReturnType(e: Event) {
  const val = (e.target as HTMLInputElement).value
  if (message.value) {
    store.updateMessage(message.value.id, { returnType: val })
  }
}

function updateGuard(e: Event) {
  const val = (e.target as HTMLInputElement).value
  if (message.value) {
    store.updateMessage(message.value.id, { guard: val })
  }
}

function updateArguments(e: Event) {
  const val = (e.target as HTMLInputElement).value
  if (!message.value) return
  // Parse "name:type, name:type" format
  const args = val.split(',').map(s => s.trim()).filter(Boolean).map(s => {
    const parts = s.split(':').map(p => p.trim())
    return { name: parts[0] || '', type: parts[1] || 'void' }
  })
  store.updateMessage(message.value.id, { arguments: args })
}

const argsString = computed(() =>
  message.value?.arguments.map(a => `${a.name}:${a.type}`).join(', ') ?? ''
)
</script>

<template>
  <div v-if="message" class="props-section">
    <h3>消息属性</h3>

    <label>
      <span>方法名</span>
      <input type="text" :value="message.name" @change="updateName" />
    </label>

    <label>
      <span>消息类型</span>
      <select :value="message.type" @change="updateType">
        <option value="sync">同步调用 (sync)</option>
        <option value="async">异步调用 (async)</option>
        <option value="return">返回 (return)</option>
        <option value="create">创建 (create)</option>
        <option value="destroy">销毁 (destroy)</option>
      </select>
    </label>

    <label>
      <span>参数 (格式: name:type, ...)</span>
      <input type="text" :value="argsString" @change="updateArguments"
             placeholder="username:string, password:string" />
    </label>

    <label>
      <span>返回类型</span>
      <input type="text" :value="message.returnType" @change="updateReturnType"
             placeholder="void" />
    </label>

    <label>
      <span>守卫条件</span>
      <input type="text" :value="message.guard" @change="updateGuard"
             placeholder="如 [isValid]" />
    </label>

    <div class="info-row">
      <span>来源</span>
      <span class="info-value">{{ sourceLifelineName }}</span>
    </div>

    <div class="info-row">
      <span>目标</span>
      <span class="info-value">{{ targetLifelineName }}</span>
    </div>

    <div class="info-row">
      <span>顺序</span>
      <span class="info-value">#{{ message.orderIndex }}</span>
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
</style>
