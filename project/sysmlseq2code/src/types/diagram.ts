export interface Position {
  x: number
  y: number
}

export interface Lifeline {
  id: string
  name: string
  type: 'class' | 'actor' | 'component' | 'interface'
  position: Position
  properties: {
    stereotype: string
    attributes: string[]
    namespace: string
  }
}

export interface MessageArgument {
  name: string
  type: string
}

export interface Message {
  id: string
  name: string
  type: 'sync' | 'async' | 'return' | 'create' | 'destroy'
  sourceLifelineId: string
  targetLifelineId: string
  orderIndex: number
  customY: number | null
  arguments: MessageArgument[]
  returnType: string
  guard: string
  parentFragmentId: string | null
  linkedReturnId: string | null
}

export interface FragmentOperand {
  id: string
  guard: string
  messageIds: string[]
}

export interface CombinedFragment {
  id: string
  type: 'alt' | 'loop' | 'opt' | 'par' | 'break'
  parentFragmentId: string | null
  operands: FragmentOperand[]
  // Stored geometry (set when created via drag, resizable later)
  x: number
  y: number
  width: number
  height: number
  // Ratios (0~1) for divider positions between operands (length = operands.length - 1)
  dividerRatios: number[]
}

export interface ViewState {
  zoom: number
  panX: number
  panY: number
  gridEnabled: boolean
  gridSize: number
}

export interface CodeGenConfig {
  outputDir: string
  language: 'cpp' | 'java' | 'python'
  templateSet: string
  oldVersionDir: string | null
  namespace: string
  lifelineOverrides: Record<string, {
    outputDir?: string
    namespace?: string
  }>
}

export interface DiagramMetadata {
  id: string
  name: string
  createdAt: string
  updatedAt: string
  author: string
  codeGenConfig: CodeGenConfig
}

export interface DiagramFile {
  version: string
  metadata: DiagramMetadata
  lifelines: Lifeline[]
  messages: Message[]
  combinedFragments: CombinedFragment[]
  viewState: ViewState
}

export type ToolType =
  | 'select'
  | 'lifeline'
  | 'sync-message'
  | 'async-message'
  | 'return-message'
  | 'create-message'
  | 'destroy-message'
  | 'alt'
  | 'loop'
  | 'opt'
  | 'par'
  | 'break'
  | 'delete'

export type ElementType = 'lifeline' | 'message' | 'fragment' | null
