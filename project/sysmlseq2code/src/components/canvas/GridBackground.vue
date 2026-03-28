<script setup lang="ts">
import { useDiagramStore } from '../../stores/diagram'
import { computed } from 'vue'

const store = useDiagramStore()

const patternId = 'grid-pattern'
const gridSize = computed(() => store.viewState.gridSize * store.viewState.zoom)
</script>

<template>
  <g v-if="store.viewState.gridEnabled">
    <defs>
      <pattern :id="patternId" :width="gridSize" :height="gridSize" patternUnits="userSpaceOnUse">
        <circle :cx="gridSize / 2" :cy="gridSize / 2" r="0.5" fill="#3c3f41" />
      </pattern>
    </defs>
    <rect width="100%" height="100%" :fill="`url(#${patternId})`" />
  </g>
</template>
