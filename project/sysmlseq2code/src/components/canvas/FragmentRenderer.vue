<script setup lang="ts">
import { computed } from 'vue'
import { useDiagramStore } from '../../stores/diagram'
import type { CombinedFragment } from '../../types/diagram'

const props = defineProps<{
  fragment: CombinedFragment
  baseY: number
  height: number
  minX: number
  maxX: number
}>()

const store = useDiagramStore()

const isSelected = computed(() =>
  store.selectedElementId === props.fragment.id &&
  store.selectedElementType === 'fragment'
)

const padding = 20
const x = computed(() => props.minX - padding)
const y = computed(() => props.baseY - 20)
const width = computed(() => props.maxX - props.minX + padding * 2)
const height = computed(() => props.height + 40)

const typeLabel = computed(() => props.fragment.type.toUpperCase())

// Click detection handled by CanvasArea
</script>

<template>
  <g class="fragment" :class="{ selected: isSelected }">
    <!-- Outer rectangle -->
    <rect
      :x="x" :y="y"
      :width="width" :height="height"
      fill="none"
      :stroke="isSelected ? '#4b7bff' : '#666'"
      stroke-width="1.5"
      rx="2"
    />

    <!-- Type label pentagon -->
    <polygon
      :points="`${x},${y} ${x + 60},${y} ${x + 70},${y + 14} ${x + 60},${y + 24} ${x},${y + 24}`"
      :fill="isSelected ? '#2d5cdb' : '#3c3f41'"
      :stroke="isSelected ? '#4b7bff' : '#666'"
      stroke-width="1"
    />
    <text
      :x="x + 30" :y="y + 16"
      text-anchor="middle"
      fill="#ddd"
      font-size="11"
      font-weight="bold"
    >{{ typeLabel }}</text>

    <!-- Guard condition -->
    <text
      v-if="fragment.operands.length > 0 && fragment.operands[0].guard"
      :x="x + 78" :y="y + 16"
      fill="#aaa"
      font-size="11"
    >[{{ fragment.operands[0].guard }}]</text>

    <!-- Dashed divider for alt else -->
    <template v-if="fragment.type === 'alt' && fragment.operands.length > 1">
      <line
        :x1="x" :y1="y + height / 2"
        :x2="x + width" :y2="y + height / 2"
        stroke="#666"
        stroke-width="1"
        stroke-dasharray="6,4"
      />
      <text
        :x="x + 8" :y="y + height / 2 + 16"
        fill="#aaa"
        font-size="11"
      >[{{ fragment.operands[1].guard }}]</text>
    </template>
  </g>
</template>

<style scoped>
.fragment rect {
  cursor: pointer;
}
</style>
