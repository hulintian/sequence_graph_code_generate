<script setup lang="ts">
import { computed } from 'vue'
import { useDiagramStore } from '../../stores/diagram'
import type { CombinedFragment } from '../../types/diagram'

const props = defineProps<{
  fragment: CombinedFragment
}>()

const store = useDiagramStore()

const isSelected = computed(() =>
  store.selectedElementId === props.fragment.id &&
  store.selectedElementType === 'fragment'
)

const x = computed(() => props.fragment.x)
const y = computed(() => props.fragment.y)
const width = computed(() => props.fragment.width)
const height = computed(() => props.fragment.height)

const typeLabel = computed(() => props.fragment.type.toUpperCase())

const dividerY = computed(() => {
  if (props.fragment.type !== 'alt' || props.fragment.operands.length <= 1) return 0
  return y.value + height.value * props.fragment.dividerRatio
})
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

    <!-- Resize handles (visible when selected) -->
    <template v-if="isSelected">
      <!-- North edge -->
      <line :x1="x" :y1="y" :x2="x + width" :y2="y"
            stroke="transparent" stroke-width="8" style="cursor: ns-resize" />
      <!-- South edge -->
      <line :x1="x" :y1="y + height" :x2="x + width" :y2="y + height"
            stroke="transparent" stroke-width="8" style="cursor: ns-resize" />
      <!-- West edge -->
      <line :x1="x" :y1="y" :x2="x" :y2="y + height"
            stroke="transparent" stroke-width="8" style="cursor: ew-resize" />
      <!-- East edge -->
      <line :x1="x + width" :y1="y" :x2="x + width" :y2="y + height"
            stroke="transparent" stroke-width="8" style="cursor: ew-resize" />
    </template>

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
      fill="#e8a838"
      font-size="11"
      font-family="monospace"
    >[{{ fragment.operands[0].guard }}]</text>

    <!-- ALT dashed divider (draggable) -->
    <template v-if="fragment.type === 'alt' && fragment.operands.length > 1">
      <line
        :x1="x" :y1="dividerY"
        :x2="x + width" :y2="dividerY"
        stroke="#666"
        stroke-width="1"
        stroke-dasharray="6,4"
      />
      <!-- Invisible wider hit area for dragging -->
      <line
        :x1="x" :y1="dividerY"
        :x2="x + width" :y2="dividerY"
        stroke="transparent"
        stroke-width="10"
        style="cursor: row-resize"
      />
      <text
        :x="x + 8" :y="dividerY + 16"
        fill="#e8a838"
        font-size="11"
        font-family="monospace"
      >[{{ fragment.operands[1].guard }}]</text>
    </template>
  </g>
</template>

<style scoped>
.fragment rect {
  cursor: pointer;
}
</style>
