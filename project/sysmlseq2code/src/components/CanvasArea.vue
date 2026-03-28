<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useDiagramStore } from '../stores/diagram'
import GridBackground from './canvas/GridBackground.vue'
import LifelineRenderer from './canvas/LifelineRenderer.vue'
import MessageRenderer from './canvas/MessageRenderer.vue'
import FragmentRenderer from './canvas/FragmentRenderer.vue'

const store = useDiagramStore()
const svgRef = ref<SVGSVGElement | null>(null)
const containerRef = ref<HTMLDivElement | null>(null)
const canvasWidth = ref(2000)
const canvasHeight = ref(1200)

// Pan state
const isPanning = ref(false)
const panStart = ref({ x: 0, y: 0 })

// Message drawing state
const isDrawingMessage = ref(false)
const messageDrawStart = ref<{ lifelineId: string; x: number; y: number } | null>(null)
const messageDrawEnd = ref({ x: 0, y: 0 })

// Lifeline drag state
const isDraggingLifeline = ref(false)
const dragLifelineId = ref<string | null>(null)
const dragOffsetX = ref(0)

// Message Y positions
const messageBaseY = 140
const messageSpacing = 50

const messageYPositions = computed(() => {
  const positions: Record<string, number> = {}
  const sorted = [...store.messages].sort((a, b) => a.orderIndex - b.orderIndex)
  sorted.forEach((msg, i) => {
    positions[msg.id] = messageBaseY + i * messageSpacing
  })
  return positions
})

// Fragment layout
const fragmentLayouts = computed(() => {
  return store.combinedFragments.map(frag => {
    const msgIds = frag.operands.flatMap(op => op.messageIds)
    const fragMessages = store.messages.filter(m => msgIds.includes(m.id))
    if (fragMessages.length === 0) {
      return { fragment: frag, baseY: messageBaseY, height: messageSpacing, minX: 100, maxX: 300 }
    }

    const ys = fragMessages.map(m => messageYPositions.value[m.id] ?? messageBaseY)
    const lifelineIds = new Set(fragMessages.flatMap(m => [m.sourceLifelineId, m.targetLifelineId]))
    const xs = store.lifelines
      .filter(l => lifelineIds.has(l.id))
      .map(l => l.position.x)

    return {
      fragment: frag,
      baseY: Math.min(...ys),
      height: Math.max(...ys) - Math.min(...ys) + messageSpacing,
      minX: xs.length > 0 ? Math.min(...xs) : 100,
      maxX: xs.length > 0 ? Math.max(...xs) : 300
    }
  })
})

function getSvgPoint(e: MouseEvent): { x: number; y: number } {
  if (!svgRef.value) return { x: 0, y: 0 }
  const svg = svgRef.value
  const pt = svg.createSVGPoint()
  pt.x = e.clientX
  pt.y = e.clientY
  // Transform screen coords to SVG viewBox coords (handles viewBox scaling)
  const ctm = svg.getScreenCTM()
  if (ctm) {
    const svgPt = pt.matrixTransform(ctm.inverse())
    // Then undo the pan/zoom transform applied to the inner <g>
    return {
      x: (svgPt.x - store.viewState.panX) / store.viewState.zoom,
      y: (svgPt.y - store.viewState.panY) / store.viewState.zoom,
    }
  }
  // Fallback
  const rect = svg.getBoundingClientRect()
  const scaleX = canvasWidth.value / rect.width
  const scaleY = canvasHeight.value / rect.height
  return {
    x: ((e.clientX - rect.left) * scaleX - store.viewState.panX) / store.viewState.zoom,
    y: ((e.clientY - rect.top) * scaleY - store.viewState.panY) / store.viewState.zoom,
  }
}

function findLifelineAtX(x: number): string | null {
  const threshold = 60
  for (const ll of store.lifelines) {
    if (Math.abs(ll.position.x - x) < threshold) {
      return ll.id
    }
  }
  return null
}

function handleCanvasMouseDown(e: MouseEvent) {
  if (e.button === 1 || (e.button === 0 && e.shiftKey)) {
    // Middle click or shift+click: start panning
    isPanning.value = true
    panStart.value = { x: e.clientX - store.viewState.panX, y: e.clientY - store.viewState.panY }
    return
  }

  const pt = getSvgPoint(e)
  const tool = store.activeTool

  if (tool === 'lifeline') {
    // Snap to grid
    const gridSize = store.viewState.gridSize
    const snappedX = Math.round(pt.x / gridSize) * gridSize
    store.addLifeline(snappedX)
    store.setTool('select')
    return
  }

  if (tool === 'sync-message' || tool === 'async-message' || tool === 'return-message') {
    const lifelineId = findLifelineAtX(pt.x)
    if (lifelineId) {
      isDrawingMessage.value = true
      messageDrawStart.value = { lifelineId, x: pt.x, y: pt.y }
      messageDrawEnd.value = { x: pt.x, y: pt.y }
    }
    return
  }

  if (tool === 'select') {
    // Check if clicking on a lifeline header for dragging
    for (const ll of store.lifelines) {
      const boxLeft = ll.position.x - 60
      const boxRight = ll.position.x + 60
      const boxTop = ll.position.y
      const boxBottom = ll.position.y + 40
      if (pt.x >= boxLeft && pt.x <= boxRight && pt.y >= boxTop && pt.y <= boxBottom) {
        isDraggingLifeline.value = true
        dragLifelineId.value = ll.id
        dragOffsetX.value = pt.x - ll.position.x
        store.selectElement(ll.id, 'lifeline')
        return
      }
    }

    // Check if clicking on a message line
    for (const msg of store.messages) {
      const my = messageYPositions.value[msg.id] ?? messageBaseY
      const src = store.lifelines.find(l => l.id === msg.sourceLifelineId)
      const tgt = store.lifelines.find(l => l.id === msg.targetLifelineId)
      if (src && tgt && Math.abs(pt.y - my) < 10) {
        const minX = Math.min(src.position.x, tgt.position.x)
        const maxX = Math.max(src.position.x, tgt.position.x)
        if (pt.x >= minX - 10 && pt.x <= maxX + 10) {
          store.selectElement(msg.id, 'message')
          return
        }
      }
    }

    // Check if clicking on a fragment border
    for (const fl of fragmentLayouts.value) {
      const fx = fl.minX - 20
      const fy = fl.baseY - 20
      const fw = fl.maxX - fl.minX + 40
      const fh = fl.height + 40
      if (pt.x >= fx && pt.x <= fx + fw && pt.y >= fy && pt.y <= fy + fh) {
        store.selectElement(fl.fragment.id, 'fragment')
        return
      }
    }

    // Click on empty space: clear selection
    store.clearSelection()
  }
}

function handleCanvasMouseMove(e: MouseEvent) {
  if (isPanning.value) {
    store.viewState.panX = e.clientX - panStart.value.x
    store.viewState.panY = e.clientY - panStart.value.y
    return
  }

  if (isDrawingMessage.value) {
    const pt = getSvgPoint(e)
    messageDrawEnd.value = { x: pt.x, y: pt.y }
    return
  }

  if (isDraggingLifeline.value && dragLifelineId.value) {
    const pt = getSvgPoint(e)
    const gridSize = store.viewState.gridSize
    const snappedX = Math.round((pt.x - dragOffsetX.value) / gridSize) * gridSize
    store.moveLifeline(dragLifelineId.value, snappedX)
  }
}

function handleCanvasMouseUp(e: MouseEvent) {
  if (isPanning.value) {
    isPanning.value = false
    return
  }

  if (isDraggingLifeline.value) {
    isDraggingLifeline.value = false
    dragLifelineId.value = null
    return
  }

  if (isDrawingMessage.value && messageDrawStart.value) {
    const pt = getSvgPoint(e)
    const targetId = findLifelineAtX(pt.x)
    if (targetId && targetId !== messageDrawStart.value.lifelineId) {
      const msgType = store.activeTool === 'async-message' ? 'async'
        : store.activeTool === 'return-message' ? 'return' : 'sync'
      store.addMessage(messageDrawStart.value.lifelineId, targetId, msgType)
      store.setTool('select')
    } else if (targetId && targetId === messageDrawStart.value.lifelineId) {
      // Self-call
      store.addMessage(messageDrawStart.value.lifelineId, targetId, 'sync')
      store.setTool('select')
    }
    isDrawingMessage.value = false
    messageDrawStart.value = null
  }
}

function handleWheel(e: WheelEvent) {
  e.preventDefault()
  const delta = e.deltaY > 0 ? -0.1 : 0.1
  store.setZoom(store.viewState.zoom + delta)
}

// Keyboard shortcuts
function handleKeyDown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    store.setTool('select')
    store.clearSelection()
    isDrawingMessage.value = false
    return
  }
  if (e.key === 'Delete' || e.key === 'Backspace') {
    if (document.activeElement?.tagName !== 'INPUT' && document.activeElement?.tagName !== 'TEXTAREA') {
      store.deleteSelected()
    }
    return
  }
  if (e.ctrlKey || e.metaKey) {
    if (e.key === 'z') { e.preventDefault(); store.undo() }
    if (e.key === 'y') { e.preventDefault(); store.redo() }
    if (e.key === '0') { e.preventDefault(); store.setZoom(1); store.setPan(0, 0) }
    if (e.key === '=' || e.key === '+') { e.preventDefault(); store.setZoom(store.viewState.zoom + 0.1) }
    if (e.key === '-') { e.preventDefault(); store.setZoom(store.viewState.zoom - 0.1) }
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})

// Drawing message preview line
const drawStartLifeline = computed(() => {
  if (!messageDrawStart.value) return null
  return store.lifelines.find(l => l.id === messageDrawStart.value!.lifelineId) ?? null
})
</script>

<template>
  <div class="canvas-container" ref="containerRef">
    <svg
      ref="svgRef"
      class="canvas-svg"
      :viewBox="`0 0 ${canvasWidth} ${canvasHeight}`"
      @mousedown="handleCanvasMouseDown"
      @mousemove="handleCanvasMouseMove"
      @mouseup="handleCanvasMouseUp"
      @wheel="handleWheel"
      :style="{
        cursor: isPanning ? 'grabbing'
          : store.activeTool === 'lifeline' ? 'crosshair'
          : (store.activeTool.endsWith('-message') || store.activeTool === 'sync-message') ? 'crosshair'
          : isDraggingLifeline ? 'ew-resize'
          : 'default'
      }"
    >
      <g :transform="`translate(${store.viewState.panX}, ${store.viewState.panY}) scale(${store.viewState.zoom})`">
        <!-- Grid -->
        <GridBackground />

        <!-- Fragments (rendered behind messages) -->
        <FragmentRenderer
          v-for="fl in fragmentLayouts"
          :key="fl.fragment.id"
          :fragment="fl.fragment"
          :baseY="fl.baseY"
          :height="fl.height"
          :minX="fl.minX"
          :maxX="fl.maxX"
        />

        <!-- Lifelines -->
        <LifelineRenderer
          v-for="ll in store.lifelines"
          :key="ll.id"
          :lifeline="ll"
          :canvasHeight="canvasHeight"
        />

        <!-- Arrow markers -->
        <defs>
          <marker id="arrow-filled" markerWidth="10" markerHeight="8" refX="10" refY="4" orient="auto">
            <polygon points="0,0 10,4 0,8" fill="#aaa" />
          </marker>
          <marker id="arrow-filled-blue" markerWidth="10" markerHeight="8" refX="10" refY="4" orient="auto">
            <polygon points="0,0 10,4 0,8" fill="#4b7bff" />
          </marker>
          <marker id="arrow-open" markerWidth="10" markerHeight="8" refX="10" refY="4" orient="auto">
            <polyline points="0,0 10,4 0,8" fill="none" stroke="#aaa" stroke-width="1.5" />
          </marker>
        </defs>

        <!-- Messages -->
        <MessageRenderer
          v-for="msg in store.messages"
          :key="msg.id"
          :message="msg"
          :baseY="messageYPositions[msg.id] ?? messageBaseY"
        />

        <!-- Drawing preview line -->
        <line
          v-if="isDrawingMessage && drawStartLifeline"
          :x1="drawStartLifeline.position.x"
          :y1="messageBaseY + store.messages.length * messageSpacing"
          :x2="messageDrawEnd.x"
          :y2="messageBaseY + store.messages.length * messageSpacing"
          stroke="#4b7bff"
          stroke-width="1.5"
          stroke-dasharray="4,4"
          marker-end="url(#arrow-filled-blue)"
          pointer-events="none"
        />
      </g>

      <!-- Empty state hint -->
      <text
        v-if="store.lifelines.length === 0"
        x="50%" y="50%"
        text-anchor="middle"
        fill="#555"
        font-size="16"
      >选择左侧「生命线」工具，点击画布添加参与者</text>
    </svg>
  </div>
</template>

<style scoped>
.canvas-container {
  flex: 1;
  overflow: hidden;
  background: #1e1f22;
  position: relative;
}

.canvas-svg {
  width: 100%;
  height: 100%;
  display: block;
}
</style>
