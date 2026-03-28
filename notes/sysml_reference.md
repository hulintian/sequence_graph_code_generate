# SysML (Systems Modeling Language) 详细参考文档

## 目录

1. [概述](#1-概述)
2. [SysML v1.x 九种图类型](#2-sysml-v1x-九种图类型)
3. [SysML v2](#3-sysml-v2)
4. [核心概念](#4-核心概念)
5. [SysML vs UML](#5-sysml-vs-uml)
6. [工具支持](#6-工具支持)
7. [MBSE 与 SysML](#7-mbse-与-sysml)
8. [行业应用模式](#8-行业应用模式)
9. [SysML v2 文本化语法](#9-sysml-v2-文本化语法)
10. [与其他标准的集成](#10-与其他标准的集成)
11. [参考资料](#11-参考资料)

---

## 1. 概述

### 1.1 什么是 SysML

SysML（Systems Modeling Language）是一种通用的系统架构建模语言，用于系统工程应用。它支持对各类系统及系统之系统（systems-of-systems）的规约（specification）、分析（analysis）、设计（design）、验证（verification）和确认（validation），涵盖硬件、软件、信息、流程、人员和设施等。

### 1.2 发展历史

| 时间 | 事件 |
|------|------|
| 2001年1月 | INCOSE（国际系统工程协会）模型驱动系统设计工作组决定为系统工程定制 UML |
| 2001年7月 | INCOSE 与 OMG 联合成立 SE DSIG（系统工程领域特定兴趣组） |
| 2003年3月 | OMG 发布 "UML for Systems Engineering" RFP；SysML Partners 开源规范项目启动 |
| 2006年7月 | OMG 正式宣布采纳 SysML |
| 2007年9月 | SysML v1.0 发布 |
| 2007-2019 | 相继发布 v1.1 至 v1.7，持续完善规范 |
| 2023年6月 | OMG 采纳 KerML、SysML v2 和 Systems Modeling API 的 Beta 规范 |
| 2025年7月21日 | OMG 正式批准 SysML v2.0、KerML v1.0 和 Systems Modeling API and Services v1.0 |
| 2025年9月3日 | SysML v2 作为新标准正式发布 |

### 1.3 与 UML 的关系

SysML 定义为 UML 2 子集的扩展，使用 UML 的 Profile 机制（构造型、标记值、约束）实现。它复用了 UML 2 的 14 种图中的 7 种，并新增 2 种图（需求图和参数图），共计 9 种图。SysML 移除了不适用于系统工程的软件中心化构件（如类图被块定义图取代，对象图被内部块图取代）。

> **注**: SysML v2 不再基于 UML Profile，而是构建在全新的 KerML（Kernel Modeling Language）之上，成为独立的建模语言。

### 1.4 治理机构

- **OMG（Object Management Group）**：维护 SysML 规范
- **INCOSE**：密切协作，推动 MBSE 和 SysML 的应用

### 1.5 目标与用途

- 为系统工程提供标准建模语言
- 支持 MBSE（基于模型的系统工程）
- 支持复杂系统的规约、分析、设计、V&V
- 桥接硬件、软件、信息、流程、人员和设施的建模
- 改善跨工程学科的沟通

---

## 2. SysML v1.x 九种图类型

SysML v1.x 分为 **结构图（Structure Diagrams）** 和 **行为图（Behavior Diagrams）** 两大类：

```
SysML 图
├── 结构图
│   ├── 块定义图 (bdd)
│   ├── 内部块图 (ibd)
│   ├── 包图 (pkg)
│   └── 参数图 (par)
└── 行为图
    ├── 活动图 (act)
    ├── 顺序图 (sd)
    ├── 状态机图 (stm)
    └── 用例图 (uc)
（跨领域图）
    └── 需求图 (req)
```

### 2.1 块定义图 (Block Definition Diagram, bdd)

**用途**：定义系统结构，展示块（系统组件）及其属性、关系（关联、泛化、组合、聚合）和操作。

**核心元素**：
- **Block（块）**：SysML 的基本结构单元，对应 UML 的 Class
- **Value Type（值类型）**：定义属性的数据类型（含单位和量纲）
- **Constraint Block（约束块）**：定义参数化约束
- **Flow Specification（流规约）**：定义流经端口的内容
- 关系：泛化（Generalization）、关联（Association）、组合（Composition）、聚合（Aggregation）

**符号说明**：
```
┌──────────────────────────┐
│       <<block>>          │
│        Vehicle           │
├──────────────────────────┤
│ values                   │
│   mass : kg              │
│   maxSpeed : km/h        │
├──────────────────────────┤
│ parts                    │
│   engine : Engine        │
│   transmission : Trans   │
├──────────────────────────┤
│ operations               │
│   start()                │
│   stop()                 │
└──────────────────────────┘
```

**使用场景**：系统架构早期定义、组件分类和层次结构、结构关系建模。

**示例**：Vehicle 块通过组合关系包含 Engine、Transmission 和 Chassis 块；Engine 泛化为 ElectricEngine 和 CombustionEngine。

---

### 2.2 内部块图 (Internal Block Diagram, ibd)

**用途**：展示块的内部结构——各部件如何通过端口和连接器相互连接（白盒视图）。

**核心元素**：
- **Part（部件）**：块的实例
- **Port（端口）**：Standard Port、Flow Port（in/out/inout）、Full Port
- **Connector（连接器）**：部件间的连接
- **Item Flow（项目流）**：流经连接器的具体内容

**符号说明**：
```
┌─────────────── Vehicle ──────────────────┐
│                                          │
│  ┌──────────┐          ┌──────────┐      │
│  │  engine   │──[fuel]──│ fuelTank │      │
│  │ :Engine   ◄─────────►│:FuelTank │      │
│  └─────┬────┘          └──────────┘      │
│        │ [torque]                         │
│  ┌─────▼──────────┐                      │
│  │  transmission   │                      │
│  │ :Transmission   │                      │
│  └────────────────┘                      │
└──────────────────────────────────────────┘
```

**使用场景**：定义组件间的接口和交互、展示数据/物质/能量流。

---

### 2.3 包图 (Package Diagram, pkg)

**用途**：将模型元素组织到包中，用于命名空间管理、模型组织和访问控制。

**核心元素**：
- **Package（包）**：模型元素的容器
- **Package Import（包导入）**：包之间的依赖
- **Model Library（模型库）**
- **Profile（剖面）**、**View（视图）**、**Viewpoint（视点）**

**使用场景**：组织大型模型为逻辑分组（如 Requirements、Structure、Behavior、Analysis 包）。

**示例**：
```
┌─── VehicleSystem ───┐
│                     │
│  ┌──────────┐       │
│  │Structure │       │
│  └──────────┘       │
│  ┌──────────┐       │
│  │Behavior  │       │
│  └──────────┘       │
│  ┌──────────────┐   │
│  │Requirements  │   │
│  └──────────────┘   │
│  ┌──────────┐       │
│  │Analysis  │       │
│  └──────────┘       │
└─────────────────────┘
```

---

### 2.4 参数图 (Parametric Diagram, par)

**用途**：表达绑定系统属性的数学约束和方程，支持工程分析（性能、可靠性、质量汇总等）。

**核心元素**：
- **Constraint Block（约束块）**：定义数学关系
- **Constraint Property（约束属性）**：约束块的实例
- **Constraint Parameter（约束参数）**：约束的输入输出
- **Binding Connector（绑定连接器）**：将约束参数绑定到块的值属性

**示例**：
```
┌─────────────────────────────┐
│   <<constraintBlock>>       │
│      NewtonsLaw             │
│                             │
│  {F = m * a}                │
│                             │
│  params:                    │
│    F : Force ◄──── Vehicle.totalForce
│    m : Mass  ◄──── Vehicle.mass
│    a : Accel ◄──── Vehicle.acceleration
└─────────────────────────────┘
```

**使用场景**：权衡研究、性能分析、集成分析模型（如 F=ma、功率方程、热传导）。

---

### 2.5 需求图 (Requirement Diagram, req)

**用途**：捕获基于文本的需求及其关系。

**核心元素**：
- **Requirement（需求）**：包含 id 和 text 属性
- **关系类型**：
  - `<<deriveReqt>>`：派生需求
  - `<<satisfy>>`：设计元素满足需求
  - `<<verify>>`：测试用例验证需求
  - `<<refine>>`：细化
  - `<<trace>>`：追踪
  - `<<copy>>`：复制
  - Containment：包含/分解

**符号说明**：
```
┌────────────────────────┐
│    <<requirement>>     │
│      MaxSpeed          │
│  id = R1               │
│  text = "车辆最高速度   │
│         应达到200km/h" │
└────────────┬───────────┘
             │ <<satisfy>>
┌────────────▼───────────┐
│       <<block>>        │
│       Vehicle          │
└────────────────────────┘
```

**使用场景**：捕获利益相关者和系统需求，链接到设计元素（satisfy）和测试用例（verify），实现需求追踪。

---

### 2.6 活动图 (Activity Diagram, act)

**用途**：将功能行为建模为动作流，包括控制流和对象流（数据/物质/能量流）。图灵完备，支持仿真。

**核心元素**：
- **Action（动作）**：Call Behavior、Call Operation、Send Signal、Accept Event
- **Object Node（对象节点）**：数据/物料的缓冲
- **Pin（引脚）**：动作的输入/输出
- **Control Node（控制节点）**：
  - Fork（分叉）/ Join（汇合）
  - Decision（决策）/ Merge（合并）
  - Initial（初始节点）/ Final（终止节点）
- **Activity Partition（活动分区/泳道）**
- **Interruptible Region（可中断区域）**

**使用场景**：功能分解、流程工作流、数据流建模、操作场景描述。

**示例**：
```
[●] ──► [StartEngine] ──► [Accelerate] ──► <Decision>
                                              │     │
                                         [stop?]  [else]
                                              │     │
                                          [Brake]  [Cruise]
                                              │        │
                                         [StopEngine]  └──► (回到Decision)
                                              │
                                             [◉]
```

---

### 2.7 顺序图 (Sequence Diagram, sd)

**用途**：展示部件/参与者之间按时间顺序的交互，通过生命线上交换的消息表示。图灵完备。

**核心元素**：
- **Lifeline（生命线）**：参与交互的对象
- **Message（消息）**：同步、异步、返回
- **Combined Fragment（组合片段）**：
  - `alt`：替代（条件分支）
  - `loop`：循环
  - `opt`：可选
  - `break`：中断
  - `par`：并行
  - `critical`：临界区
  - `neg`：无效交互
  - `assert`：断言
  - `seq`/`strict`：顺序约束
  - `ignore`/`consider`：过滤
- **Interaction Use（交互使用，ref）**
- **Gate（门）**
- **Execution Specification（执行规约）**

**符号说明**：
```
  User          VehicleCtrl       FuelSystem       Engine
   │                │                  │              │
   │ startEngine()  │                  │              │
   │───────────────►│                  │              │
   │                │  checkFuel()     │              │
   │                │─────────────────►│              │
   │                │                  │              │
   │        ┌───────┤ alt              │              │
   │        │[fuelOK]                  │              │
   │        │       │  ignite()        │              │
   │        │       │─────────────────────────────────►
   │        │       │                  │    running   │
   │        │       │◄─────────────────────────────────
   │        ├───────┤                  │              │
   │        │[fuelLow]                 │              │
   │        │       │  error           │              │
   │        │       │◄─────────────────│              │
   │        └───────┤                  │              │
   │                │                  │              │
```

**使用场景**：详细交互场景、协议规约、时序敏感行为、接口验证。

> **对本项目的重要性**：顺序图是本项目代码生成的核心输入，XMI/XML 格式的顺序图将被解析并转换为可编译代码。

---

### 2.8 状态机图 (State Machine Diagram, stm)

**用途**：对块的离散事件驱动行为进行建模，通过状态、转换、事件、守卫和动作描述。图灵完备。

**核心元素**：
- **State（状态）**：Simple（简单）、Composite（复合）、Orthogonal（正交）、Submachine（子状态机）
- **Transition（转换）**：由触发器、守卫、动作组成
- **Trigger（触发器）**：引发转换的事件
- **Guard（守卫）**：布尔条件
- **Entry/Do/Exit Action**：入口/执行/退出动作
- **Pseudostate（伪状态）**：Initial、Choice、Junction、Fork、Join、History、Terminate

**符号说明**：
```
┌─────────────────────────────────┐
│         EngineStates            │
│                                 │
│   [●] ──► ┌─────┐              │
│            │ Off │              │
│            └──┬──┘              │
│    startCmd   │ [fuelAvailable] │
│    /initIgn.  │                 │
│            ┌──▼──────┐         │
│            │Starting │         │
│            └──┬──────┘         │
│               │                 │
│            ┌──▼──────┐         │
│            │Running  │         │
│            └──┬──────┘         │
│    stopCmd    │                 │
│            ┌──▼────────────┐   │
│            │ShuttingDown   │   │
│            └──┬────────────┘   │
│               │                 │
│            ┌──▼──┐              │
│            │ Off │              │
│            └─────┘              │
└─────────────────────────────────┘
```

**使用场景**：生命周期建模、协议状态机、反应式系统行为、模式管理。

---

### 2.9 用例图 (Use Case Diagram, uc)

**用途**：从利益相关者视角捕获功能需求，展示参与者及其与系统用例的交互。

**核心元素**：
- **Actor（参与者）**：与系统交互的外部实体
- **Use Case（用例）**：系统提供的功能
- **System Boundary（系统边界）**
- **关系**：Association（关联）、Include（包含）、Extend（扩展）、Generalization（泛化）

**使用场景**：早期需求获取、范围定义、利益相关者沟通。

---

## 3. SysML v2

### 3.1 重大变化

SysML v2 是一次**彻底的重新设计**，而非增量更新。核心变化包括：

#### 3.1.1 全新元模型基础 (KerML)

SysML v2 构建在 **KerML（Kernel Modeling Language）** 之上——一个形式化基础的通用建模内核，而非 UML Profile。这赋予了它独立的语义精确性。

```
┌─────────────┐
│   SysML v2  │  ← 系统建模语言
├─────────────┤
│   KerML     │  ← 内核建模语言（形式化语义基础）
└─────────────┘
```

#### 3.1.2 文本化表示法

SysML v2 引入了一等公民地位的**文本化表示法**，与图形化表示法并列：
- 支持版本控制（git友好）
- 支持 diff/merge
- 支持脚本化和自动化
- 支持 AI/自动化集成
- 文件扩展名：`.sysml`（SysML 构件）和 `.kerml`（KerML 构件）

#### 3.1.3 定义 vs 使用（Definitions vs Usages）

清晰区分类型定义和使用/实例：
```sysml
part def Vehicle { ... }       // 定义（类型）
part myCar : Vehicle { ... }   // 使用（实例）
```

#### 3.1.4 改进的表达能力

- 增强的参数化/约束建模
- 更丰富的需求（含 subject、约束表达式）
- 改进的行为建模（actions、states、occurrences）
- 一等公民的 item 和 flow

#### 3.1.5 移除 UML 依赖

不再是 UML Profile；拥有自己的独立元模型。

#### 3.1.6 标准 API

**Systems Modeling API and Services v1.0** 提供标准 REST API：
- 工具互操作性
- 模型查询
- CRUD 操作
- 模型交换

#### 3.1.7 更好的扩展性

**Metadata 定义**取代了 Stereotypes/Profiles：
```sysml
metadata def SafetyCritical;

@SafetyCritical
part brakingSystem : BrakingSystem;
```

### 3.2 三大规范

| 规范 | 版本 | 说明 |
|------|------|------|
| KerML | v1.0 | 内核建模语言——基础元模型 |
| SysML | v2.0 | 构建在 KerML 之上的系统建模语言 |
| Systems Modeling API and Services | v1.0 | 工具互操作的 RESTful API |

### 3.3 当前状态（截至 2026 年初）

- **正式采纳**：2025年7月21日（OMG）
- **正式发布**：2025年9月3日
- **工具支持**：
  - PTC Modeler 10.2（2025年夏季，支持 SysML v2）
  - Eclipse 和 Jupyter 的试点实现（GitHub 开源）
  - Sensmetry SysMD（VS Code 集成）
  - MontiCore SysMLv2 实现
- **开源项目**：
  - https://github.com/Systems-Modeling/SysML-v2-Release
  - https://github.com/Systems-Modeling/SysML-v2-Pilot-Implementation

---

## 4. 核心概念

### 4.1 Block / Part（块 / 部件）

**SysML v1**：Block 是基本结构元素，扩展自 UML Class。包含：
- Value Properties（值属性）
- Part Properties（部件属性）
- Reference Properties（引用属性）
- Flow Properties（流属性）
- Constraint Properties（约束属性）
- Operations（操作）
- Ports（端口）

**SysML v2**：由 `part def`（定义）和 `part`（使用）取代。更精确地区分类型和实例。

```sysml
// SysML v2
part def Engine {
    attribute horsePower : Real;
    attribute displacement : Real;
    port fuelIn : ~FuelPort;
}

part myEngine : Engine {
    :>> horsePower = 250;
}
```

### 4.2 Port（端口）

定义块/部件上的交互点。

**SysML v1**：
- Standard Port：类型化的交互点
- Flow Port：方向性（in/out/inout）的流端口
- Full Port：具有自身行为的端口

**SysML v2**：
```sysml
port def FuelingPort {
    attribute flowRate : Real;
    out item fuelOut : Fuel;
    in item fuelIn : Fuel;
}
port fuelTankPort : FuelingPort;
port conjugatePort : ~FuelingPort;  // 共轭（方向取反）
```

### 4.3 Flow（流）

表示部件之间通过端口交换的项目（数据、物质、能量、信号）。

**SysML v1**：连接器上的 Item Flow，块/端口上的 Flow Properties。

**SysML v2**：
```sysml
item def Fuel {
    attribute fuelMass :> ISQ::mass;
}

flow fuelFlow from fuelTank.fuelOut to engine.fuelIn;
succession flow dataFlow from sensor.output to processor.input;
```

### 4.4 Allocation（分配）

将功能（行为）元素映射到结构元素。

**SysML v1**：`<<allocate>>` 依赖关系和分配矩阵/表。活动分区（泳道）可以展示分配。

**SysML v2**：通过使用关系和 `allocation def` / `allocation` 构件集成。

### 4.5 Requirement（需求）

**SysML v1**：`<<requirement>>` 构造型，包含 id 和 text 属性；关系：deriveReqt、satisfy、verify、refine、trace、copy。

**SysML v2**：
```sysml
requirement def MassRequirement {
    subject vehicle : Vehicle;
    attribute massActual : ISQ::MassValue;
    attribute massLimit : ISQ::MassValue;
    require constraint { massActual <= massLimit }
}

requirement <R1> vehicleMass : MassRequirement {
    attribute :>> massActual = vehicle.mass;
    attribute :>> massLimit = 1800 [kg];
}

satisfy R1 by vehicle;
```

### 4.6 Constraint（约束）与 Parametric（参数化）

**SysML v1**：Constraint Block 包含参数；在参数图中用于绑定值，支持数学分析。

**SysML v2**：
```sysml
constraint def NewtonsLaw {
    in force : ISQ::ForceValue;
    in mass : ISQ::MassValue;
    in acceleration : ISQ::AccelerationValue;
    force == mass * acceleration
}
```

### 4.7 Stereotype / Profile（v1）→ Metadata（v2）

**SysML v1**：标准 UML 扩展机制。SysML 本身就是一个 UML Profile。用户可以创建额外的 Profile 和 Stereotype。

**SysML v2**：由 `metadata def` 和 `@MetadataAnnotation` 语法取代。更灵活的一等公民语言扩展机制。

---

## 5. SysML vs UML

### 5.1 SysML 相对 UML 的增加

| 特性 | UML | SysML |
|------|-----|-------|
| 需求图 | 无 | 有——专用 req 图 |
| 参数图 | 无 | 有——基于约束的分析 |
| 块定义图 | 类图 | 取代/扩展类图用于系统 |
| 内部块图 | 复合结构图 | 增强：流端口、项目流 |
| 分配 | 无 | 有——功能到结构的映射 |
| 流端口/属性 | 有限 | 丰富的流建模（物质、能量、数据） |
| 连续/离散流 | 有限 | 一等公民支持 |
| 值类型（含单位/量纲） | 无 | 有，支持单位和量纲类型化 |

### 5.2 SysML 从 UML 移除的内容

| 移除的 UML 图 | 替代方案 |
|---------------|---------|
| 类图 | 块定义图（bdd）取代 |
| 对象图 | 内部块图（ibd）取代 |
| 组件图 | 吸收入块定义图 |
| 部署图 | 可通过分配建模 |
| 通信图 | 移除 |
| 交互概览图 | 移除 |
| 时序图 | 移除 |
| 复合结构图 | 内部块图取代 |
| Profile 图 | SysML 本身即为 Profile |

### 5.3 核心哲学差异

- **UML** 以软件为中心；**SysML** 以系统为中心（硬件 + 软件 + 流程 + 人员）
- SysML 增加了工程分析能力（参数图）
- SysML 增加了显式需求建模
- SysML 支持连续和混合（连续 + 离散）系统行为

---

## 6. 工具支持

### 6.1 商业工具

| 工具 | 厂商 | 说明 |
|------|------|------|
| **Cameo Systems Modeler** | Dassault Systemes（原 No Magic） | 领先的 SysML 工具；MagicDraw 重命名。完整 SysML v1 支持，v2 支持开发中。包含仿真工具包 |
| **IBM Engineering Rhapsody** | IBM（原 Rational） | 实时/嵌入式系统强项；SysML 和 UML 支持；C/C++/Java 代码生成 |
| **Enterprise Architect** | Sparx Systems | 性价比高；通过 MDG Technology 插件支持 SysML；用户基数大 |
| **PTC Modeler** | PTC（原 Windchill Modeler/Atego Artisan） | v10.2（2025夏季）包含 SysML v2 支持；与 Windchill PLM 集成 |
| **CATIA Magic** | Dassault Systemes | 企业级 MBSE 平台，基于 Cameo/MagicDraw 技术 |

### 6.2 开源工具

| 工具 | 说明 |
|------|------|
| **Eclipse Papyrus** | 开源 UML/SysML 建模工具；基于 Eclipse；支持 SysML v1 Profile |
| **SysML v2 Pilot Implementation** | OMG/Systems-Modeling 官方 GitHub 项目；基于 Eclipse 和 Jupyter；支持文本化表示法 |
| **Sensmetry SysMD** | SysML v2 文本化表示法工具，VS Code 集成 |
| **MontiCore SysMLv2** | 学术界实现的 SysML v2 文本化表示法解析器/分析器 |
| **PlantUML** | 文本转图表工具，有限 SysML 支持（主要是顺序图、活动图） |
| **Capella** | Eclipse 基金会/Thales 开源 MBSE 工具，使用 Arcadia 方法论（非纯 SysML 但相关） |

---

## 7. MBSE 与 SysML

### 7.1 什么是 MBSE

MBSE（Model-Based Systems Engineering，基于模型的系统工程）是从概念设计阶段开始，贯穿开发和后续生命周期阶段，将建模形式化应用于支持系统需求、设计、分析、验证和确认活动的方法。SysML 是 MBSE 的主要建模语言。

### 7.2 SysML 在 MBSE 中的角色

SysML 提供表示法和语义；MBSE 方法论提供过程。二者结合实现：
- 系统设计的**单一事实来源**（Single Source of Truth）
- 从需求到设计到 V&V 的**可追踪性**
- 通过共享模型实现**跨学科沟通**
- **自动化**分析、仿真和代码生成

### 7.3 常见 MBSE 方法论/框架

#### OOSEM（面向对象系统工程方法）

- 由 INCOSE 成员（Friedenthal, Moore, Steiner）开发
- 将面向对象技术与传统系统工程实践集成
- 活动流程：分析需要 → 定义系统需求 → 定义逻辑架构 → 综合分配架构 → 优化/评估 → V&V
- 与 ISO-15288 集成
- 使用 SysML 作为建模语言

#### Harmony-SE（IBM Rational）

- 基于经典 V 模型
- 三个主要阶段：需求分析、系统功能分析、设计综合
- 迭代和增量式
- 与 Rhapsody 工具深度集成

#### MagicGrid / MBSE Grid

- No Magic/Dassault 开发的简化 SysML 方法
- 3×3 网格：
  - 行：问题域 / 解决方案域 / 实现
  - 列：黑盒 / 白盒 / 需求
- 实用、工具支持良好

#### OpenSE Cookbook

- 基于配方的 MBSE 模式集
- 由天文/天文台社区开发
- 为科学系统应用 SysML 提供实用指导

---

## 8. 行业应用模式

### 8.1 航空航天

- **应用领域**：飞机系统、卫星系统、航空电子、飞控、导航、推进、生命保障
- **典型案例**：Airbus A350 XWB 开发广泛使用 MBSE；Boeing 积极参与 SysML v2 演进；NASA 使用 SysML 进行任务规划和航天器设计
- **常用模式**：需求可追踪性（req 图）、功能分解（act 图）、接口定义（ibd）、安全分析（par 图）、模式管理（stm）

### 8.2 汽车

- **应用领域**：车辆架构、ADAS、动力总成、车身电子、信息娱乐、自动驾驶
- **典型案例**：主要 OEM（BMW、Daimler、Toyota）使用 SysML 进行系统架构；AUTOSAR 集成
- **常用模式**：特性建模、变体管理、E/E 架构、功能安全（ISO 26262）、硬件-软件分配

### 8.3 国防

- **应用领域**：武器系统、C4ISR（指挥、控制、通信、计算机、情报、监视、侦察）、雷达、电子战
- **典型案例**：美国国防部要求 MBSE 方法；DoDAF/MODAF 架构框架集成
- **常用模式**：体系（System-of-Systems）建模、架构框架视图（作战、系统、技术）、互操作性分析

### 8.4 医疗设备

- **应用领域**：影像系统、手术机器人、患者监护、植入设备
- **常用模式**：法规可追踪性（FDA 需求）、安全关键设计、生物相容性参数分析

### 8.5 轨道交通

- **应用领域**：信号系统、机车车辆、基础设施
- **常用模式**：安全分析（SIL 等级）、功能架构、运营场景

---

## 9. SysML v2 文本化语法

### 9.1 基本语法

#### 注释
```sysml
// 行注释
/* 块注释 */
doc /* 附加到元素的文档注释 */
```

#### 包和导入
```sysml
package VehicleModel {
    public import VehicleParts::*;
    private import ISQ::MassValue;
}
alias MV for ISQ::MassValue;
```

#### 部件定义和使用
```sysml
part def Vehicle {
    attribute mass : ISQ::MassValue;
    attribute numWheels : Integer;
    part engine : Engine;
    part transmission : Transmission;
    port fuelPort : FuelingPort;
}

// 特化（继承）
part def SportsCar :> Vehicle {
    :>> numWheels = 4;  // 重定义
}

// 实例化
part myCar : Vehicle {
    :>> mass = 1500 [kg];
}
```

#### 项目定义（可流动的事物）
```sysml
item def Fuel {
    attribute fuelMass :> ISQ::mass;
}

item def ElectricalSignal {
    attribute voltage : ISQ::ElectricPotentialValue;
}
```

### 9.2 端口
```sysml
port def FuelingPort {
    attribute flowRate : Real;
    out item fuelOut : Fuel;
    in item fuelIn : Fuel;
}

port fuelTankPort : FuelingPort;
port conjugatePort : ~FuelingPort;  // 共轭（方向取反）
```

### 9.3 连接和接口
```sysml
connection def DeviceConn {
    end part hub : Hub;
    end part device : Device;
    attribute bandwidth : Real;
}

part context {
    part source { port output; }
    part target { port input; }
    interface connect source.output to target.input {
        flow from source.output to target.input;
    }
}
```

### 9.4 动作（行为建模）
```sysml
action def StartEngine {
    in ignitionSignal : Boolean;
    out status : EngineStatus;
}

action def Drive {
    action accelerate : Accelerate;
    action cruise : Cruise;
    first start then accelerate;
    then cruise;
}

part vehicle : Vehicle {
    perform action driveVehicle : Drive;
}
```

### 9.5 状态
```sysml
state def EngineStates {
    entry action initSensors;

    state off;
    state starting;
    state running;

    transition powerOn
        first off
        accept startCmd
        then starting;

    transition started
        first starting
        then running;

    transition shutDown
        first running
        accept stopCmd
        then off;
}

part engine : Engine {
    exhibit state engineBehavior : EngineStates;
}
```

### 9.6 需求
```sysml
requirement def MassRequirement {
    subject vehicle : Vehicle;
    attribute massActual : ISQ::MassValue;
    attribute massLimit : ISQ::MassValue;
    require constraint { massActual <= massLimit }
}

requirement <R1> vehicleMass : MassRequirement {
    attribute :>> massActual = vehicle.mass;
    attribute :>> massLimit = 1800 [kg];
}

satisfy R1 by vehicle;
```

### 9.7 约束
```sysml
constraint def NewtonsLaw {
    in force : ISQ::ForceValue;
    in mass : ISQ::MassValue;
    in acceleration : ISQ::AccelerationValue;
    force == mass * acceleration
}
```

### 9.8 多重性
```sysml
part wheels : Wheel[4];           // 恰好 4 个
part passengers : Person[0..5];   // 0 到 5 个
part sensors : Sensor[1..*];      // 至少 1 个
```

### 9.9 流和继承
```sysml
flow fuelFlow from fuelTank.fuelOut to engine.fuelIn;
succession first actionA then actionB;
succession flow dataFlow from sensor.output to processor.input;
```

### 9.10 元数据（替代 Stereotype）
```sysml
metadata def SafetyCritical {
    attribute silLevel : Integer;
}

@SafetyCritical { silLevel = 4; }
part brakingSystem : BrakingSystem;
```

### 9.11 文本化 vs 图形化对比

| 方面 | 文本化 | 图形化 |
|------|--------|--------|
| 版本控制 | Git 友好，可 diff/merge | 需要专用工具 |
| 可读性 | 代码风格，适合工程师 | 直观，适合沟通 |
| 自动化 | 可脚本化、CI/CD 集成 | 需要 API |
| 编辑工具 | 任何文本编辑器/IDE | 需要专用建模工具 |
| 表达能力 | 完整语义 | 完整语义 |
| 文件扩展名 | `.sysml` / `.kerml` | 工具特定格式 |

---

## 10. 与其他标准的集成

### 10.1 MARTE（实时和嵌入式系统建模与分析）

- OMG 的 UML/SysML Profile
- 扩展建模用于实时、嵌入式和软硬件协同设计
- 添加时序约束、可调度性分析、性能建模、硬件资源建模的 Stereotype
- 补充 SysML 的参数图提供领域特定分析能力
- 在航空航天和汽车领域与 SysML 配合使用进行时序/性能分析

### 10.2 SoaML（面向服务架构建模语言）

- OMG 的 UML Profile
- 建模面向服务架构：服务接口、服务合同、参与者、消息类型
- 可与 SysML 结合用于包含基于服务的软件架构的系统

### 10.3 UPDM / UAF（统一架构框架）

- **UPDM**（Unified Profile for DoDAF/MODAF）：OMG 标准，提供基于 UML/SysML 的 DoDAF 和 MODAF 架构描述方式
- 构建在 UML 2.0、SysML 和 SoaML 之上
- **UAF**（UPDM 的后继者）：扩展覆盖 DoDAF、MODAF、NAF（北约架构框架），增加人因、安全分析和系统之系统生命周期视图
- SysML 块和结构/行为图映射到 UPDM/UAF 的系统和服务视图

### 10.4 DoDAF / MODAF

- **DoDAF**（美国国防部架构框架）：美国 DoD 标准
- **MODAF**（英国国防部架构框架）：英国 MoD 等价标准
- 均定义视点：作战、系统、服务、技术等
- SysML 作为通过 UPDM/UAF 实现这些框架的底层建模语言

### 10.5 其他相关标准

| 标准 | 与 SysML 的关系 |
|------|-----------------|
| **FMI/FMU** | SysML 参数化模型可与 FMI 兼容的仿真工具进行协同仿真 |
| **OSLC** | SysML v2 API 与 OSLC 原则对齐，用于工具集成 |
| **ISO 15288** | 系统和软件工程生命周期过程；OOSEM 方法论将 SysML 与 ISO 15288 集成 |
| **AUTOSAR** | 汽车架构标准；SysML 模型可用于定义 AUTOSAR 兼容架构 |
| **AADL** | 与 SysML 互补用于实时嵌入式系统分析；存在正式映射 |

---

## 11. 参考资料

### 官方资源
- [OMG SysML v2 最终采纳公告](https://www.omg.org/news/releases/pr2025/07-21-25.htm)
- [OMG SysML v2 规范页面](https://www.omg.org/sysml/sysmlv2/)
- [SysML v2 Release GitHub](https://github.com/Systems-Modeling/SysML-v2-Release)
- [SysML v2 Pilot Implementation GitHub](https://github.com/Systems-Modeling/SysML-v2-Pilot-Implementation)
- [SysML.org - 什么是 SysML](https://sysml.org/sysml-faq/what-is-sysml.html)
- [SysML.org - 规范](https://sysml.org/sysml-specs/)

### 学习资源
- [Sensmetry SysML v2 速查表](https://sensmetry.com/sysml-cheatsheet/)
- [Siemens SysML v2 实用指南](https://blogs.sw.siemens.com/teamcenter/sysml-v2-guide/)
- [PTC SysML 2.0 概述](https://www.ptc.com/en/resources/application-lifecycle-management/white-paper/overview-of-sysml2)
- [INCOSE MBSE 方法论调查](https://www.omg.org/sysml/MBSE_Methodology_Survey_RevB.pdf)

### 工具与实现
- [MontiCore SysMLv2 实现](https://github.com/MontiCore/sysmlv2)
- [Eclipse Papyrus](https://www.eclipse.org/papyrus/)
- [Capella](https://www.eclipse.org/capella/)
