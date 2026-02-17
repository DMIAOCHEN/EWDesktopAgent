# API Contracts: 智能桌面客户端

## Gateway API

### POST /api/gateway/auth/map
系统Token转换为FastGPT凭证

**Request**:
```json
{
  "system_token": "string",
  "requested_capabilities": ["ris:read", "pis:write"]
}
```

**Response**:
```json
{
  "mapped_token": "string",
  "expires_in": 3600,
  "allowed_capabilities": ["ris:read"]
}
```

### POST /api/gateway/capabilities/register
业务系统注册能力

**Request**:
```json
{
  "system": "ris",
  "version": "1.0.0",
  "endpoint": "http://ris-server/api",
  "capabilities": [
    {
      "id": "ris.query.patient",
      "name": "查询患者",
      "description": "根据ID查询患者信息",
      "input_schema": {},
      "output_schema": {},
      "auth_required": "ris:read"
    }
  ]
}
```

**Response**:
```json
{
  "success": true,
  "registered_count": 5
}
```

### GET /api/gateway/capabilities
获取所有可用能力

**Request**: (Authorization header required)

**Response**:
```json
{
  "capabilities": [
    {
      "system": "ris",
      "id": "ris.query.patient",
      "name": "查询患者",
      "description": "根据ID查询患者信息",
      "keywords": ["患者", "查询", "信息"]
    }
  ]
}
```

### POST /api/gateway/ai/request
AI请求统一入口

**Request**:
```json
{
  "user_message": "帮我查一下患者王建的CT检查",
  "context": {
    "current_system": "ris",
    "current_page": "report_edit",
    "page_data": {}
  },
  "options": {
    "stream": true
  }
}
```

**Response**:
```json
{
  "ai_response": "好的，我帮您查询...",
  "actions": [
    {
      "type": "execute",
      "system": "ris",
      "capability": "ris.query.patient",
      "params": {"patientName": "王建", "examType": "CT"}
    }
  ],
  "session_id": "uuid"
}
```

---

## Risk Management API

### POST /api/risk/assess
评估操作风险

**Request**:
```json
{
  "operation": {
    "type": "send_email",
    "target": "patient_report",
    "scope": "external"
  },
  "context": {}
}
```

**Response**:
```json
{
  "level": "high",
  "score": 85,
  "factors": ["data_sensitivity", "external_scope"],
  "requires_confirmation": true,
  "message": "此操作涉及数据外发，需要确认"
}
```

### POST /api/risk/confirm
确认操作

**Request**:
```json
{
  "operation_id": "uuid",
  "confirmed": true
}
```

**Response**:
```json
{
  "success": true,
  "can_execute": true
}
```

### GET /api/risk/rules
获取风险规则列表

**Response**:
```json
{
  "rules": [
    {
      "id": "uuid",
      "name": "发送邮件",
      "pattern": "*.send_email",
      "level": "high",
      "requires_confirmation": true
    }
  ]
}
```

### POST /api/risk/whitelist
添加白名单

**Request**:
```json
{
  "pattern": "search_*",
  "risk_level": "low",
  "expires_in_days": 30
}
```

**Response**:
```json
{
  "success": true,
  "whitelist_id": "uuid"
}
```

---

## File Operations API

### POST /api/files/list
列出目录内容

**Request**:
```json
{
  "path": "C:/Users/test/Documents/reports",
  "include_hidden": false
}
```

**Response**:
```json
{
  "files": [
    {"name": "report_2024.pdf", "type": "file", "size": 1024000, "modified": "2024-01-15T10:30:00Z"},
    {"name": "archives", "type": "directory", "modified": "2024-01-10T08:00:00Z"}
  ]
}
```

### POST /api/files/organize
整理文件

**Request**:
```json
{
  "source_dir": "C:/Users/test/Desktop/reports",
  "rule": {
    "type": "by_date",
    "options": {
      "format": "YYYY-MM",
      "field": "modified"
    }
  }
}
```

**Response**:
```json
{
  "success": true,
  "operations": [
    {"from": "report1.pdf", "to": "2024-01/report1.pdf"},
    {"from": "report2.pdf", "to": "2024-02/report2.pdf"}
  ]
}
```

### POST /api/files/delete
删除文件/文件夹

**Request**:
```json
{
  "path": "C:/Users/test/Documents/temp",
  "recursive": true
}
```

**Response**:
```json
{
  "success": true,
  "deleted_count": 5
}
```

---

## Authentication API

## Authentication API

### POST /api/auth/login
用户登录

**Request**:
```json
{
  "username": "string",
  "password": "string",
  "device_id": "string"
}
```

**Response**:
```json
{
  "token": "string",
  "user": {
    "id": "uuid",
    "name": "string",
    "department": "string",
    "role": "string"
  },
  "expires_at": "datetime"
}
```

### POST /api/auth/logout
用户登出

**Request**: (Authorization header required)

**Response**:
```json
{
  "success": true
}
```

---

## Business System API

### GET /api/business-systems
获取可用的业务系统列表

**Request**: (Authorization header required)

**Response**:
```json
{
  "systems": [
    {
      "id": "uuid",
      "name": "string",
      "url": "string",
      "icon": "string",
      "is_enabled": true
    }
  ]
}
```

---

## AI Assistant API

### POST /api/assistant/chat
发送聊天消息

**Request**:
```json
{
  "message": "string",
  "context": {
    "current_system": "uuid",
    "current_url": "string",
    "page_elements": []
  },
  "session_id": "uuid"
}
```

**Response**:
```json
{
  "response": "string",
  "actions": [
    {
      "type": "click|input|navigate|search",
      "target": "string",
      "value": "string"
    }
  ],
  "session_id": "uuid"
}
```

### POST /api/assistant/execute
执行AI操作

**Request**:
```json
{
  "action": {
    "type": "click",
    "selector": "#submit-btn"
  }
}
```

**Response**:
```json
{
  "success": true,
  "result": "any"
}
```

---

## Reminder API

### GET /api/reminders/rules
获取提醒规则列表

**Request**: (Authorization header required)

**Response**:
```json
{
  "rules": [
    {
      "id": "uuid",
      "name": "string",
      "trigger_type": "time|event|api",
      "trigger_config": {},
      "content": "string",
      "target_url": "string",
      "is_enabled": true
    }
  ]
}
```

### POST /api/reminders/rules
创建提醒规则

**Request**:
```json
{
  "name": "string",
  "trigger_type": "time",
  "trigger_config": {
    "cron": "0 8 * * 1-5"
  },
  "content": "您有{count}份报告待审核",
  "target_url": "/ris/reports/pending"
}
```

**Response**:
```json
{
  "id": "uuid",
  "name": "string",
  "is_enabled": true
}
```

### PUT /api/reminders/rules/{id}
更新提醒规则

### DELETE /api/reminders/rules/{id}
删除提醒规则

---

## Voice API

### POST /api/voice/stt
语音转文字

**Request**: (multipart/form-data)
- audio: binary
- language: string (default: "zh-CN")

**Response**:
```json
{
  "text": "帮我查一下患者王建的检查记录",
  "confidence": 0.95
}
```

### POST /api/voice/tts
文字转语音

**Request**:
```json
{
  "text": "本周共完成CT检查856例",
  "voice": "zh-CN-XiaoxiaoNeural"
}
```

**Response**: (audio/binary)

---

## Personalization API

### GET /api/personalization/model
获取用户行为模型

**Request**: (Authorization header required)

**Response**:
```json
{
  "operation_patterns": {},
  "time_patterns": {},
  "query_preferences": {},
  "model_version": "1.0.0",
  "updated_at": "datetime"
}
```

### POST /api/personalization/feedback
提交推荐反馈

**Request**:
```json
{
  "recommendation_id": "uuid",
  "is_relevant": true,
  "feedback": "string"
}
```

**Response**:
```json
{
  "success": true
}
```

---

## Sync API

### POST /api/sync/upload
上传本地数据

**Request**:
```json
{
  "behavior_model": {},
  "preferences": {},
  "last_sync_at": "datetime"
}
```

**Response**:
```json
{
  "success": true,
  "synced_at": "datetime"
}
```

### POST /api/sync/download
下载服务端数据

**Request**: (Authorization header required)

**Response**:
```json
{
  "behavior_model": {},
  "preferences": {},
  "synced_at": "datetime"
}
```
