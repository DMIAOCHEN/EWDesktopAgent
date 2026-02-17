# 架构设计: AI智能网关与业务系统集成

## 1. 整体架构

### 1.1 系统拓扑

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           智能桌面客户端                                     │
│   ┌─────────┐  ┌──────────┐  ┌─────────┐  ┌────────────┐  ┌───────────┐  │
│   │ 浏览器  │  │ 智能助手  │  │ 语音模块 │  │ 个性化学习 │  │ 提醒服务  │  │
│   └────┬────┘  └────┬─────┘  └────┬────┘  └─────┬──────┘  └─────┬─────┘  │
│        │            │             │              │               │         │
│        └────────────┴─────────────┴──────────────┴───────────────┘         │
│                                      │                                      │
│                                      ▼                                      │
│                           ┌─────────────────┐                              │
│                           │   统一通信层    │                              │
│                           │  (Tauri IPC)    │                              │
│                           └────────┬────────┘                              │
└────────────────────────────────────┼──────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              AI网关 (新增模块)                               │
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                         统一鉴权层                                    │   │
│   │   • Token验证  • 权限映射  • 审计日志  • 速率限制                    │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                      │                                      │
│   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                │
│   │  Token转换   │    │  能力注册    │    │  AI编排     │                │
│   │   服务       │    │    中心      │    │   引擎      │                │
│   └──────┬───────┘    └──────┬───────┘    └──────┬───────┘                │
│          │                   │                   │                          │
│          └───────────────────┼───────────────────┘                          │
│                              │                                              │
└──────────────────────────────┼──────────────────────────────────────────────┘
                               │
          ┌────────────────────┼────────────────────┐
          │                    │                    │
          ▼                    ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│  现有业务系统后端 │  │  现有业务系统后端 │  │   FastGPT 服务   │
│     (RIS)       │  │     (PIS)       │  │                 │
│                 │  │                 │  │  • 对话能力      │
│  • 能力注册     │  │  • 能力注册     │  │  • 分析能力      │
│  • 业务API      │  │  • 业务API      │  │  • 工具调用      │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

---

## 2. 鉴权与Token设计

### 2.1 用户体系

```
客户 (Customer) [唯一]
  │
  ├── 机构A (Institution)
  │     ├── 系统1 (System) - RIS
  │     ├── 系统2 (System) - PIS
  │     └── 系统3 (System) - EIS
  │
  └── 机构B (Institution)
        ├── 系统4 (System) - LIS
        └── ...
```

### 2.2 Token类型

| Token类型 | 用途 | 有效期 | 存储位置 |
|-----------|------|--------|----------|
| SystemToken | 业务系统登录凭证 | 可配置 | 客户端安全存储 |
| FastGPTmapped | AI网关映射凭证 | 短效 | 网关内存/缓存 |
| APIToken | 业务API调用凭证 | 长期 | 服务端数据库 |

### 2.3 Token转换流程

```
1. 用户登录
   │
   ├── 客户端 → 业务系统API → 获取SystemToken
   │
   └── 客户端存储 SystemToken (加密)

2. AI请求
   │
   ├── 客户端携带SystemToken请求AI服务
   │
   ├── AI网关验证Token
   │   ├── 检查Token有效性
   │   ├── 解析用户身份 (user_id, institution_id)
   │   ├── 获取用户权限
   │   └── 映射到FastGPT可用能力
   │
   ├── 转发到FastGPT
   │
   └── 返回AI响应
```

### 2.4 权限模型

```json
{
  "user": {
    "id": "user_001",
    "name": "张医生",
    "institution_id": "inst_001",
    "role": "doctor"
  },
  "permissions": {
    "ris": ["read", "write", "approve"],
    "pis": ["read"],
    "eis": ["read", "write"]
  },
  "fastgpt_access": {
    "allowed_systems": ["ris", "pis"],
    "max_tokens": 4000,
    "rate_limit": "100 req/hour"
  }
}
```

---

## 3. 后端API暴露方案

### 3.1 能力注册机制

每个业务系统需要在AI网关注册自己的能力：

**注册时机**: 系统启动时 / 定时心跳 / 手动注册

**注册接口**:
```
POST /api/gateway/capabilities/register
Body:
{
  "system": "ris",
  "version": "1.0.0",
  "endpoint": "http://ris-server.internal/api",
  "capabilities": [...]
}
```

### 3.2 能力描述标准

```typescript
interface Capability {
  // 基础信息
  id: string;                    // 唯一标识
  system: string;                 // 系统标识
  name: string;                   // 能力名称 (中文)
  name_en: string;                // 能力名称 (英文)
  description: string;            // 详细描述

  // 输入输出
  input: {
    parameters: Parameter[];       // 输入参数
    example: object;              // 输入示例
  };
  output: {
    schema: object;               // 输出Schema
    example: object;              // 输出示例
  };

  // 权限与限制
  auth: {
    required: boolean;
    permission: string;            // 所需权限标识
  };
  rateLimit?: {
    requests: number;
    window: string;               // e.g., "1m", "1h"
  };

  // 元数据
  tags: string[];                  // 分类标签
  keywords: string[];              // 关键词(供AI匹配)
  examples: Example[];             // 使用示例
}

interface Parameter {
  name: string;
  type: string;                   // string, number, boolean, object, array
  required: boolean;
  description: string;
  default?: any;
  validation?: string;            // JSON Schema
  enum?: any[];                   // 枚举值
}
```

### 3.3 能力分类

| 类别 | 描述 | 示例 |
|------|------|------|
| **Query** | 数据查询 | 查询患者列表、查询检查报告 |
| **Action** | 业务操作 | 提交报告、审核报告 |
| **Navigation** | 页面导航 | 跳转到指定页面 |
| **DataExtract** | 数据提取 | 获取当前页面患者信息 |
| **Report** | 报表生成 | 生成统计报表 |

### 3.4 医疗业务能力示例

```json
{
  "system": "ris",
  "capabilities": [
    {
      "id": "ris.query.patient.exams",
      "name": "查询患者检查列表",
      "description": "根据患者ID查询该患者的所有检查记录，包括检查类型、检查日期、报告状态、影像所见等详细信息",
      "input": {
        "parameters": [
          {"name": "patientId", "type": "string", "required": true, "description": "患者唯一标识ID"},
          {"name": "examType", "type": "string", "required": false, "description": "检查类型: CT/MRI/XRay"},
          {"name": "dateFrom", "type": "date", "required": false, "description": "开始日期"},
          {"name": "dateTo", "type": "date", "required": false, "description": "结束日期"}
        ],
        "example": {"patientId": "P001", "examType": "CT"}
      },
      "output": {
        "schema": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "examId": {"type": "string"},
              "patientName": {"type": "string"},
              "examType": {"type": "string"},
              "examDate": {"type": "datetime"},
              "reportStatus": {"type": "enum", "values": ["pending", "draft", "approved"]}
            }
          }
        }
      },
      "auth": {"required": true, "permission": "ris:read"},
      "keywords": ["检查", "患者", "报告", "CT", "MRI", "影像"],
      "tags": ["query", "patient"]
    }
  ]
}
```

---

## 4. 前端交互协议

### 4.1 桥接层架构

```
┌─────────────────────────────────────────────────────────────────┐
│                     业务系统前端页面                              │
│                                                                 │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              IntelligentAgentBridge (桥接组件)           │   │
│   │                                                          │   │
│   │   • registerCapabilities()    注册页面能力              │   │
│   │   • executeAction()          执行AI返回的动作           │   │
│   │   • getPageContext()         获取页面上下文             │   │
│   │   • onPageChange()           页面变化监听               │   │
│   │   └── onDataChange()         数据变化监听               │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│                              ▼                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   业务系统原生代码                        │   │
│   │   • API调用        • DOM操作        • 路由控制          │   │
│   └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │   客户端SDK     │
                    │ (Tauri Bridge)  │
                    └────────┬────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │     AI网关      │
                    └─────────────────┘
```

### 4.2 能力注册协议

```javascript
// 业务系统前端初始化时调用
const bridge = new IntelligentAgentBridge();

// 注册本页面可用的能力
bridge.registerCapabilities({
  page: {
    id: 'ris_report_edit',
    name: '报告编辑页',
    path: '/ris/report/edit',
  },

  // 可执行的动作
  actions: [
    {
      id: 'save_report',
      name: '保存报告',
      description: '保存当前编辑的报告内容',
      params: [],
      handler: 'saveReport'  // 对应前端方法名
    },
    {
      id: 'query_patient',
      name: '查询患者',
      description: '根据患者ID查询患者基本信息',
      params: [
        {name: 'patientId', type: 'string', required: true}
      ],
      handler: 'queryPatient'
    }
  ],

  // 可获取的上下文
  contextProviders: [
    {
      name: 'currentPatient',
      description: '当前页面关联的患者信息',
      handler: 'getCurrentPatient'
    },
    {
      name: 'formData',
      description: '当前表单数据',
      handler: 'getFormData'
    }
  ]
});
```

### 4.3 动作执行协议

AI网关返回的动作格式：

```json
{
  "actions": [
    {
      "type": "execute",
      "system": "ris",
      "action": "query_patient",
      "params": {
        "patientId": "P001"
      }
    },
    {
      "type": "navigate",
      "target": "ris_report_list"
    },
    {
      "type": "extract",
      "from": "page",
      "field": "currentPatient"
    }
  ]
}
```

客户端执行流程：

```
1. 接收AI动作序列
2. 解析每个动作
3. 按顺序执行 (可并行/可串行)
4. 收集执行结果
5. 返回给AI进行下一步处理
```

### 4.4 页面上下文标准

```json
{
  "page": {
    "id": "ris_report_edit",
    "path": "/ris/report/edit/:id",
    "title": "报告编辑"
  },
  "user": {
    "id": "user_001",
    "role": "doctor",
    "permissions": ["ris:write"]
  },
  "context": {
    "currentPatient": {
      "id": "patient_001",
      "name": "王建国",
      "gender": "男",
      "age": 45
    },
    "currentVisit": {
      "id": "visit_001",
      "type": "门诊"
    },
    "formData": {
      "examId": "exam_001",
      "finding": "...",
      "diagnosis": "..."
    }
  },
  "availableActions": ["save", "submit", "print", "preview"]
}
```

---

## 5. 操作风险分级管控

### 5.1 风险等级定义

| 等级 | 定义 | 示例操作 | 确认要求 |
|------|------|----------|----------|
| **低风险** | 只读操作，不涉及敏感数据 | 查询、浏览、搜索 | 直接执行 |
| **中风险** | 修改数据，但不外发 | 保存、修改、创建 | 预览确认 |
| **高风险** | 数据外发、删除、系统操作 | 发送邮件、删除文件、执行命令 | 明确确认+日志 |

### 5.2 风险评估模型

```typescript
interface RiskLevel {
  level: 'low' | 'medium' | 'high';
  score: number;        // 0-100
  factors: string[];    // 评估因素
}

function assessRisk(operation: Operation, context: Context): RiskLevel {
  // 1. 操作类型权重
  const typeScore = getTypeWeight(operation.type);

  // 2. 数据敏感度权重
  const sensitivityScore = getDataSensitivity(operation.target);

  // 3. 目标范围权重
  const scopeScore = getScopeWeight(operation.scope);

  // 4. 综合评分
  const totalScore = typeScore + sensitivityScore + scopeScore;

  return {
    level: totalScore >= 70 ? 'high' : totalScore >= 40 ? 'medium' : 'low',
    score: totalScore,
    factors: [...]
  };
}
```

### 5.3 确认流程

```
AI执行请求 → 风险评估 → 等级判定
                        │
          ┌─────────────┼─────────────┐
          ▼             ▼             ▼
        低风险        中风险        高风险
          │             │             │
          ▼             ▼             ▼
       直接执行     预览确认      弹窗确认
          │             │             │
          └─────────────┼─────────────┘
                        ▼
                     执行操作
                        │
                        ▼
                     记录日志
```

### 5.4 白名单机制

用户可以设置信任的操作模式，后续自动执行：

```json
{
  "whitelist": [
    {
      "pattern": "search_*",
      "risk_level": "low",
      "expires_at": "2026-12-31",
      "auto_execute": true
    }
  ]
}
```

---

## 6. 本地文件系统操作

### 6.1 支持的操作能力

| 能力ID | 名称 | 描述 | 风险 |
|--------|------|------|------|
| fs.read_file | 读取文件 | 读取指定文件内容 | 低 |
| fs.list_dir | 列出目录 | 列出目录下的文件和文件夹 | 低 |
| fs.create_dir | 创建目录 | 创建新文件夹 | 低 |
| fs.move_file | 移动文件 | 移动文件到指定目录 | 中 |
| fs.rename_file | 重命名 | 重命名文件或文件夹 | 中 |
| fs.copy_file | 复制文件 | 复制文件到指定位置 | 中 |
| fs.delete_file | 删除文件 | 删除指定文件或文件夹 | 高 |
| fs.organize | 文件整理 | 按规则自动整理文件 | 中 |

### 6.2 目录访问控制

```typescript
interface FileAccessControl {
  // 默认授权目录
  allowedPaths: [
    '{user}/Documents',
    '{user}/Downloads',
    '{user}/Desktop'
  ];

  // 需要额外授权的目录
  restrictedPaths: [
    '/system',
    '/program files',
    '/windows'
  ];

  // 当前会话临时授权
  tempPermissions: [];
}
```

### 6.3 文件整理规则

```typescript
interface OrganizeRule {
  sourceDir: string;           // 源目录
  targetDir: string;           // 目标目录
  ruleType: 'by_date' | 'by_type' | 'by_name' | 'mixed';

  // 按日期整理
  dateOptions?: {
    format: 'YYYY' | 'YYYY-MM' | 'YYYY-MM-DD';
    field: 'created' | 'modified' | 'accessed';
  };

  // 按类型整理
  typeOptions?: {
    groups: [
      {name: 'Documents', extensions: ['.pdf', '.doc', '.docx']},
      {name: 'Images', extensions: ['.jpg', '.png', '.gif']}
    ];
  };
}
```

---

## 7. 实施指南

### 5.1 实施步骤

| 阶段 | 任务 | 负责方 |
|------|------|--------|
| **Phase 1** | AI网关开发 | 后端团队 |
| **Phase 2** | 能力注册规范制定 | 架构组 |
| **Phase 3** | 业务系统后端改造 | 各系统负责人 |
| **Phase 4** | 前端SDK开发 | 前端团队 |
| **Phase 5** | 业务系统前端集成 | 各系统前端负责人 |
| **Phase 6** | 联调与测试 | QA |

### 5.2 交付物清单

| 交付物 | 说明 |
|--------|------|
| AI网关服务 | Token转换、请求路由、鉴权 |
| 能力注册平台 | 可视化管理能力清单 |
| 后端SDK | 各语言能力注册客户端 |
| 前端SDK | 前端桥接组件 |
| 集成文档 | 业务系统接入指南 |

---

## 8. AI网关技术架构 (.NET Core)

### 8.1 技术栈

| 组件 | 技术 | 版本 |
|------|------|------|
| 框架 | .NET Core | 8.0+ |
| Web框架 | ASP.NET Core | 8.0 |
| API文档 | Swagger/OpenAPI | |
| 认证 | JWT + OAuth 2.0 | |
| 缓存 | Redis | 7.0+ |
| 数据库 | PostgreSQL | 15+ |
| 日志 | Serilog | |
| 部署 | Docker + K8s | |

### 8.2 项目结构

```
src/
├── EW.AiGateway.Api/               # API层
├── EW.AiGateway.Core/              # 核心业务
├── EW.AiGateway.Infrastructure/    # 基础设施
└── EW.AiGateway.Sdk/              # 客户端SDK
```

### 8.3 核心服务

| 服务 | 职责 |
|------|------|
| AuthService | Token验证与转换 |
| CapabilityService | 能力注册与管理 |
| AiOrchestrationService | AI请求编排 |
| AuditService | 审计日志 |
| RiskService | 风险评估 |

---

## 9. 语音交互技术架构

### 9.1 技术选型

| 模块 | 技术 | 说明 |
|------|------|------|
| 唤醒词检测 | Snowboy + Silero VAD | 离线唤醒 + 语音活动检测 |
| ASR | 第三方医疗语音API | 医疗术语优化 |
| TTS (本地) | Kokoro | 高质量离线语音合成 |
| TTS (云端) | Azure TTS | 兜底方案 |

### 9.2 语音处理流程

```
麦克风 → 唤醒检测(Snowboy+SileroVAD) → ASR(第三方API) → AI处理
                                                    ↓
                                                TTS合成
                                               (本地优先)
                                                    ↓
                                                扬声器播放
```

---

## 6. 附录

### 6.1 API命名规范

- 系统标识: 2-4位字母 (ris, pis, eis, lis)
- 能力ID: `{system}.{category}.{action}`
- 权限标识: `{system}:{operation}`

### 6.2 错误码设计

| 错误码 | 含义 |
|--------|------|
| 401 | Token无效或过期 |
| 403 | 权限不足 |
| 404 | 能力不存在 |
| 429 | 请求过于频繁 |
| 500 | 业务系统错误 |

### 6.3 审计日志字段

- user_id, institution_id, system
- request_time, request_type
- ai_prompt, ai_response
- executed_actions, action_results
- duration_ms, status
