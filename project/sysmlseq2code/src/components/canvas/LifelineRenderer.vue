<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
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

const isEditing = ref(false)
const editValue = ref('')
const inputRef = ref<HTMLInputElement | null>(null)

const isDuplicate = computed(() =>
  editValue.value !== '' && store.isLifelineNameDuplicate(editValue.value, props.lifeline.id)
)

function startEditing() {
  isEditing.value = true
  editValue.value = props.lifeline.name
  store.pushUndo()
  nextTick(() => {
    inputRef.value?.focus()
    inputRef.value?.select()
  })
}

function onInput() {
  store.renameLifeline(props.lifeline.id, editValue.value)
}

function finishEditing() {
  if (!isEditing.value) return
  isEditing.value = false
  if (isDuplicate.value) {
    // Revert to original name via undo
    store.undo()
  }
}

function onKeydown(e: KeyboardEvent) {
  e.stopPropagation()
  if (e.key === 'Enter') {
    finishEditing()
  } else if (e.key === 'Escape') {
    isEditing.value = false
    store.undo()
  }
}

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
      @dblclick.stop="startEditing"
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

    <!-- Name text (hidden when editing) -->
    <text
      v-if="!isEditing"
      :x="x" :y="y + (lifeline.properties.stereotype ? 30 : 25)"
      text-anchor="middle"
      fill="#ddd"
      font-size="13"
      font-weight="500"
      @dblclick.stop="startEditing"
    >{{ lifeline.name }}</text>

    <!-- Inline edit input -->
    <foreignObject
      v-if="isEditing"
      :x="x - boxWidth / 2 + 2" :y="y + 4"
      :width="boxWidth - 4" :height="boxHeight - 8"
    >
      <div xmlns="http://www.w3.org/1999/xhtml" class="edit-wrapper">
        <input
          ref="inputRef"
          v-model="editValue"
          class="inline-input"
          :class="{ 'input-error': isDuplicate }"
          @input="onInput"
          @blur="finishEditing"
          @keydown="onKeydown"
          @mousedown.stop
          @pointerdown.stop
        />
        <div v-if="isDuplicate" class="dup-warning">名称已存在</div>
      </div>
    </foreignObject>

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

.edit-wrapper {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
}

.inline-input {
  width: 100%;
  height: 100%;
  background: #1e1f22;
  border: 1px solid #4b7bff;
  border-radius: 3px;
  color: #fff;
  font-size: 13px;
  text-align: center;
  outline: none;
  padding: 0 4px;
  box-sizing: border-box;
}

.inline-input.input-error {
  border-color: #e05555;
}

.dup-warning {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  text-align: center;
  color: #e05555;
  font-size: 11px;
  margin-top: 2px;
  white-space: nowrap;
}
</style>
