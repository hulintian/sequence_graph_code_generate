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

// Fragment drawing state (drag rectangle to select messages)
const isDrawingFragment = ref(false)
const fragmentDrawStart = ref({ x: 0, y: 0 })
const fragmentDrawEnd = ref({ x: 0, y: 0 })

// Message vertical drag state
const isDraggingMessage = ref(false)
const dragMessageId = ref<string | null>(null)

// Fragment resize state
const isResizingFragment = ref(false)
const resizeFragmentId = ref<string | null>(null)
const resizeEdge = ref<'n' | 's' | 'e' | 'w' | null>(null)
const resizeStartMouse = ref({ x: 0, y: 0 })
const resizeStartRect = ref({ x: 0, y: 0, width: 0, height: 0 })

// ALT divider drag state
const isDraggingDivider = ref(false)
const dividerFragmentId = ref<string | null>(null)

// Message Y positions
const messageBaseY = 140
const messageSpacing = 50

const messageYPositions = computed(() => {
  const positions: Record<string, number> = {}
  const sorted = [...store.messages].sort((a, b) => a.orderIndex - b.orderIndex)
  sorted.forEach((msg, i) => {
    positions[msg.id] = msg.customY ?? (messageBaseY + i * messageSpacing)
  })
  return positions
})

// Activation bars (execution specifications) — based on call-return pairs only
const activationBars = computed(() => {
  const bars: { lifelineId: string; x: number; topY: number; bottomY: number }[] = []
  const sorted = [...store.messages].sort((a, b) => {
    const ya = messageYPositions.value[a.id] ?? 0
    const yb = messageYPositions.value[b.id] ?? 0
    return ya - yb
  })

  for (const ll of store.lifelines) {
    // Find each incoming call (non-return) to this lifeline
    for (let i = 0; i < sorted.length; i++) {
      const msg = sorted[i]
      if (msg.targetLifelineId !== ll.id || msg.type === 'return') continue

      const topY = messageYPositions.value[msg.id] ?? messageBaseY

      // Find matching return: this lifeline sends a return back to the caller
      let bottomY = topY + 20 // minimum height if no return found
      for (let j = i + 1; j < sorted.length; j++) {
        const next = sorted[j]
        if (
          next.sourceLifelineId === ll.id &&
          next.targetLifelineId === msg.sourceLifelineId &&
          next.type === 'return'
        ) {
          bottomY = messageYPositions.value[next.id] ?? bottomY
          break
        }
      }

      // Also extend to cover any outgoing calls this lifeline makes during the activation
      for (let j = i + 1; j < sorted.length; j++) {
        const next = sorted[j]
        // Stop at the return
        if (next.sourceLifelineId === ll.id && next.targetLifelineId === msg.sourceLifelineId && next.type === 'return') break
        if (next.sourceLifelineId === ll.id && next.type !== 'return') {
          const nextY = messageYPositions.value[next.id] ?? 0
          if (nextY > bottomY) bottomY = nextY + 4
        }
      }

      bars.push({
        lifelineId: ll.id,
        x: ll.position.x,
        topY,
        bottomY,
      })
    }
  }
  return bars
})

// Fragment layout — uses stored geometry from CombinedFragment
const fragmentLayouts = computed(() => {
  return store.combinedFragments.map(frag => ({
    fragment: frag,
    x: frag.x,
    y: frag.y,
    width: frag.width,
    height: frag.height,
  }))
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

  if (tool === 'alt' || tool === 'loop' || tool === 'opt' || tool === 'par') {
    isDrawingFragment.value = true
    fragmentDrawStart.value = { x: pt.x, y: pt.y }
    fragmentDrawEnd.value = { x: pt.x, y: pt.y }
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

    // Check if clicking on a message line (drag to move vertically)
    for (const msg of store.messages) {
      const my = messageYPositions.value[msg.id] ?? messageBaseY
      const src = store.lifelines.find(l => l.id === msg.sourceLifelineId)
      const tgt = store.lifelines.find(l => l.id === msg.targetLifelineId)
      if (src && tgt && Math.abs(pt.y - my) < 10) {
        const minX = Math.min(src.position.x, tgt.position.x)
        const maxX = Math.max(src.position.x, tgt.position.x)
        if (pt.x >= minX - 10 && pt.x <= maxX + 10) {
          store.selectElement(msg.id, 'message')
          isDraggingMessage.value = true
          dragMessageId.value = msg.id
          return
        }
      }
    }

    // Check if clicking on fragment edges (resize) or ALT divider
    const edgeThreshold = 8
    for (const fl of fragmentLayouts.value) {
      const fx = fl.x, fy = fl.y, fw = fl.width, fh = fl.height
      const inside = pt.x >= fx - edgeThreshold && pt.x <= fx + fw + edgeThreshold &&
                     pt.y >= fy - edgeThreshold && pt.y <= fy + fh + edgeThreshold

      if (!inside) continue

      // ALT divider drag check
      if (fl.fragment.type === 'alt' && fl.fragment.operands.length > 1) {
        const divY = fy + fh * fl.fragment.dividerRatio
        if (Math.abs(pt.y - divY) < edgeThreshold && pt.x >= fx && pt.x <= fx + fw) {
          isDraggingDivider.value = true
          dividerFragmentId.value = fl.fragment.id
          store.selectElement(fl.fragment.id, 'fragment')
          return
        }
      }

      // Edge resize detection
      let edge: 'n' | 's' | 'e' | 'w' | null = null
      if (Math.abs(pt.y - fy) < edgeThreshold && pt.x >= fx && pt.x <= fx + fw) edge = 'n'
      else if (Math.abs(pt.y - (fy + fh)) < edgeThreshold && pt.x >= fx && pt.x <= fx + fw) edge = 's'
      else if (Math.abs(pt.x - fx) < edgeThreshold && pt.y >= fy && pt.y <= fy + fh) edge = 'w'
      else if (Math.abs(pt.x - (fx + fw)) < edgeThreshold && pt.y >= fy && pt.y <= fy + fh) edge = 'e'

      if (edge) {
        isResizingFragment.value = true
        resizeFragmentId.value = fl.fragment.id
        resizeEdge.value = edge
        resizeStartMouse.value = { x: pt.x, y: pt.y }
        resizeStartRect.value = { x: fx, y: fy, width: fw, height: fh }
        store.selectElement(fl.fragment.id, 'fragment')
        return
      }

      // Click inside fragment body — select it
      store.selectElement(fl.fragment.id, 'fragment')
      return
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

  if (isDrawingFragment.value) {
    const pt = getSvgPoint(e)
    fragmentDrawEnd.value = { x: pt.x, y: pt.y }
    return
  }

  if (isDrawingMessage.value) {
    const pt = getSvgPoint(e)
    messageDrawEnd.value = { x: pt.x, y: pt.y }
    return
  }

  if (isDraggingMessage.value && dragMessageId.value) {
    const pt = getSvgPoint(e)
    const msg = store.messages.find(m => m.id === dragMessageId.value)
    if (msg) {
      msg.customY = Math.max(messageBaseY, pt.y)
    }
    return
  }

  if (isResizingFragment.value && resizeFragmentId.value) {
    const pt = getSvgPoint(e)
    const frag = store.combinedFragments.find(f => f.id === resizeFragmentId.value)
    if (frag) {
      const dx = pt.x - resizeStartMouse.value.x
      const dy = pt.y - resizeStartMouse.value.y
      const minSize = 60
      if (resizeEdge.value === 'n') {
        const newY = resizeStartRect.value.y + dy
        const newH = resizeStartRect.value.height - dy
        if (newH >= minSize) { frag.y = newY; frag.height = newH }
      } else if (resizeEdge.value === 's') {
        frag.height = Math.max(minSize, resizeStartRect.value.height + dy)
      } else if (resizeEdge.value === 'w') {
        const newX = resizeStartRect.value.x + dx
        const newW = resizeStartRect.value.width - dx
        if (newW >= minSize) { frag.x = newX; frag.width = newW }
      } else if (resizeEdge.value === 'e') {
        frag.width = Math.max(minSize, resizeStartRect.value.width + dx)
      }
      store.isDirty = true
    }
    return
  }

  if (isDraggingDivider.value && dividerFragmentId.value) {
    const pt = getSvgPoint(e)
    const frag = store.combinedFragments.find(f => f.id === dividerFragmentId.value)
    if (frag) {
      const ratio = Math.max(0.1, Math.min(0.9, (pt.y - frag.y) / frag.height))
      frag.dividerRatio = ratio
      store.isDirty = true
    }
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

  if (isDrawingFragment.value) {
    isDrawingFragment.value = false
    // Find all messages within the dragged rectangle
    const x1 = Math.min(fragmentDrawStart.value.x, fragmentDrawEnd.value.x)
    const x2 = Math.max(fragmentDrawStart.value.x, fragmentDrawEnd.value.x)
    const y1 = Math.min(fragmentDrawStart.value.y, fragmentDrawEnd.value.y)
    const y2 = Math.max(fragmentDrawStart.value.y, fragmentDrawEnd.value.y)

    const selectedMsgIds: string[] = []
    for (const msg of store.messages) {
      const my = messageYPositions.value[msg.id] ?? messageBaseY
      const src = store.lifelines.find(l => l.id === msg.sourceLifelineId)
      const tgt = store.lifelines.find(l => l.id === msg.targetLifelineId)
      if (src && tgt) {
        const mx = (src.position.x + tgt.position.x) / 2
        if (mx >= x1 && mx <= x2 && my >= y1 && my <= y2) {
          selectedMsgIds.push(msg.id)
        }
      }
    }

    const fragType = store.activeTool as 'alt' | 'loop' | 'opt' | 'par'
    const fragRect = { x: x1, y: y1, width: x2 - x1, height: y2 - y1 }
    store.addCombinedFragment(fragType, selectedMsgIds, fragRect)
    // Link messages inside the rectangle to this fragment
    const newFrag = store.combinedFragments[store.combinedFragments.length - 1]
    for (const mid of selectedMsgIds) {
      const msg = store.messages.find(m => m.id === mid)
      if (msg) msg.parentFragmentId = newFrag.id
    }
    store.setTool('select')
    return
  }

  if (isDraggingMessage.value) {
    isDraggingMessage.value = false
    dragMessageId.value = null
    return
  }

  if (isResizingFragment.value) {
    isResizingFragment.value = false
    resizeFragmentId.value = null
    resizeEdge.value = null
    return
  }

  if (isDraggingDivider.value) {
    isDraggingDivider.value = false
    dividerFragmentId.value = null
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
    if (targetId) {
      const msgType = store.activeTool === 'async-message' ? 'async'
        : store.activeTool === 'return-message' ? 'return' : 'sync'
      const drawY = messageDrawStart.value.y
      store.addMessage(messageDrawStart.value.lifelineId, targetId, msgType)
      // Set customY to where the user drew the line
      const newMsg = store.messages[store.messages.length - 1]
      newMsg.customY = Math.max(messageBaseY, drawY)
      // Auto-assign parentFragmentId based on geometric containment
      const msgY = newMsg.customY
      for (const frag of store.combinedFragments) {
        if (msgY >= frag.y && msgY <= frag.y + frag.height &&
            pt.x >= frag.x && pt.x <= frag.x + frag.width) {
          newMsg.parentFragmentId = frag.id
          if (frag.operands.length > 0 && !frag.operands[0].messageIds.includes(newMsg.id)) {
            frag.operands[0].messageIds.push(newMsg.id)
          }
          break
        }
      }
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
    if (e.shiftKey) {
      // Ctrl+Shift +/-/0: 全局缩放（整个界面）
      if (e.key === '+' || e.key === '=') { e.preventDefault(); store.setAppZoom(store.appZoom + 0.1) }
      if (e.key === '_' || e.key === '-') { e.preventDefault(); store.setAppZoom(store.appZoom - 0.1) }
      if (e.key === ')' || e.key === '0') { e.preventDefault(); store.setAppZoom(1) }
    } else {
      // Ctrl +/-/0: 仅画布缩放
      if (e.key === '0') { e.preventDefault(); store.setZoom(1); store.setPan(0, 0) }
      if (e.key === '=' || e.key === '+') { e.preventDefault(); store.setZoom(store.viewState.zoom + 0.1) }
      if (e.key === '-') { e.preventDefault(); store.setZoom(store.viewState.zoom - 0.1) }
    }
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
          : isResizingFragment ? (resizeEdge === 'n' || resizeEdge === 's' ? 'ns-resize' : 'ew-resize')
          : isDraggingDivider ? 'row-resize'
          : store.activeTool === 'lifeline' ? 'crosshair'
          : (store.activeTool.endsWith('-message') || store.activeTool === 'sync-message') ? 'crosshair'
          : ['alt','loop','opt','par'].includes(store.activeTool) ? 'crosshair'
          : isDraggingLifeline ? 'ew-resize'
          : isDrawingFragment ? 'crosshair'
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
        />

        <!-- Lifelines -->
        <LifelineRenderer
          v-for="ll in store.lifelines"
          :key="ll.id"
          :lifeline="ll"
          :canvasHeight="canvasHeight"
        />

        <!-- Activation bars (execution specifications) -->
        <rect
          v-for="(bar, idx) in activationBars"
          :key="'act-' + idx"
          :x="bar.x - 5"
          :y="bar.topY"
          :width="10"
          :height="Math.max(bar.bottomY - bar.topY, 10)"
          fill="#3c3f41"
          stroke="#666"
          stroke-width="1"
          rx="1"
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
          :y1="messageDrawStart?.y ?? 0"
          :x2="messageDrawEnd.x"
          :y2="messageDrawStart?.y ?? 0"
          stroke="#4b7bff"
          stroke-width="1.5"
          stroke-dasharray="4,4"
          marker-end="url(#arrow-filled-blue)"
          pointer-events="none"
        />

        <!-- Fragment selection rectangle preview -->
        <rect
          v-if="isDrawingFragment"
          :x="Math.min(fragmentDrawStart.x, fragmentDrawEnd.x)"
          :y="Math.min(fragmentDrawStart.y, fragmentDrawEnd.y)"
          :width="Math.abs(fragmentDrawEnd.x - fragmentDrawStart.x)"
          :height="Math.abs(fragmentDrawEnd.y - fragmentDrawStart.y)"
          fill="rgba(75, 123, 255, 0.1)"
          stroke="#4b7bff"
          stroke-width="1.5"
          stroke-dasharray="6,3"
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
