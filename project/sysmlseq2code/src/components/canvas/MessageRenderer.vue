<script setup lang="ts">
import { computed } from 'vue'
import { useDiagramStore } from '../../stores/diagram'
import type { Message } from '../../types/diagram'

const props = defineProps<{
  message: Message
  baseY: number
}>()

const store = useDiagramStore()

const isSelected = computed(() =>
  store.selectedElementId === props.message.id &&
  store.selectedElementType === 'message'
)

const sourceLifeline = computed(() =>
  store.lifelines.find(l => l.id === props.message.sourceLifelineId)
)

const targetLifeline = computed(() =>
  store.lifelines.find(l => l.id === props.message.targetLifelineId)
)

const x1 = computed(() => sourceLifeline.value?.position.x ?? 0)
const x2 = computed(() => targetLifeline.value?.position.x ?? 0)
const y = computed(() => props.baseY)

const isSelfCall = computed(() => props.message.sourceLifelineId === props.message.targetLifelineId)

const arrowMarkerId = computed(() => {
  if (props.message.type === 'async') return 'arrow-open'
  if (props.message.type === 'return') return 'arrow-open'
  if (props.message.type === 'create') return 'arrow-open'
  // destroy uses filled arrow (same as sync)
  return 'arrow-filled'
})

const strokeDash = computed(() => {
  if (props.message.type === 'return') return '6,4'
  if (props.message.type === 'create') return '6,4'
  return 'none'
})

const label = computed(() => {
  const args = props.message.arguments.map(a => a.name).join(', ')
  const name = props.message.name
  if (props.message.type === 'return') {
    return props.message.returnType !== 'void' ? `return ${props.message.returnType}` : 'return'
  }
  if (props.message.type === 'create') {
    return name ? `<<create>> ${name}` : '<<create>>'
  }
  if (props.message.type === 'destroy') {
    return '<<destroy>>'
  }
  return args ? `${name}(${args})` : `${name}()`
})

// Click detection handled by CanvasArea
</script>

<template>
  <g class="message" :class="{ selected: isSelected }">
    <!-- Self-call: loop shape -->
    <template v-if="isSelfCall">
      <polyline
        :points="`${x1},${y} ${x1 + 40},${y} ${x1 + 40},${y + 24} ${x1},${y + 24}`"
        fill="none"
        :stroke="isSelected ? '#4b7bff' : '#aaa'"
        stroke-width="1.5"
        :marker-end="`url(#${arrowMarkerId})`"
      />
      <text
        :x="x1 + 45" :y="y + 15"
        fill="#ccc"
        font-size="12"
        text-anchor="start"
      >{{ label }}</text>
    </template>

    <!-- Normal message arrow -->
    <template v-else>
      <line
        :x1="x1" :y1="y"
        :x2="x2" :y2="y"
        :stroke="isSelected ? '#4b7bff' : '#aaa'"
        stroke-width="1.5"
        :stroke-dasharray="strokeDash"
        :marker-end="`url(#${arrowMarkerId})`"
        class="message-line"
      />

      <!-- Label -->
      <text
        :x="(x1 + x2) / 2" :y="y - 6"
        fill="#ccc"
        font-size="12"
        text-anchor="middle"
      >{{ label }}</text>
    </template>

    <!-- Destroy X symbol at target -->
    <template v-if="message.type === 'destroy' && !isSelfCall">
      <line :x1="x2 - 10" :y1="y - 10" :x2="x2 + 10" :y2="y + 10"
            stroke="#e05555" stroke-width="2.5" />
      <line :x1="x2 - 10" :y1="y + 10" :x2="x2 + 10" :y2="y - 10"
            stroke="#e05555" stroke-width="2.5" />
    </template>

    <!-- Invisible wider hit area -->
    <line
      v-if="!isSelfCall"
      :x1="x1" :y1="y"
      :x2="x2" :y2="y"
      stroke="transparent"
      stroke-width="12"
      class="hit-area"
    />
  </g>
</template>

<style scoped>
.message-line, .hit-area {
  cursor: pointer;
}

.message:hover .message-line {
  stroke: #6b9eff;
}
</style>
