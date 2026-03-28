import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type {
  Lifeline, Message, CombinedFragment, ViewState,
  DiagramMetadata, CodeGenConfig, ToolType, ElementType,
  DiagramFile
} from '../types/diagram'

let idCounter = 0
let lifelineCounter = 0
function generateId(prefix: string): string {
  return `${prefix}-${Date.now()}-${++idCounter}`
}

function createDefaultCodeGenConfig(): CodeGenConfig {
  return {
    outputDir: './generated',
    language: 'cpp',
    templateSet: 'default',
    oldVersionDir: null,
    namespace: '',
    lifelineOverrides: {}
  }
}

function createDefaultMetadata(): DiagramMetadata {
  const now = new Date().toISOString()
  return {
    id: generateId('diagram'),
    name: '未命名顺序图',
    createdAt: now,
    updatedAt: now,
    author: 'user',
    codeGenConfig: createDefaultCodeGenConfig()
  }
}

export interface DiagramSnapshot {
  lifelines: Lifeline[]
  messages: Message[]
  combinedFragments: CombinedFragment[]
}

export const useDiagramStore = defineStore('diagram', () => {
  // Data
  const metadata = ref<DiagramMetadata>(createDefaultMetadata())
  const lifelines = ref<Lifeline[]>([])
  const messages = ref<Message[]>([])
  const combinedFragments = ref<CombinedFragment[]>([])
  const viewState = ref<ViewState>({
    zoom: 1.0,
    panX: 0,
    panY: 0,
    gridEnabled: true,
    gridSize: 20
  })

  // UI State
  const selectedElementId = ref<string | null>(null)
  const selectedElementType = ref<ElementType>(null)
  const activeTool = ref<ToolType>('select')
  const isDirty = ref(false)
  const currentFilePath = ref<string | null>(null)

  // Undo/Redo
  const undoStack = ref<DiagramSnapshot[]>([])
  const redoStack = ref<DiagramSnapshot[]>([])

  // Computed
  const selectedLifeline = computed(() => {
    if (selectedElementType.value !== 'lifeline' || !selectedElementId.value) return null
    return lifelines.value.find(l => l.id === selectedElementId.value) ?? null
  })

  const selectedMessage = computed(() => {
    if (selectedElementType.value !== 'message' || !selectedElementId.value) return null
    return messages.value.find(m => m.id === selectedElementId.value) ?? null
  })

  const selectedFragment = computed(() => {
    if (selectedElementType.value !== 'fragment' || !selectedElementId.value) return null
    return combinedFragments.value.find(f => f.id === selectedElementId.value) ?? null
  })

  const elementCount = computed(() =>
    lifelines.value.length + messages.value.length + combinedFragments.value.length
  )

  // Helpers
  function pushUndo() {
    undoStack.value.push({
      lifelines: JSON.parse(JSON.stringify(lifelines.value)),
      messages: JSON.parse(JSON.stringify(messages.value)),
      combinedFragments: JSON.parse(JSON.stringify(combinedFragments.value)),
    })
    redoStack.value = []
    isDirty.value = true
    metadata.value.updatedAt = new Date().toISOString()
  }

  // Actions
  function selectElement(id: string | null, type: ElementType) {
    selectedElementId.value = id
    selectedElementType.value = type
  }

  function clearSelection() {
    selectedElementId.value = null
    selectedElementType.value = null
  }

  function setTool(tool: ToolType) {
    activeTool.value = tool
    if (tool !== 'select') {
      clearSelection()
    }
  }

  function addLifeline(x: number, name?: string) {
    pushUndo()
    const id = generateId('ll')
    lifelines.value.push({
      id,
      name: name ?? `Object${++lifelineCounter}`,
      type: 'class',
      position: { x, y: 60 },
      properties: {
        stereotype: '',
        attributes: [],
        namespace: ''
      }
    })
    selectElement(id, 'lifeline')
  }

  function addMessage(
    sourceId: string,
    targetId: string,
    type: 'sync' | 'async' | 'return' = 'sync',
    name?: string
  ) {
    pushUndo()
    const id = generateId('msg')
    const orderIndex = messages.value.length + 1
    messages.value.push({
      id,
      name: name ?? `method${orderIndex}`,
      type,
      sourceLifelineId: sourceId,
      targetLifelineId: targetId,
      orderIndex,
      customY: null,
      arguments: [],
      returnType: 'void',
      guard: '',
      parentFragmentId: null
    })
    selectElement(id, 'message')
  }

  function addCombinedFragment(
    type: 'alt' | 'loop' | 'opt' | 'par',
    messageIds: string[],
    rect?: { x: number; y: number; width: number; height: number }
  ) {
    pushUndo()
    const id = generateId('cf')
    const defaultGuard = type === 'loop' ? 'i < n' : type === 'alt' ? 'condition' : 'condition'
    const operands = type === 'alt'
      ? [
          { id: generateId('op'), guard: defaultGuard, messageIds },
          { id: generateId('op'), guard: 'else', messageIds: [] }
        ]
      : [{ id: generateId('op'), guard: defaultGuard, messageIds }]

    combinedFragments.value.push({
      id,
      type,
      parentFragmentId: null,
      operands,
      x: rect?.x ?? 80,
      y: rect?.y ?? 120,
      width: rect?.width ?? 240,
      height: rect?.height ?? 120,
      dividerRatio: 0.5
    })
    selectElement(id, 'fragment')
  }

  function updateLifeline(id: string, updates: Partial<Lifeline>) {
    pushUndo()
    const idx = lifelines.value.findIndex(l => l.id === id)
    if (idx !== -1) {
      lifelines.value[idx] = { ...lifelines.value[idx], ...updates }
    }
  }

  function updateMessage(id: string, updates: Partial<Message>) {
    pushUndo()
    const idx = messages.value.findIndex(m => m.id === id)
    if (idx !== -1) {
      messages.value[idx] = { ...messages.value[idx], ...updates }
    }
  }

  function moveMessageOrder(id: string, direction: 'up' | 'down') {
    const sorted = [...messages.value].sort((a, b) => a.orderIndex - b.orderIndex)
    const idx = sorted.findIndex(m => m.id === id)
    if (idx < 0) return
    const swapIdx = direction === 'up' ? idx - 1 : idx + 1
    if (swapIdx < 0 || swapIdx >= sorted.length) return
    pushUndo()
    const tmpOrder = sorted[idx].orderIndex
    sorted[idx].orderIndex = sorted[swapIdx].orderIndex
    sorted[swapIdx].orderIndex = tmpOrder
  }

  function updateFragment(id: string, updates: Partial<CombinedFragment>) {
    pushUndo()
    const idx = combinedFragments.value.findIndex(f => f.id === id)
    if (idx !== -1) {
      combinedFragments.value[idx] = { ...combinedFragments.value[idx], ...updates }
    }
  }

  function deleteSelected() {
    if (!selectedElementId.value || !selectedElementType.value) return
    pushUndo()
    const id = selectedElementId.value
    if (selectedElementType.value === 'lifeline') {
      lifelines.value = lifelines.value.filter(l => l.id !== id)
      messages.value = messages.value.filter(
        m => m.sourceLifelineId !== id && m.targetLifelineId !== id
      )
    } else if (selectedElementType.value === 'message') {
      messages.value = messages.value.filter(m => m.id !== id)
    } else if (selectedElementType.value === 'fragment') {
      combinedFragments.value = combinedFragments.value.filter(f => f.id !== id)
    }
    clearSelection()
  }

  function undo() {
    if (undoStack.value.length === 0) return
    redoStack.value.push({
      lifelines: JSON.parse(JSON.stringify(lifelines.value)),
      messages: JSON.parse(JSON.stringify(messages.value)),
      combinedFragments: JSON.parse(JSON.stringify(combinedFragments.value)),
    })
    const snapshot = undoStack.value.pop()!
    lifelines.value = snapshot.lifelines
    messages.value = snapshot.messages
    combinedFragments.value = snapshot.combinedFragments
    clearSelection()
  }

  function redo() {
    if (redoStack.value.length === 0) return
    undoStack.value.push({
      lifelines: JSON.parse(JSON.stringify(lifelines.value)),
      messages: JSON.parse(JSON.stringify(messages.value)),
      combinedFragments: JSON.parse(JSON.stringify(combinedFragments.value)),
    })
    const snapshot = redoStack.value.pop()!
    lifelines.value = snapshot.lifelines
    messages.value = snapshot.messages
    combinedFragments.value = snapshot.combinedFragments
    clearSelection()
  }

  const appZoom = ref(1)

  function setZoom(zoom: number) {
    viewState.value.zoom = Math.max(0.25, Math.min(3, zoom))
  }

  function setAppZoom(zoom: number) {
    appZoom.value = Math.max(0.25, Math.min(3, zoom))
  }

  function setPan(x: number, y: number) {
    viewState.value.panX = x
    viewState.value.panY = y
  }

  function moveLifeline(id: string, x: number) {
    const ll = lifelines.value.find(l => l.id === id)
    if (ll) {
      ll.position.x = x
      isDirty.value = true
    }
  }

  // Serialize / Deserialize
  function toJSON(): DiagramFile {
    return {
      version: '1.0.0',
      metadata: JSON.parse(JSON.stringify(metadata.value)),
      lifelines: JSON.parse(JSON.stringify(lifelines.value)),
      messages: JSON.parse(JSON.stringify(messages.value)),
      combinedFragments: JSON.parse(JSON.stringify(combinedFragments.value)),
      viewState: JSON.parse(JSON.stringify(viewState.value))
    }
  }

  function loadFromJSON(data: DiagramFile) {
    metadata.value = data.metadata
    lifelines.value = data.lifelines
    messages.value = data.messages
    combinedFragments.value = data.combinedFragments
    viewState.value = data.viewState
    undoStack.value = []
    redoStack.value = []
    isDirty.value = false
    clearSelection()
  }

  function newDiagram() {
    metadata.value = createDefaultMetadata()
    lifelines.value = []
    messages.value = []
    combinedFragments.value = []
    viewState.value = { zoom: 1, panX: 0, panY: 0, gridEnabled: true, gridSize: 20 }
    undoStack.value = []
    redoStack.value = []
    isDirty.value = false
    currentFilePath.value = null
    lifelineCounter = 0
    clearSelection()
  }

  return {
    // State
    metadata, lifelines, messages, combinedFragments, viewState, appZoom,
    selectedElementId, selectedElementType, activeTool, isDirty, currentFilePath,
    undoStack, redoStack,
    // Computed
    selectedLifeline, selectedMessage, selectedFragment, elementCount,
    // Actions
    selectElement, clearSelection, setTool,
    addLifeline, addMessage, addCombinedFragment,
    updateLifeline, updateMessage, moveMessageOrder, updateFragment,
    deleteSelected, undo, redo,
    setZoom, setAppZoom, setPan, moveLifeline,
    toJSON, loadFromJSON, newDiagram,
  }
})
