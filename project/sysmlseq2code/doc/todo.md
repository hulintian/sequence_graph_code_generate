# TODO / 进度文档

> 最后更新: 2026-03-28
> 项目截止日期: 2026-04-30 (参见 `notes/sysml_full_report.md`)

## 总体进度概览

根据 `notes/detailed_design.md` 的五部分设计和 `notes/sysml_full_report.md` 的技术要求，当前实现进度如下:

| 模块 | 设计文档 | 状态 | 完成度 |
|------|----------|------|--------|
| Part 1: 绘图工具 | `detailed_design.md` Part 1 | **基本完成** | ~95% |
| Part 2: 代码生成引擎 | `detailed_design.md` Part 2 | **已完成** | ~95% |
| Part 3: 用户代码保护区 | `detailed_design.md` Part 3 | **已完成** | ~100% |
| Part 4: 增量更新 | `detailed_design.md` Part 4 | **已完成** | ~90% |
| Part 5: 生成路径配置 | `detailed_design.md` Part 5 | **基本完成** | ~70% |

---

## Part 1: 绘图工具

### 已完成

- [x] JSON 持久化格式 (`.seqd.json`)，与设计文档数据结构基本一致
- [x] SVG 画布交互 (平移、缩放、网格)
- [x] 生命线: 创建、拖动、选择、属性编辑、删除
- [x] 消息: sync/async/return 类型绘制，自调用支持
- [x] 组合片段: alt/loop/opt/par 拖拽创建与渲染
- [x] 撤销/重做系统
- [x] Tauri 文件保存/加载 (原子写入 + 备份)
- [x] 属性面板: 生命线/消息/片段属性编辑
- [x] 全局缩放 (Ctrl+Shift+/-) 与画布缩放 (Ctrl+/-) 分离
- [x] 消息重排序 (属性面板上/下按钮)
- [x] 消息垂直拖拽 (customY 任意位置)
- [x] 消息源/目标编辑 (属性面板下拉框)
- [x] 执行规格 (激活条) — 基于调用-返回配对
- [x] 片段存储几何信息 (x/y/width/height)，可拖拽边缘调整大小
- [x] 片段内/外消息自动归属 (基于几何包含)
- [x] ALT if/else 分隔线可拖拽 (dividerRatio)
- [x] ALT else 分支消息管理 ("移入消息..." 下拉)
- [x] Fragment guard 条件自定义 (用于生成 if/while 条件)

### 待完成

- [ ] **XMI 导入/导出** — 菜单项已存在但 disabled；设计文档和研究文档有详细 XMI 格式规范
- [ ] **消息类型补全** — 类型定义中有 `create` / `destroy` 但画布工具栏缺少对应工具，渲染器未处理这两种类型
- [ ] **组合片段 break 类型** — 类型定义中有 `break` 但工具栏未暴露
- [ ] **片段嵌套** — 数据模型支持 `parentFragmentId`，但 UI 上创建嵌套片段的交互未完善
- [ ] **消息参数编辑优化** — 当前为文本输入 (`name:type, ...`)，可改为结构化编辑

---

## Part 2: 代码生成引擎

> 参考: `notes/detailed_design.md` Part 2, 附录 IR 定义

### 设计要点 (来自设计文档)

- **IR 中间表示**: `BehaviorIR` → `ClassModel` → `MethodModel` → `Statement` 层级
- **模板引擎**: Jinja2 (Rust 端用 MiniJinja)
- **最小模板单元**: CallUnit, IfUnit, LoopUnit, MethodImplUnit, FileUnit
- **映射关系**:
  - Lifeline → Class 定义
  - sync message → 方法调用
  - async message → 异步调用/事件
  - alt fragment → if-else
  - loop fragment → while/for
  - par fragment → 并行/线程

### 已完成

- [x] **IR 数据结构定义** (`src-tauri/src/ir.rs`): BehaviorIR → ClassModel → MethodModel → Statement enum
- [x] **图表→IR 转换器** (`src-tauri/src/parser.rs`): 按 orderIndex 排序，按 target 分组，递归处理 fragment
- [x] **模板系统** (`src-tauri/src/generator.rs`): MiniJinja 集成，C++ 模板集
- [x] **IR→代码生成器**: 遍历 IR 调用模板渲染 .h/.cpp
- [x] **生成按钮对接** (CodeGenConfig.vue 的 "生成代码" 按钮)
- [x] **生成预览** (CodeGenConfig.vue 的 "预览" 按钮 + 弹窗)

### 待完成

- [ ] **多语言支持** (Java, Python 模板集 — 目前只有 C++)

---

## Part 3: 用户代码保护区 (Protected Region)

> 参考: `notes/detailed_design.md` Part 3

### 设计要点

- 标记格式: `// <user-code-begin {uniqueID}>` ... `// <user-code-end {uniqueID}>`
- ID 命名: `{className}_{methodName}_{position}`
- position 包括: `pre`, `post`, `alt_{guard}_body`, `loop_body` 等
- 提取+注入算法: 正则匹配提取旧文件用户代码块，注入到新生成代码对应位置

### 已完成

- [x] **保护区标记生成** (模板中 `// <user-code-begin ID>` / `// <user-code-end ID>`)
- [x] **用户代码提取器** (`src-tauri/src/protected_region.rs`: `extract_regions()`)
- [x] **用户代码注入器** (`merge_regions()`: 将提取的内容合并到新生成的文件)

---

## Part 4: 增量更新

> 参考: `notes/detailed_design.md` Part 4

### 设计要点

- 快照对比: `oldVersionDir` 保存上次生成的快照
- `.codegen_manifest.json`: 记录每个文件的 hash、生成时间
- 基于 hash 比对决定: 跳过/覆盖/合并

### 已完成

- [x] **Hash 比对引擎** (`src-tauri/src/incremental.rs`: SHA256 比对新旧文件)
- [x] **IncrementalUpdater** (集成保护区提取 + hash 比对 + 合并写入)
- [x] **WriteResult 报告** (Created / Updated / Unchanged / MergedUserCode)

### 待完成

- [ ] **Manifest 文件管理** (生成时创建/更新 `.codegen_manifest.json`)
- [ ] **快照目录管理** (生成前保存旧版本到 `oldVersionDir`)

---

## Part 5: 代码生成路径配置

> 参考: `notes/detailed_design.md` Part 5

### 已完成

- [x] `CodeGenConfig` 数据结构 (outputDir, language, templateSet, namespace, oldVersionDir, lifelineOverrides)
- [x] `CodeGenConfig.vue` 配置界面 (输出目录、语言、模板集、命名空间)
- [x] 配置嵌入 DiagramMetadata 中，随图表保存

### 已完成

- [x] **生成触发流程** (按钮 → 读取配置 → invoke generate_code → 写入文件 → 显示结果)

### 待完成

- [ ] **OutputPathResolver** (按优先级解析: lifeline override > global > default)
- [ ] **Lifeline 独立路径覆盖 UI** (LifelineProps 中添加路径覆盖配置)

---

## 技术要求对照 (来自 `notes/sysml_full_report.md`)

| 技术要求 | 状态 | 备注 |
|----------|------|------|
| 解析 SysML 顺序图 (XMI/XML) | **部分** | 自定义 JSON 格式已完成，XMI 导入待实现 |
| 提取逻辑和控制结构 | **已完成** | parser.rs: 图表→IR Statement 树 |
| 生成 C++ 代码 | **已完成** | generator.rs + MiniJinja 模板 |
| 支持手动代码补充 | **已完成** | protected_region.rs: 保护区提取/注入 |
| 支持增量修改与代码保护 | **已完成** | incremental.rs: hash 比对 + 合并写入 |
| C++17，控制结构支持 | **已完成** | 模板支持 if/else/while/for，可扩展 |
| clang-format 兼容 | **未开始** | 生成后格式化 |

## 交付物对照 (来自 `notes/sysml_full_report.md`)

| 交付物 | 状态 | 备注 |
|--------|------|------|
| 技术研究报告 | **已完成** | `notes/sequence_diagram_research_report.md`, `notes/sysml_reference.md` |
| 源代码集成文档 | **进行中** | `doc/code_structure.md` (本文档) |
| 测试大纲 | **未开始** | - |
| 测试报告 | **未开始** | - |
| 项目验收总结报告 | **未开始** | - |

---

## 优先级建议

考虑到截止日期 (2026-04-30)，建议按以下顺序推进:

1. **[高] Part 2: 代码生成引擎** — 核心功能，是所有后续部分的基础
2. **[高] Part 3: 用户代码保护区** — 技术要求中的必选项
3. **[中] Part 4: 增量更新** — 技术要求中的必选项
4. **[中] Part 1 补全** — XMI 导入、缺失的消息/片段类型
5. **[低] Part 5 完善** — 路径配置是锦上添花
6. **[高] 测试与文档** — 测试大纲和报告是交付物
