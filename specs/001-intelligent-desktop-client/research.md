# Research: 智能桌面客户端

## 系统架构设计 (新增)

### 1. 用户体系与鉴权架构

**Hierarchy**: 客户 -> 机构 -> 系统
```
客户 (Customer)
└── 机构 (Institution)
    └── 系统 (System)
       ├── RIS (放射信息系统)
       ├── PIS (病理信息系统)
       ├── EIS (检验信息系统)
       └── ...
```

**Decision**: 三层鉴权网关架构

```
┌─────────────────────────────────────────────────────────────────┐
│                      智能桌面客户端                              │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                    AI网关 (新增)                                 │
│  ┌─────────────────┐  ┌─────────────────┐                      │
│  │ Token转换服务   │  │ 能力注册中心    │                      │
│  │ System→FastGPT │  │ API能力清单     │                      │
│  └────────┬────────┘  └────────┬────────┘                      │
│           │                    │                                │
│           ▼                    ▼                                │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    统一鉴权层                              │  │
│  │   - Token验证  - 权限校验  - 审计日志                      │  │
│  └──────────────────────────────────────────────────────────┘  │
└──────────────────────────┬──────────────────────────────────────┘
                           │
           ┌───────────────┴───────────────┐
           ▼                               ▼
┌──────────────────────┐      ┌──────────────────────────┐
│   现有业务系统后端    │      │      FastGPT 服务        │
│   (RIS/PIS/EIS)      │      │                          │
│                      │      │   AI对话/分析能力        │
└──────────────────────┘      └──────────────────────────┘
```

**Rationale**:
- 网关解耦：原有系统和FastGPT独立演进
- 统一鉴权：一次登录，多处可用
- 审计追溯：所有AI调用都有记录

### 2. Token转换机制

**Decision**: 双Token机制 + Token映射表

```
用户登录 → 获取SystemToken → 存储在客户端
            │
            ▼
客户端请求AI → 携带SystemToken → AI网关验证
                │
                ├─ 检查Token有效性
                ├─ 解析用户身份和机构
                ├─ 映射到FastGPT权限
                └─ 转发请求到FastGPT
```

**Token映射表设计**:
```json
{
  "system_token": "xxx",
  "user_id": "user_123",
  "institution_id": "inst_456",
  "permissions": ["ris:read", "pis:write"],
  "fastgpt_token": "映射的FastGPT Token",
  "expires_at": "2026-02-18T00:00:00Z"
}
```

### 3. 后端API暴露方案

**Decision**: 能力注册 + 标准化描述

每个业务系统需要注册自己的能力，供AI理解和使用：

```typescript
// 能力注册标准格式
interface Capability {
  system: string;        // 系统标识 (ris/pis/eis)
  version: string;       // 接口版本
  name: string;          // 能力名称
  description: string;   // 能力描述(供AI理解)
  parameters: Parameter[]; // 输入参数
  output: OutputSchema;  // 输出格式
  auth_required: string; // 所需权限
  examples: Example[];   // 使用示例
}

interface Parameter {
  name: string;
  type: string;
  required: boolean;
  description: string;
  validation?: string;   // 验证规则
}
```

**API注册示例**:
```json
{
  "system": "ris",
  "capabilities": [
    {
      "name": "查询患者检查列表",
      "description": "根据患者ID查询该患者的所有检查记录，包括检查类型、检查日期、报告状态等",
      "parameters": [
        {"name": "patientId", "type": "string", "required": true, "description": "患者ID"}
      ],
      "output": {
        "type": "array",
        "items": {
          "checkId": "string",
          "checkType": "string",
          "checkDate": "datetime",
          "reportStatus": "enum"
        }
      }
    }
  ]
}
```

### 4. 前端交互协议设计

**Decision**: Web Component + 统一接口协议

在每个业务系统前端嵌入智能助手桥接层：

```javascript
// 业务系统前端集成标准
class IntelligentAgentBridge {
  // 注册系统能力
  registerCapabilities(capabilities) {
    // 向AI网关注册本系统可用的功能
  }

  // 执行操作
  async executeAction(action) {
    // 解析AI返回的操作指令
    // 调用对应的前端方法或API
  }

  // 获取页面上下文
  getPageContext() {
    // 返回当前页面的关键信息
    // 供AI理解当前状态
  }

  // 监听页面变化
  onPageChange(callback) {
    // 页面跳转时通知AI助手
  }
}
```

**前端暴露能力标准**:

| 能力类型 | 示例 | 描述方式 |
|----------|------|----------|
| **页面跳转** | 跳转到报告编辑页 | goto(pageId, params) |
| **数据查询** | 查询患者信息 | query(table, conditions) |
| **数据提交** | 提交报告 | submit(formId, data) |
| **UI操作** | 打开弹窗 | openModal(modalId) |
| **数据提取** | 获取当前页面患者ID | getCurrentPatient() |

**页面上下文标准**:
```json
{
  "pageId": "ris_report_edit",
  "pageName": "报告编辑页",
  "currentPatient": {
    "id": "patient_123",
    "name": "张三",
    "visitId": "visit_456"
  },
  "availableActions": ["save", "submit", "print"],
  "formData": {}
}
```

### 5. 完整调用流程

```
用户: "帮我查一下患者王建的CT检查"

┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ 客户端      │     │ AI网关       │     │ 业务系统    │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │
       │ 1. 发送请求       │                   │
       │──────────────────>│                   │
       │                   │ 2. 验证Token      │
       │                   │──┐                │
       │                   │<─┘                │
       │                   │                   │
       │ 3. 转发AI请求     │                   │
       │───────────────────────────────────────>│
       │                   │                   │
       │                   │<────────────────────│
       │                   │ 4. 返回AI结果+动作 │
       │<──────────────────────────────────────│
       │                   │                   │
       │ 5. 解析动作: query                    │
       │───────────────────────────────────────>│
       │                   │                   │ 6. 执行查询
       │                   │<──────────────────────────────────│
       │                   │                   │
       │<───────────────────────────────────────│
       │                   │                   │
       ▼                   ▼                   ▼
```

---

## 7. 操作风险分级设计

### 7.1 风险等级定义

| 等级 | 定义 | 示例 | 确认要求 |
|------|------|------|----------|
| **低风险** | 只读操作、不涉及敏感数据 | 查询患者列表、查看报告内容 | 直接执行 |
| **中风险** | 修改数据但不外发 | 保存报告、修改患者信息 | 展示预览，用户确认 |
| **高风险** | 数据外发、删除、系统操作 | 发送邮件、删除文件、执行命令 | 明确确认，记录日志 |

### 7.2 风险评估引擎

```typescript
interface RiskAssessment {
  operation: Operation;
  context: OperationContext;
  target: RiskTarget;

  // 评估结果
  level: 'low' | 'medium' | 'high';
  factors: RiskFactor[];
  requiresConfirmation: boolean;
}

interface RiskFactor {
  type: 'data_sensitivity' | 'operation_type' | 'target_scope' | 'user_history';
  weight: number;
  score: number;
}
```

### 7.3 风险规则配置

```json
{
  "rules": [
    {
      "pattern": "*.send_email",
      "level": "high",
      "requires_confirmation": true,
      "description": "发送邮件涉及数据外发"
    },
    {
      "pattern": "*.delete_file",
      "level": "high",
      "requires_confirmation": true,
      "description": "删除文件需确认"
    },
    {
      "pattern": "*.write_data",
      "level": "medium",
      "requires_confirmation": true,
      "description": "数据写入需预览确认"
    },
    {
      "pattern": "*.read_data",
      "level": "low",
      "requires_confirmation": false,
      "description": "只读操作直接执行"
    }
  ]
}
```

### 7.4 用户确认流程

```
用户指令 → AI解析 → 风险评估 → [高风险] → 确认弹窗 → 执行
                              → [中风险] → 预览确认 → 执行
                              → [低风险] → 直接执行
```

---

## 8. 本地文件系统操作设计

### 8.1 支持的操作

| 操作类型 | 描述 | 风险等级 |
|----------|------|----------|
| **read_file** | 读取文件内容 | 低 |
| **list_dir** | 列出目录内容 | 低 |
| **create_dir** | 创建文件夹 | 低 |
| **move_file** | 移动文件 | 中 |
| **rename_file** | 重命名文件 | 中 |
| **copy_file** | 复制文件 | 中 |
| **delete_file** | 删除文件/文件夹 | 高 |
| **organize_files** | 按规则整理文件 | 中 |

### 8.2 文件操作安全

**Sandbox限制**:
- 只允许操作用户授权的目录
- 默认授权: 用户文档目录、下载目录、桌面
- 需要明确授权: 系统目录、敏感目录

**操作预览**:
- 执行前展示操作计划
- 显示源路径、目标路径、操作类型
- 预估影响范围

**操作日志**:
- 记录所有文件操作
- 包含操作时间、操作类型、文件路径、操作结果

### 8.3 文件整理功能

```typescript
interface OrganizeRule {
  type: 'by_date' | 'by_type' | 'by_name' | 'custom';
  options: {
    dateFormat?: string;      // e.g., "YYYY-MM"
    extensions?: string[];    // e.g., [".pdf", ".docx"]
    pattern?: string;         // regex for name matching
  };
}

// 示例：按年月整理
{
  "type": "by_date",
  "options": {
    "dateFormat": "YYYY-MM",
    "sourceField": "modified"
  }
}
```

---

## Technical Decisions

### 1. 客户端框架选型

**Decision**: Tauri (Rust + WebView2)

**Rationale**:
- 内存效率高：相比Electron，Tauri内存占用更低，适合2GB低配环境
- WebView2：Windows原生WebView，基于Chromium，兼容性好
- 安全：Rust内存安全，无GC暂停
- 包体小：发布体积远小于Electron

**Alternatives Considered**:
- Electron: 内存占用较高(>300MB空载)，不符合500MB约束
- C#/WPF + WebView2: 开发效率高，但跨平台能力弱
- C++/Qt: 开发成本高，不适合快速迭代

### 2. 浏览器引擎选型

**Decision**: WebView2 (Microsoft Edge Chromium)

**Rationale**:
- Windows原生集成，性能好
- 与现有业务系统(RIS/PIS/EIS)兼容性最好
- 支持多标签页架构
- 支持文件下载

**Alternatives Considered**:
- CEF (Chromium Embedded): 包体大，内存占用高
- WebKitGTK: Windows支持不佳

### 3. AI服务集成

**Decision**: HTTP API调用FastGPT服务

**Rationale**:
- FastGPT已部署，通过HTTP API调用最简单
- 客户端只做轻量推理，主要在服务端
- 支持流式输出

**Alternatives Considered**:
- 本地部署小模型: 受限于客户端算力，不可行
- WebSocket长连接: 需要服务端支持，当前FastGPT API足够

### 4. 语音交互方案

**Decision**: Snowboy/Silero VAD + 第三方ASR + 本地Kokoro TTS + 云端TTS兜底

**Rationale**:
- **唤醒词**: Snowboy (轻量级) + Silero VAD (高精度语音检测)
  - Snowboy: 离线唤醒，低资源占用
  - Silero VAD: 优秀的语音活动检测，过滤噪音
- **ASR (语音识别)**: 第三方专业医疗语音服务接口
  - 医疗术语识别准确率高
  - 支持实时流式识别
- **TTS (语音合成)**:
  - 本地优先: Kokoro (高质量本地TTS)
  - 兜底方案: 云端TTS (网络不稳定时切换)

**语音交互架构**:
```
┌─────────────────────────────────────────────────────────────┐
│                        客户端                               │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │  唤醒模块   │───▶│  ASR模块    │───▶│  TTS模块    │  │
│  │ Snowboy    │    │ 第三方API   │    │ Kokoro/云端 │  │
│  │ Silero VAD │    │             │    │             │  │
│  └─────────────┘    └─────────────┘    └─────────────┘  │
│         │                  │                  │            │
│         └──────────────────┼──────────────────┘            │
│                            ▼                               │
│                   ┌─────────────┐                         │
│                   │  语音处理   │                         │
│                   │   引擎      │                         │
│                   └─────────────┘                         │
└───────────────────────────┬─────────────────────────────────┘
                          ▼
                 ┌─────────────────┐
                 │    AI网关       │
                 │  (FastGPT)      │
                 └─────────────────┘
```

**模块设计**:

| 模块 | 技术 | 说明 |
|------|------|------|
| 唤醒检测 | Snowboy + Silero VAD | 离线唤醒 + 语音活动检测 |
| ASR | 第三方医疗语音API | 流式识别，医疗术语优化 |
| TTS | Kokoro (本地) | 高质量离线语音合成 |
| TTS兜底 | 云端TTS | 网络问题时自动切换 |

**Alternatives Considered**:
- 纯本地ASR: 医疗术语识别困难
- 纯云端: 网络依赖大，断网完全不可用
- Windows SAPI: 通用性强但医疗专业度不足

### 5. 数据存储

**Decision**: SQLite (本地) + 服务端同步

**Rationale**:
- SQLite轻量，无需独立数据库服务
- 用户行为数据本地存储，保护隐私
- 个性化模型参数加密同步到服务端

**Alternatives Considered**:
- 文件存储: 查询性能差
- LevelDB: 移动端好，桌面端不如SQLite成熟

### 6. 内存优化策略

**Decision**: 多进程架构 + 惰性加载

**Rationale**:
- 主进程：UI框架 + 业务逻辑
- 浏览器进程：每个标签页独立进程，隔离内存
- 按需加载：非核心功能模块延迟加载

## Module Architecture (per Constitution)

```
src/
├── main/                    # 主进程入口
├── core/                   # 核心模块
│   ├── browser/           # 浏览器引擎封装
│   ├── ai/               # AI服务客户端
│   ├── voice/            # 语音交互
│   └── storage/          # 数据存储
├── features/              # 功能模块
│   ├── assistant/        # 智能助手
│   ├── reminders/        # 智能提醒
│   └── personalization/  # 个性化学习
└── shared/               # 共享组件
    ├── config/           # 配置管理
    ├── logging/          # 日志 (per Constitution III)
    └── security/         # 安全模块 (per Constitution IV)
```

## Non-Functional Considerations

### Memory Constraints (from Spec NFRs)
- 客户端总内存: <500MB
- 单标签页: <200MB
- 启动时间: <5秒

### Security (per Constitution IV)
- 用户数据加密存储
- API密钥安全管理
- 医疗数据审计日志
- 最小权限原则

### Testing (per Constitution II)
- 核心模块 >80% 测试覆盖率
- TDD流程：测试→实现→重构
