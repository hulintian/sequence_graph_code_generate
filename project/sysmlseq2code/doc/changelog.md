# 修改记录

> 本文档记录项目的所有重要修改，按时间倒序排列。

---

## [2026-03-28] 消息绘制位置跟随鼠标

### 修改
- **消息绘制位置**: 拖拽画消息线时，预览线和最终创建的消息都在鼠标按下的 Y 坐标位置，不再固定在底部自动排列
- `CanvasArea.vue`: 预览线 Y 改用 `messageDrawStart.y`；创建后设置 `customY = drawY`

---

## [2026-03-28] 片段几何存储 + 激活条重写 + ALT分隔线拖拽

### 新增
- **片段存储几何信息**: `CombinedFragment` 新增 `x/y/width/height/dividerRatio` 字段，创建时从拖拽矩形获取初始尺寸
- **片段拖拽调整大小**: 选中片段后可拖拽四条边 (N/S/E/W) 进行 resize，最小尺寸 60px
- **ALT 分隔线可拖拽**: if/else 分隔线位置由 `dividerRatio` (0.1~0.9) 控制，拖拽实时更新
- **消息自动归属片段**: 在片段框内绘制消息时自动设置 `parentFragmentId`，并添加到第一个 operand
- 片段选中时显示边缘热区 (8px)，光标变为 resize 方向指示

### 修改
- **激活条逻辑重写**: 纯粹基于调用-返回配对，不再受"下一个调用"影响
  - 收到非 return 消息时开始激活
  - 该生命线向调用者发送 return 消息时结束
  - 中间有外发调用时延伸覆盖
- `types/diagram.ts`: `CombinedFragment` 新增 `x, y, width, height, dividerRatio` 字段
- `stores/diagram.ts`: `addCombinedFragment` 接受可选 `rect` 参数
- `CanvasArea.vue`: 片段布局改用存储几何，移除动态计算；新增 resize/divider 拖拽状态机
- `FragmentRenderer.vue`: 改为从 `fragment` 直接读取 x/y/width/height，不再接收 baseY/height/minX/maxX props；ALT 分隔线带透明宽热区

---

## [2026-03-28] 消息拖拽 + 激活条 + 消息重排序

### 新增
- **消息垂直拖拽**: 消息支持 `customY` 字段，拖拽可在任意位置放置
- **激活条 (执行规格)**: 生命线上显示矩形激活条，表示执行期
- **消息重排序**: 属性面板中上/下按钮调整消息顺序 (`moveMessageOrder`)
- **消息源/目标编辑**: 属性面板中可通过下拉框更换消息的源/目标生命线

### 修改
- `types/diagram.ts`: `Message` 新增 `customY: number | null`
- `stores/diagram.ts`: 新增 `moveMessageOrder` 方法
- `CanvasArea.vue`: 新增消息拖拽状态机、`activationBars` 计算属性
- `MessageProps.vue`: 源/目标改为 `<select>` 下拉框，新增排序按钮

---

## [2026-03-28] 组合片段绘制 + 生命线命名修复

### 新增
- **组合片段拖拽创建**: 选择 alt/loop/opt/par 工具后在画布上拖拽矩形，框选消息自动创建片段
- **ALT else 分支消息管理**: FragmentProps 中 "移入消息..." 下拉可将消息在 operand 间移动

### 修复
- **生命线命名碰撞**: 删除后重新创建不再出现重名 (独立计数器 `lifelineCounter`)

---

## [2026-03-28] 代码生成引擎 (Part 2~5)

### 新增
- **IR 数据结构** (`src-tauri/src/ir.rs`): BehaviorIR → ClassModel → MethodModel → Statement enum
- **图表→IR 解析器** (`src-tauri/src/parser.rs`): 按 orderIndex 排序消息，按 target 分组为方法，递归处理 fragment
- **MiniJinja 模板渲染** (`src-tauri/src/generator.rs`): 编译时嵌入模板，生成 .h/.cpp 文件
- **用户代码保护区** (`src-tauri/src/protected_region.rs`): `// <user-code-begin ID>` 标记的提取与注入
- **增量写入** (`src-tauri/src/incremental.rs`): SHA256 hash 比对，按需写入 (Created/Updated/Unchanged/MergedUserCode)
- **Tauri 命令**: `generate_code` (写磁盘) + `preview_code` (仅返回内容)
- **C++ 模板**: `file_unit.h.j2`, `file_unit.cpp.j2`, `statements.j2`
- **前端对接**: CodeGenConfig.vue 启用 "生成代码" 和 "预览" 按钮，预览弹窗

### 依赖
- `minijinja = { version = "2", features = ["builtins"] }`
- `sha2 = "0.10"`

---

## [2026-03-28] 全局缩放功能

### 新增
- **全局 UI 缩放**: `Ctrl+Shift +/-` 可缩放整个界面 (菜单栏、工具栏、属性面板等全部一起缩放)
- **Tauri 原生缩放**: Tauri 环境使用 `WebviewWindow.setZoom()` API，Web 环境回退到 CSS zoom
- Store 新增 `appZoom` 状态和 `setAppZoom()` 方法
- 状态栏分别显示「画布缩放」和「全局缩放」百分比
- Tauri 权限新增 `core:webview:allow-set-webview-zoom`

### 修改
- `CanvasArea.vue`: 键盘快捷键区分 `Ctrl`(画布) 和 `Ctrl+Shift`(全局)
- `App.vue`: 添加 appZoom watcher，根据运行环境选择缩放方式
- `StatusBar.vue`: 显示两个缩放指标
- `stores/diagram.ts`: 新增 `appZoom` ref 和 `setAppZoom` action

### 快捷键变更

| 快捷键 | 原功能 | 新功能 |
|--------|--------|--------|
| `Ctrl + =/-` | 缩放 (含 Shift) | 仅画布缩放 |
| `Ctrl + 0` | 重置缩放 | 仅重置画布缩放 |
| `Ctrl + Shift + =/-` | (无) | 全局 UI 缩放 |
| `Ctrl + Shift + 0` | (无) | 重置全局缩放 |

---

## [2026-03-28 之前] 初始版本

### Git 提交记录

- `4f1f5ee` - update gitignore
- `47b9595` - add the sysml research doc, and sysml SD doc, and the tauri project
- `7eae26a` - init

### 初始功能清单

#### 绘图编辑器
- SVG 画布，支持平移 (中键/Shift+拖拽) 和缩放 (Ctrl+滚轮)
- 生命线: 创建、拖动 (水平网格对齐)、选择、属性编辑、删除
  - 类型: class / actor / component / interface
  - 属性: name, stereotype, namespace, attributes
- 消息: 在生命线之间绘制、支持自调用
  - 类型: sync (实心箭头) / async (空心箭头) / return (虚线)
  - 属性: name, arguments, returnType, guard
- 组合片段: alt (含 else) / loop / opt / par / break
  - 属性: guard conditions, operands
  - 基于包含消息自动计算布局
- 网格背景 (可切换显示/隐藏)

#### 状态管理
- Pinia 集中式状态管理
- 完整的撤销/重做系统 (深拷贝快照)
- 脏标记 (isDirty) 追踪未保存修改

#### 文件操作
- 保存/加载 JSON 格式图表文件 (`.seqd.json`)
- Rust 后端实现原子写入 + 自动备份 (.bak)
- 新建图表 (重置所有状态)

#### UI 布局
- 深色主题 (#1e1f22)
- 菜单栏: 文件/编辑/视图/代码生成 (代码生成菜单项 disabled)
- 工具面板: 操作工具 + 元素工具 + 片段工具
- 属性面板: 根据选中元素类型动态显示
- 状态栏: 文件名、修改状态、缩放、元素数量、当前工具
- 代码生成配置面板 (UI 已实现，按钮 disabled)

#### 键盘快捷键
- `Escape`: 取消操作/清除选择
- `Delete / Backspace`: 删除选中元素
- `Ctrl+Z / Ctrl+Y`: 撤销/重做

#### Tauri 桌面集成
- 窗口: 1280x800，最小 900x600
- 标题: "SysML 顺序图代码生成平台"
- IPC 命令: save_diagram, load_diagram
