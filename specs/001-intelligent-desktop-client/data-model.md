# Data Model: 智能桌面客户端

## 扩展: 用户体系与网关

### 10. Customer (客户)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 客户唯一标识 |
| name | String | 客户名称 |
| contact | String | 联系人 |
| config | JSON | 客户级配置 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

---

### 11. Institution (机构)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 机构唯一标识 |
| customer_id | UUID | 所属客户ID |
| name | String | 机构名称 |
| code | String | 机构代码 |
| config | JSON | 机构级配置 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

**Relationships**:
- Belongs to `Customer`
- Has many `System`

---

### 12. System (业务系统)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 系统唯一标识 |
| institution_id | UUID | 所属机构ID |
| name | String | 系统名称(RIS/PIS/EIS) |
| code | String | 系统代码 |
| api_endpoint | String | API端点 |
| capabilities | JSON | 已注册的能力清单 |
| is_enabled | Boolean | 是否启用 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

**Relationships**:
- Belongs to `Institution`

---

### 13. TokenMapping (Token映射)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 映射唯一标识 |
| user_id | UUID | 用户ID |
| system_token | String | 系统Token(加密存储) |
| fastgpt_token | String | FastGPT Token |
| expires_at | DateTime | 过期时间 |
| created_at | DateTime | 创建时间 |

---

### 14. SystemCapability (系统能力)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 能力唯一标识 |
| system_id | UUID | 所属系统ID |
| capability_id | String | 能力标识(如 ris.query.patient) |
| name | String | 能力名称 |
| description | String | 能力描述 |
| input_schema | JSON | 输入参数Schema |
| output_schema | JSON | 输出Schema |
| auth_required | String | 所需权限 |
| keywords | JSON | 关键词(供AI匹配) |
| examples | JSON | 使用示例 |
| is_enabled | Boolean | 是否启用 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

**Relationships**:
- Belongs to `System`

---

### 15. AuditLog (审计日志 - 扩展)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 日志唯一标识 |
| user_id | UUID | 用户ID |
| institution_id | UUID | 机构ID |
| system_id | UUID | 业务系统ID |
| action_type | Enum | 操作类型(ai_request/business_api) |
| request_data | JSON | 请求内容 |
| response_data | JSON | 响应内容 |
| duration_ms | Integer | 耗时 |
| status | Enum | 状态 |
| created_at | DateTime | 创建时间 |

---

### 16. RiskRule (风险规则)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 规则唯一标识 |
| name | String | 规则名称 |
| pattern | String | 操作匹配模式 |
| level | Enum | 风险等级(low/medium/high) |
| requires_confirmation | Boolean | 是否需要确认 |
| description | String | 规则描述 |
| is_enabled | Boolean | 是否启用 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

---

### 17. OperationRecord (操作记录)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 记录唯一标识 |
| user_id | UUID | 用户ID |
| session_id | UUID | 会话ID |
| operation_type | String | 操作类型 |
| operation_detail | JSON | 操作详情 |
| risk_level | Enum | 风险等级 |
| confirmation_status | Enum | 确认状态(pending/approved/rejected) |
| confirmation_time | DateTime | 确认时间 |
| result | JSON | 执行结果 |
| duration_ms | Integer | 耗时 |
| created_at | DateTime | 创建时间 |

---

### 18. Whitelist (白名单)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 白名单唯一标识 |
| user_id | UUID | 用户ID |
| pattern | String | 操作匹配模式 |
| risk_level | Enum | 对应风险等级 |
| expires_at | DateTime | 过期时间 |
| is_active | Boolean | 是否生效 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

---

### 19. FileOperation (文件操作记录)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 操作唯一标识 |
| user_id | UUID | 用户ID |
| operation_type | Enum | 操作类型(read/write/delete/move/...) |
| source_path | String | 源路径 |
| target_path | String | 目标路径(如有) |
| risk_level | Enum | 风险等级 |
| confirmed | Boolean | 是否经过确认 |
| result | Enum | 执行结果(success/failed/cancelled) |
| error_message | String | 错误信息(如有) |
| created_at | DateTime | 创建时间 |

---

## Entities

### 1. User (用户)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 用户唯一标识 |
| name | String | 用户姓名 |
| department | String | 科室 |
| role | String | 角色(医生/护士/主任等) |
| permissions | JSON | 权限配置 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

**Relationships**:
- Has many `BusinessSystem` (accessible)
- Has many `AssistantSession`
- Has many `ReminderRule`
- Has one `UserBehaviorModel`

---

### 2. BusinessSystem (业务系统)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 业务系统唯一标识 |
| name | String | 系统名称(RIS/PIS/EIS) |
| url | String | 系统URL(后台配置) |
| icon | String | 图标路径 |
| auth_type | Enum | 认证方式(sso/password/api_key) |
| auth_config | JSON | 认证配置(加密存储) |
| page_mappings | JSON | 页面元素映射(用于AI操作) |
| is_enabled | Boolean | 是否启用 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

**Relationships**:
- Has many `User` (accessible users)

---

### 3. AssistantSession (智能助手会话)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 会话唯一标识 |
| user_id | UUID | 用户ID |
| business_system_id | UUID | 当前所在业务系统 |
| context | JSON | 会话上下文 |
| messages | JSON | 消息历史 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

**Relationships**:
- Belongs to `User`
- Belongs to `BusinessSystem`

---

### 4. ReminderRule (提醒规则)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 规则唯一标识 |
| user_id | UUID | 用户ID |
| name | String | 规则名称 |
| trigger_type | Enum | 触发类型(time/event/api) |
| trigger_config | JSON | 触发配置 |
| content | String | 提醒内容模板 |
| target_url | String | 目标页面URL |
| is_enabled | Boolean | 是否启用 |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |

**Relationships**:
- Belongs to `User`
- Has many `ReminderRecord`

---

### 5. ReminderRecord (提醒记录)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 记录唯一标识 |
| rule_id | UUID | 规则ID |
| content | String | 实际提醒内容 |
| is_read | Boolean | 是否已读 |
| is_clicked | Boolean | 是否被点击 |
| triggered_at | DateTime | 触发时间 |
| read_at | DateTime | 阅读时间 |

**Relationships**:
- Belongs to `ReminderRule`

---

### 6. VoiceCommand (语音指令)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 指令唯一标识 |
| user_id | UUID | 用户ID |
| audio_data | Blob | 原始音频 |
| text | String | 识别文本 |
| intent | String | 解析意图 |
| entities | JSON | 提取实体 |
| result | String | 执行结果 |
| status | Enum | 状态(pending/success/failed) |
| created_at | DateTime | 创建时间 |

**Relationships**:
- Belongs to `User`

---

### 7. UserBehaviorModel (用户行为模型)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 模型唯一标识 |
| user_id | UUID | 用户ID |
| operation_patterns | JSON | 操作序列模式 |
| time_patterns | JSON | 时间规律 |
| query_preferences | JSON | 查询偏好 |
| recommendation_feedback | JSON | 推荐反馈 |
| model_version | String | 模型版本 |
| updated_at | DateTime | 更新时间 |

**Relationships**:
- Belongs to `User`

---

### 8. PersonalizedRecommendation (个性化推荐)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 推荐唯一标识 |
| user_id | UUID | 用户ID |
| type | Enum | 推荐类型(action/time/content) |
| content | JSON | 推荐内容 |
| trigger_context | JSON | 触发场景 |
| is_relevant | Boolean | 用户是否认为相关 |
| feedback | String | 用户反馈 |
| created_at | DateTime | 创建时间 |

**Relationships**:
- Belongs to `User`

---

### 9. AuditLog (审计日志)

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | 日志唯一标识 |
| user_id | UUID | 用户ID |
| action | String | 操作类型 |
| resource | String | 资源类型 |
| resource_id | UUID | 资源ID |
| details | JSON | 详情 |
| ip_address | String | IP地址 |
| created_at | DateTime | 创建时间 |

---

## State Transitions

### ReminderRule
```
DISABLED → ENABLED (手动启用)
ENABLED → DISABLED (手动禁用)
ENABLED → TRIGGERED (条件满足)
TRIGGERED → READ (用户查看)
READ → CLICKED (用户点击处理)
```

### VoiceCommand
```
PENDING → PROCESSING (开始识别)
PROCESSING → SUCCESS (识别成功)
PROCESSING → FAILED (识别失败)
SUCCESS → EXECUTING (执行命令)
EXECUTING → COMPLETED (执行完成)
EXECUTING → ERROR (执行失败)
```

---

## Data Validation Rules

### User
- name: required, max 100 chars
- department: required
- role: required, enum(医生, 护士, 技师, 主任, 管理员)

### BusinessSystem
- name: required, unique
- url: required, valid URL format
- auth_config: encrypted storage

### ReminderRule
- user_id: required, foreign key
- trigger_type: required, enum
- content: required, max 500 chars

---

## Storage Strategy

| Entity | Storage | Encryption |
|--------|---------|------------|
| User | SQLite | Config encrypted |
| BusinessSystem | SQLite | Auth config encrypted |
| AssistantSession | SQLite + File | Messages encrypted |
| ReminderRule | SQLite | - |
| ReminderRecord | SQLite | - |
| VoiceCommand | File | Audio encrypted |
| UserBehaviorModel | SQLite + File | Model encrypted |
| PersonalizedRecommendation | SQLite | - |
| AuditLog | SQLite | - |
