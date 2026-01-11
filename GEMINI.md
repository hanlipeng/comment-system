# gemini.md (Compact)

## 1. 核心原则 (Core Principles)

1. **契约优先**: 接口即文档。上游需求由下游在 `Interface` 注释中定义（唯一真理）。
2. **下游驱动**: 优先级 `下游(Consumer) > 上游(Provider)`。遇依赖缺失立即 Mock，**绝不等待**。
3. **递归拆解**: 需求拆分 `L1(Macro) -> L2 -> L3(Micro)`。**计划模式严禁写代码**。
4. **TDD循环**: `自然语言描述 -> 用户确认 -> 测试(Red) -> 实现(Green) -> 重构`。

## 2. 目录结构 (Structure)

```text
project/
├── docs/               # [Memory]
│   ├── planning/       # roadmap.md (Task Pool), architecture.md
│   ├── requirements/   # macro/ (Raw), micro/ (Executable Specs)
│   ├── design/         # api_specs/ (JSON Schema), flows/ (Mermaid)
│   └── templates/      # Standard Prompts
├── src/
│   ├── commands/       # [P0] CLI Entry (Downstream)
│   ├── core/           # [P1] Business Logic (Upstream)
│   └── interfaces/     # [Contract] Javadoc = Requirements
├── tests/              # unit/, mocks/
└── dev_log.md          # Progress Log

```

## 3. 计划模式 (Planning Mode)

* **输入**: 用户需求 / 上级需求。
* **输出**: 仅 Markdown 文档 (`docs/`)。
* **流程**:
1. **拆分**: 递归拆解至微观需求。
2. **依赖**: 绘制依赖图，无依赖模块标记 `[PARALLEL]`。
3. **Roadmap**: 更新 `docs/planning/roadmap.md`。


* **优先级**: `CMD/API (P0)` > `Service (P1)` > `Repo (P2)`。

## 4. 开发模式 (Dev Mode)

* **原则**: 
1. 领取 `P0` & `Pending` 任务。
2. 严格 Mock-First。
3. **物理分离**: 严禁在 `src/` 逻辑实现文件中编写测试代码。单元测试与集成测试必须存放在与 `src/` 同级的 `tests/` 目录下。

### TDD 工作流

1. **Assert (Plan)**: 用自然语言描述测试意图 -> **等待用户确认**。
2. **Red (Test)**: 编写测试。
* *依赖缺失处理*: 定义 `Interface` (详尽注释参数/返回/异常) -> 创建 `Mock` (硬编码返回)。


3. **Green (Code)**: 编写最小实现代码 -> 调用 Mock -> 通过测试。
4. **Refactor**: 优化代码 -> 更新 `dev_log.md` (记录新接口/Mock)。

## 5. 集成 (Integration)

* **触发**: 上游模块 `Status=Done`。
* **动作**: 替换 Mock 为真实实例 -> 运行下游测试。
* **失败处理**: 修正**上游代码** (必须遵守接口契约)，禁止修改测试。

## 附录: Roadmap 格式

```markdown
| ID | Module | Dep | Prio | Status | Desc |
|:---|:---|:---|:---|:---|:---|
| C1 | InitCmd | S1 | P0 | Pending | CLI Entry |
| S1 | CfgSvc | - | P1 | Pending | Config Logic |

```