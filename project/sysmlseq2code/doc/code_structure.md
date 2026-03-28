# Tauri 代码结构文档

> 最后更新: 2026-03-28 (v2)

## 1. 技术栈

| 层级 | 技术 | 版本 |
|------|------|------|
| 前端框架 | Vue 3 (Composition API + `<script setup>`) | 3.5.13 |
| 状态管理 | Pinia | 3.0.4 |
| 类型系统 | TypeScript | 5.6 |
| 构建工具 | Vite | 6.x |
| 桌面框架 | Tauri 2 | 2.x |
| 后端语言 | Rust | 2021 edition |
| 序列化 | serde + serde_json | 1.x |
| 包管理 | pnpm | - |

## 2. 目录结构

```
sysmlseq2code/
├── src/                              # Vue 前端源码
│   ├── main.ts                       # 应用入口，初始化 Vue + Pinia
│   ├── App.vue                       # 根组件，整体布局 + 全局缩放
│   ├── vite-env.d.ts                 # Vite 环境类型声明
│   ├── types/
│   │   └── diagram.ts                # 所有 TypeScript 接口/类型定义
│   ├── stores/
│   │   └── diagram.ts                # Pinia 全局状态管理 (~350 行)
│   ├── components/
│   │   ├── MenuBar.vue               # 顶部菜单栏 (文件/编辑/视图/代码生成)
│   │   ├── ToolPanel.vue             # 左侧工具面板 (工具选择)
│   │   ├── CanvasArea.vue            # 主画布区域 (SVG 编辑器核心, ~580 行)
│   │   ├── PropertyPanel.vue         # 右侧属性面板 (条件渲染)
│   │   ├── StatusBar.vue             # 底部状态栏
│   │   ├── canvas/                   # 画布渲染子组件
│   │   │   ├── GridBackground.vue    #   网格背景 (SVG pattern)
│   │   │   ├── LifelineRenderer.vue  #   生命线渲染
│   │   │   ├── MessageRenderer.vue   #   消息/箭头渲染
│   │   │   └── FragmentRenderer.vue  #   组合片段渲染
│   │   └── properties/               # 属性编辑子组件
│   │       ├── LifelineProps.vue     #   生命线属性
│   │       ├── MessageProps.vue      #   消息属性
│   │       ├── FragmentProps.vue     #   片段属性
│   │       └── CodeGenConfig.vue     #   代码生成配置
│   └── assets/                       # 静态资源
├── src-tauri/                        # Rust 后端
│   ├── src/
│   │   ├── main.rs                   # Tauri 启动入口
│   │   ├── lib.rs                    # Tauri 命令 (save/load/generate/preview)
│   │   ├── ir.rs                     # IR 数据结构 (BehaviorIR/ClassModel/Statement)
│   │   ├── parser.rs                 # 图表 JSON → IR 转换器
│   │   ├── generator.rs              # MiniJinja 模板渲染引擎
│   │   ├── protected_region.rs       # 用户代码保护区 提取/注入
│   │   └── incremental.rs            # 增量写入 (hash 比对)
│   ├── Cargo.toml                    # Rust 依赖 (含 minijinja, sha2)
│   ├── build.rs                      # 构建脚本
│   ├── tauri.conf.json               # Tauri 应用配置
│   ├── templates/                    # C++ 代码生成模板
│   │   └── cpp/
│   │       ├── file_unit.h.j2        #   头文件模板
│   │       ├── file_unit.cpp.j2      #   实现文件模板
│   │       └── statements.j2         #   语句递归宏 (render_stmt)
│   └── capabilities/
│       └── default.json              # 权限配置
├── doc/                              # 项目文档 (本目录)
├── package.json                      # Node 依赖
├── tsconfig.json                     # TypeScript 配置
├── vite.config.ts                    # Vite 构建配置
└── index.html                        # HTML 入口
```

## 3. 架构概览

```
┌──────────────────────────────────────────────────────────┐
│                      App.vue (根布局)                      │
│  ┌────────────────────────────────────────────────────┐  │
│  │                  MenuBar.vue                        │  │
│  ├──────────┬─────────────────────┬───────────────────┤  │
│  │ToolPanel │    CanvasArea.vue   │  PropertyPanel.vue│  │
│  │  .vue    │  ┌───────────────┐  │  ┌─────────────┐ │  │
│  │          │  │GridBackground │  │  │LifelineProps│ │  │
│  │  工具选择  │  │LifelineRender│  │  │MessageProps │ │  │
│  │  撤销/重做 │  │MessageRender │  │  │FragmentProps│ │  │
│  │          │  │FragmentRender│  │  │CodeGenConfig│ │  │
│  │          │  └───────────────┘  │  └─────────────┘ │  │
│  ├──────────┴─────────────────────┴───────────────────┤  │
│  │                  StatusBar.vue                      │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
                           │
                     Pinia Store
                    (diagram.ts)
                           │
                    Tauri IPC (invoke)
                           │
              ┌────────────┴────────────┐
              │    Rust Backend          │
              │  save_diagram (原子写入)   │
              │  load_diagram (JSON验证)  │
              └─────────────────────────┘
```

## 4. 类型系统 (`src/types/diagram.ts`)

### 核心数据模型

```
DiagramFile (顶层序列化格式)
├── version: "1.0.0"
├── metadata: DiagramMetadata
│   ├── id, name, createdAt, updatedAt, author
│   └── codeGenConfig: CodeGenConfig
├── lifelines: Lifeline[]
│   ├── id, name
│   ├── type: 'class' | 'actor' | 'component' | 'interface'
│   ├── position: { x, y }
│   └── properties: { stereotype, attributes[], namespace }
├── messages: Message[]
│   ├── id, name, orderIndex
│   ├── type: 'sync' | 'async' | 'return' | 'create' | 'destroy'
│   ├── sourceLifelineId, targetLifelineId
│   ├── arguments: MessageArgument[]
│   └── returnType, guard, parentFragmentId
├── combinedFragments: CombinedFragment[]
│   ├── id
│   ├── type: 'alt' | 'loop' | 'opt' | 'par' | 'break'
│   ├── operands: FragmentOperand[] (guard + messageIds)
│   ├── parentFragmentId
│   ├── x, y, width, height (存储几何，可拖拽 resize)
│   └── dividerRatio (ALT if/else 分隔线位置, 0~1)
└── viewState: ViewState
    ├── zoom (0.25 ~ 3.0), panX, panY
    ├── gridEnabled, gridSize
    └── (appZoom 仅运行时使用，不序列化)
```

### 工具类型

```
ToolType = 'select' | 'lifeline' | 'sync-message' | 'async-message'
         | 'return-message' | 'alt' | 'loop' | 'opt' | 'par' | 'delete'
```

## 5. 状态管理 (`src/stores/diagram.ts`)

### 响应式状态

| 状态 | 类型 | 说明 |
|------|------|------|
| `metadata` | `DiagramMetadata` | 图表元信息 |
| `lifelines` | `Lifeline[]` | 所有生命线 |
| `messages` | `Message[]` | 所有消息 |
| `combinedFragments` | `CombinedFragment[]` | 所有组合片段 |
| `viewState` | `ViewState` | 缩放/平移/网格 |
| `appZoom` | `number` | 全局 UI 缩放 (0.25~3) |
| `selectedElementId` | `string \| null` | 选中元素 ID |
| `selectedElementType` | `ElementType` | 选中元素类型 |
| `activeTool` | `ToolType` | 当前工具 |
| `isDirty` | `boolean` | 是否有未保存修改 |
| `currentFilePath` | `string \| null` | 当前文件路径 |
| `undoStack / redoStack` | `DiagramSnapshot[]` | 撤销/重做快照栈 |

### 核心方法

- **元素操作**: `addLifeline`, `addMessage`, `addCombinedFragment`, `updateLifeline/Message/Fragment`, `deleteSelected`
- **选择**: `selectElement`, `clearSelection`
- **视图**: `setZoom`, `setAppZoom`, `setPan`, `moveLifeline`
- **撤销/重做**: `undo`, `redo` (基于深拷贝快照)
- **文件**: `toJSON`, `loadFromJSON`, `newDiagram`

## 6. 画布交互 (`CanvasArea.vue`)

### SVG 渲染管线

```svg
<svg viewBox="0 0 2000 1200">
  <g transform="translate(panX,panY) scale(zoom)">
    <GridBackground />                    <!-- 网格背景 -->
    <FragmentRenderer v-for />            <!-- 组合片段 (可 resize, ALT 分隔线可拖) -->
    <LifelineRenderer v-for />            <!-- 生命线 (头部+虚线) -->
    <rect v-for="activationBars" />       <!-- 激活条 (调用-返回配对) -->
    <defs> arrow markers </defs>          <!-- 箭头标记定义 -->
    <MessageRenderer v-for />             <!-- 消息箭头+标签 (可垂直拖拽) -->
    <line v-if="drawing" />               <!-- 绘制预览线 -->
    <rect v-if="drawingFragment" />       <!-- 片段选择框预览 -->
  </g>
</svg>
```

### 坐标转换

`getSvgPoint(e)` 将鼠标屏幕坐标转换为图表坐标：
1. 通过 `svg.getScreenCTM().inverse()` 转换到 SVG 坐标
2. 减去平移偏移 `(panX, panY)`
3. 除以缩放系数 `zoom`

### 交互模式

| 操作 | 触发方式 | 功能 |
|------|----------|------|
| 平移画布 | 鼠标中键 / Shift+左键拖拽 | 移动视口 |
| 画布缩放 | Ctrl + 滚轮 / Ctrl +/- | 缩放 SVG 内容 |
| 全局缩放 | Ctrl+Shift +/- | 缩放整个 UI |
| 放置生命线 | 选择生命线工具后单击画布 | 创建新生命线 (网格对齐) |
| 绘制消息 | 选择消息工具后从源拖到目标 | 在鼠标按下的 Y 位置创建消息 (自动检测片段归属) |
| 拖动生命线 | 左键拖动生命线头部 | 水平移动 (网格对齐) |
| 拖动消息 | 左键拖动消息线 | 垂直移动 (customY) |
| 创建片段 | 选择 alt/loop/opt/par 工具后拖拽矩形 | 框选消息创建组合片段 |
| 调整片段大小 | 拖拽片段四条边 (N/S/E/W) | Resize 片段框 |
| 拖动 ALT 分隔线 | 拖拽 alt 片段内虚线 | 调整 if/else 空间分配 |
| 选择元素 | 单击元素 | 选中并显示属性 |
| 删除元素 | Delete / Backspace | 删除选中元素 |
| 撤销/重做 | Ctrl+Z / Ctrl+Y | 操作历史 |

### 布局常量

- 生命线头部 Y 起点: `baseY = 140px`
- 消息间距: `messageSpacing = 50px`
- 网格大小: `gridSize = 20px` (默认)

## 7. Rust 后端 (`src-tauri/src/`)

### Tauri 命令 (`lib.rs`)

| 命令 | 参数 | 功能 |
|------|------|------|
| `save_diagram` | `path, content` | 原子写入图表 JSON (.tmp → rename, .bak 备份) |
| `load_diagram` | `path` | 读取并验证 JSON 文件 |
| `generate_code` | `diagram_json, output_dir, old_version_dir` | 完整代码生成管线 → 写入磁盘 → 返回结果 |
| `preview_code` | `diagram_json` | 代码生成管线 → 返回文件内容 (不写磁盘) |

### 代码生成管线

```
DiagramFile JSON → parser.rs → BehaviorIR → generator.rs (MiniJinja) → C++ Files
                                                   ↓
                                     protected_region.rs (merge)
                                                   ↓
                                      incremental.rs (hash 比对写入)
```

### 模块说明

| 模块 | 文件 | 功能 |
|------|------|------|
| IR | `ir.rs` | BehaviorIR, ClassModel, MethodModel, Parameter, Statement enum (Call/If/Loop/Opt/Return) |
| 解析器 | `parser.rs` | 按 orderIndex 排序消息 → 按 target 分组为方法 → 递归处理 fragment → Statement 树 |
| 生成器 | `generator.rs` | MiniJinja 模板加载 (编译时嵌入) → 渲染 .h + .cpp per class |
| 保护区 | `protected_region.rs` | `extract_regions()` 提取旧文件用户代码, `merge_regions()` 注入新文件 |
| 增量 | `incremental.rs` | SHA256 hash 比对 → Created/Updated/Unchanged/MergedUserCode |

### 安全机制

- 写入前自动创建父目录
- 覆盖前创建 `.bak` 备份文件
- 原子写入: 先写 `.tmp` 文件，成功后 rename 到目标路径
- 加载时验证 JSON 格式合法性
- 增量写入: 只在内容变化时覆盖

## 8. 权限配置 (`capabilities/default.json`)

```json
{
  "permissions": [
    "core:default",
    "core:webview:allow-set-webview-zoom",
    "opener:default"
  ]
}
```

- `core:default`: Tauri 核心功能 (IPC invoke 等)
- `core:webview:allow-set-webview-zoom`: 允许 JS 调用 webview 原生缩放
- `opener:default`: 文件打开对话框

## 9. 构建与运行

```bash
# 安装依赖
pnpm install

# 开发模式 (Vite + Tauri)
pnpm tauri dev

# 仅前端开发
pnpm dev

# 类型检查
pnpm vue-tsc --noEmit

# 构建生产包
pnpm tauri build
```

Vite 开发服务器运行在 `http://localhost:1420`，Tauri 通过 `beforeDevCommand` 自动启动。
