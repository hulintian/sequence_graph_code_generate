<script setup lang="ts">
import { computed } from 'vue'
import { useDiagramStore } from '../../stores/diagram'
import type { Lifeline } from '../../types/diagram'

const props = defineProps<{
  lifeline: Lifeline
  canvasHeight: number
  destroyY?: number | null
}>()

const store = useDiagramStore()

const isSelected = computed(() =>
  store.selectedElementId === props.lifeline.id &&
  store.selectedElementType === 'lifeline'
)

const boxWidth = 120
const boxHeight = 40

const x = computed(() => props.lifeline.position.x)
const y = computed(() => props.lifeline.position.y)
const lineEndY = computed(() => props.destroyY ?? props.canvasHeight - 40)

// No stopPropagation — let event bubble to CanvasArea for drag handling
</script>

<template>
  <g class="lifeline" :class="{ selected: isSelected }">
    <!-- Dashed vertical line -->
    <line
      :x1="x" :y1="y + boxHeight"
      :x2="x" :y2="lineEndY"
      stroke="#555"
      stroke-width="1"
      stroke-dasharray="6,4"
    />

    <!-- Header box -->
    <rect
      :x="x - boxWidth / 2" :y="y"
      :width="boxWidth" :height="boxHeight"
      rx="4"
      :fill="isSelected ? '#2d5cdb' : '#3c3f41'"
      :stroke="isSelected ? '#4b7bff' : '#555'"
      stroke-width="1.5"
      class="lifeline-box"
    />

    <!-- Stereotype text -->
    <text
      v-if="lifeline.properties.stereotype"
      :x="x" :y="y + 14"
      text-anchor="middle"
      fill="#888"
      font-size="10"
      font-style="italic"
    >{{ lifeline.properties.stereotype }}</text>

    <!-- Name text -->
    <text
      :x="x" :y="y + (lifeline.properties.stereotype ? 30 : 25)"
      text-anchor="middle"
      fill="#ddd"
      font-size="13"
      font-weight="500"
    >{{ lifeline.name }}</text>

    <!-- Destroy X symbol -->
    <template v-if="destroyY">
      <line :x1="x - 10" :y1="destroyY - 10" :x2="x + 10" :y2="destroyY + 10"
            stroke="#e05555" stroke-width="2.5" />
      <line :x1="x - 10" :y1="destroyY + 10" :x2="x + 10" :y2="destroyY - 10"
            stroke="#e05555" stroke-width="2.5" />
    </template>
  </g>
</template>

<style scoped>
.lifeline-box {
  cursor: pointer;
}

.lifeline-box:hover {
  filter: brightness(1.15);
}

.lifeline.selected .lifeline-box {
  filter: none;
}
</style>
