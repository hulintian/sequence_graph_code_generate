# 基于 SysML 顺序图的多语言代码生成平台技术研究报告

## 一、项目概述
本项目旨在构建一个从 SysML 顺序图（XMI/XML）到多语言代码（以 C++ 为主）的自动化生成平台。系统通过解析顺序图结构，构建语义模型，并基于 Xtend 模板生成可编译代码，同时支持增量更新与用户代码保护。

---

## 二、总体技术路线
系统采用“解析 → 建模 → 生成 → 合并”的技术路线：

```
SysML XMI/XML
      ↓
解析器（Parser）
      ↓
语义模型（IR）
      ↓
Xtend模板引擎
      ↓
代码生成（C++/Java/Python）
      ↓
增量合并（Protected Region）
```

---

## 三、系统架构设计

### 3.1 分层架构

```
UI层（Tauri + Vue）
        ↓
CLI接口层
        ↓
核心引擎层（Parser + IR + Generator + Merge）
        ↓
模板层（Xtend Templates）
```

---

## 四、核心模块设计

### 4.1 Parser模块
负责解析XMI/XML：
- Lifeline解析
- Message解析
- CombinedFragment解析（alt/loop/opt）

---

### 4.2 语义模型（IR）

```java
interface Statement {}

class CallStatement implements Statement {}
class IfStatement implements Statement {}
class LoopStatement implements Statement {}
```

---

### 4.3 代码生成模块

使用Xtend模板：
- 类模板
- 方法模板
- 语句模板

---

### 4.4 增量合并模块

保护区机制：

```cpp
// <user-code-begin id>
// 用户代码
// <user-code-end id>
```

---

## 五、详细类设计

### 5.1 核心类结构

```
SequenceDiagramModel
 ├── LifelineModel
 ├── MessageModel
 └── CombinedFragmentModel

BehaviorIR
 ├── Statement
 │    ├── CallStatement
 │    ├── IfStatement
 │    └── LoopStatement
```

---

## 六、包结构设计

```
core/
  parser/
  semantic/
  generator/
  merge/
cli/
ui/
templates/
```

---

## 七、多语言代码生成设计

统一IR + 多模板：

```
IR → C++模板 → C++
IR → Java模板 → Java
IR → Python模板 → Python
```

---

## 八、流程图（代码生成流程）

```
[读取XMI]
   ↓
[解析顺序图]
   ↓
[构建IR]
   ↓
[选择模板语言]
   ↓
[生成代码]
   ↓
[合并用户代码]
   ↓
[输出文件]
```

---

## 九、测试大纲

### 9.1 功能测试
- 顺序图解析测试
- 控制结构生成测试

### 9.2 增量更新测试
- 用户代码保护测试
- 再生成一致性测试

### 9.3 编译测试
- C++代码编译通过

---

## 十、测试用例示例

### 用例1：简单调用
输入：A→B
输出：A调用B方法

### 用例2：条件分支
输入：alt
输出：if/else结构

### 用例3：循环
输入：loop
输出：while结构

---

## 十一、关键技术点

- 模型驱动开发（MDD）
- Xtend模板生成
- 保护区增量更新
- 多语言扩展机制

---

## 十二、结论

本系统实现了从SysML顺序图到多语言代码的自动化生成，并具备良好的扩展性与工程实用性。

