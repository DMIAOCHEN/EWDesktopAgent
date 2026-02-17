# Implementation Plan: 智能桌面客户端

**Branch**: `001-intelligent-desktop-client` | **Date**: 2026-02-17 | **Spec**: specs/001-intelligent-desktop-client/spec.md
**Input**: Feature specification from `/specs/001-intelligent-desktop-client/spec.md`

## Summary

智能桌面客户端是一款基于Tauri框架的Windows桌面应用，通过集成WebView2浏览器引擎，为医疗信息化公司提供智能化升级方案。核心功能包括：内置浏览器访问现有业务系统(RIS/PIS/EIS)、AI智能网页助手、语音交互、智能提醒和个性化学习。

服务端架构：设计AI网关处理系统Token到FastGPT的鉴权转换，制定业务系统能力注册标准和前端桥接协议，使现有业务系统能够被AI智能体调用。

## Technical Context

**Language/Version**: Rust 1.75+ (Tauri 2.x) / TypeScript (前端) / .NET Core 8.0 (AI网关)
**Primary Dependencies**: Tauri 2.x, WebView2, SQLite, FastGPT API, Snowboy, Silero VAD, Kokoro TTS
**Storage**: SQLite (本地) + PostgreSQL (网关服务端)
**Testing**: Rust (cargo test), Vitest (前端), >80% coverage required
**Target Platform**: Windows 10/11 (64位)
**Project Type**: 桌面应用 + 服务端网关
**Performance Goals**: 内存<500MB, 启动<5秒, 单标签页<200MB
**Constraints**: 2GB内存环境可用, 离线模式支持, 医疗数据合规
**Scale**: 1000+并发用户
**User Hierarchy**: 客户 -> 机构 -> 系统 (三层结构)
**Integration**: AI网关 + 业务系统能力注册 + 前端桥接 + 风险管控 + 本地文件操作

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I: Modular Architecture ✅
- **Requirement**: Every feature as self-contained module
- **Status**: PASS - 采用多模块架构 (core/features/shared)

### Principle II: Test-First (NON-NEGOTIABLE) ✅
- **Requirement**: TDD mandatory, >80% coverage
- **Status**: PASS - 测试框架已确定

### Principle III: Observability ✅
- **Requirement**: Structured logs, traceable, error context
- **Status**: PASS - 日志模块设计已包含

### Principle IV: Security & Privacy ✅
- **Requirement**: Encryption, input validation, least-privilege, security review
- **Status**: PASS - 网关统一鉴权，审计日志

### Principle V: Simplicity ✅
- **Requirement**: YAGNI, reject unnecessary complexity
- **Status**: PASS - MVP优先，功能分阶段

## Architecture Overview

### 客户端架构 (Tauri)
```
src/
├── main/                    # 主进程入口 (Rust)
├── renderer/                # 前端界面 (TypeScript)
├── core/                   # 核心模块
│   ├── browser/           # WebView2封装
│   ├── ai/               # AI服务客户端
│   ├── voice/            # 语音交互
│   └── storage/          # SQLite存储
├── features/              # 功能模块
│   ├── assistant/        # 智能助手
│   ├── reminders/        # 智能提醒
│   └── personalization/  # 个性化学习
└── shared/               # 共享组件
    ├── config/
    ├── logging/
    └── security/
```

### 服务端架构 (AI网关 - .NET Core)
```
ai-gateway/
├── src/
│   ├── EW.AiGateway.Api/           # API层
│   ├── EW.AiGateway.Core/          # 核心业务
│   ├── EW.AiGateway.Infrastructure/ # 基础设施
│   └── EW.AiGateway.Sdk/           # SDK
├── docs/                             # 集成文档
└── samples/                         # 示例代码
```

### 客户端语音模块
```
src/
├── core/
│   ├── voice/
│   │   ├── wakeword/       # 唤醒模块 (Snowboy + Silero VAD)
│   │   ├── asr/            # 语音识别 (第三方API)
│   │   ├── tts/            # 语音合成 (Kokoro + 云端)
│   │   └── audio/          # 音频处理
```

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| AI网关 | 需要统一鉴权、Token转换、审计 | 直接调用无法控制权限 |
| 能力注册标准 | 需要让AI理解业务能力 | 无标准AI无法调用业务 |
| 前端桥接协议 | 需要让AI操作网页 | 无协议无法自动化 |
| 风险分级管控 | 医疗数据需要安全管控 | 无管控无法满足合规 |
| 本地文件操作 | 用户需要文件整理能力 | 无此能力工具价值降低 |

---

## Design Deliverables

已生成的设计文档：

| 文档 | 路径 | 说明 |
|------|------|------|
| **research.md** | specs/001-intelligent-desktop-client/research.md | 技术选型和架构设计 |
| **architecture.md** | specs/001-intelligent-desktop-client/architecture.md | 网关和集成架构详细设计 |
| **data-model.md** | specs/001-intelligent-desktop-client/data-model.md | 数据模型(含用户体系和网关) |
| **contracts/api.md** | specs/001-intelligent-desktop-client/contracts/api.md | API接口定义 |
| **quickstart.md** | specs/001-intelligent-desktop-client/quickstart.md | 快速入门指南 |

### 架构设计要点

1. **AI网关设计**
   - Token转换: 系统Token → FastGPT凭证
   - 能力注册: 统一描述业务API
   - 审计日志: 所有AI调用可追溯

2. **后端集成标准**
   - 能力注册格式 (Capability Schema)
   - API描述规范 (输入/输出/权限)
   - 注册流程 (启动注册/心跳/手动)

3. **前端集成标准**
   - 桥接组件 (IntelligentAgentBridge)
   - 动作执行协议 (Action Protocol)
   - 页面上下文标准 (Page Context)

4. **风险分级管控**
   - 三级风险评估: 低/中/高
   - 确认机制: 弹窗确认/预览确认/直接执行
   - 白名单: 用户可设置信任操作

5. **本地文件操作**
   - 支持操作: 读写/移动/复制/删除/整理
   - 安全控制: 目录访问限制
   - 文件整理: 按日期/类型/名称自动分类

---

## Next Steps

1. **Phase 1**: AI网关开发 (后端团队)
2. **Phase 2**: 能力注册规范定稿 (架构组)
3. **Phase 3**: 后端SDK开发 (各语言)
4. **Phase 4**: 前端SDK开发 (前端团队)
5. **Phase 5**: 业务系统接入 (各系统负责人)

运行 `/speckit.tasks` 生成客户端实现任务清单。
