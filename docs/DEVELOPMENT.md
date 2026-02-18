# EW Desktop Agent 开发指南

智能桌面客户端开发指南，包含环境配置、编译构建和项目结构说明。

## 项目简介

EW Desktop Agent 是一个基于 Tauri 2.x 的智能桌面客户端，用于访问医疗信息系统（RIS/PIS/EIS），提供 AI 助手、语音交互、风险控制等功能。

## 技术栈

| 组件 | 技术 |
|------|------|
| 桌面框架 | Tauri 2.x (Rust) |
| 前端 | TypeScript + React + Vite |
| 浏览器引擎 | WebView2 |
| 本地存储 | SQLite (rusqlite) |
| AI 网关 | .NET Core 8.0 |
| 网关数据库 | PostgreSQL |
| 语音唤醒 | Snowboy + Silero VAD |
| 语音识别 | 第三方 ASR API |
| 语音合成 | Kokoro TTS + 云端兜底 |

## 环境要求

### Windows 开发环境

1. **Node.js** >= 18.0
2. **Rust** >= 1.70
3. **.NET SDK** >= 8.0
4. **Visual Studio Build Tools** 2022+ (C++ 构建工具)
5. **WebView2 Runtime** (Windows 10/11 已内置)

### 检查环境

```powershell
# 检查 Node.js
node --version

# 检查 Rust
rustc --version
cargo --version

# 检查 .NET
dotnet --version
```

## 项目结构

```
EWDesktopAgent/
├── src/                    # 前端源码 (TypeScript/React)
│   ├── components/         # React 组件
│   ├── styles/            # 样式文件
│   └── App.tsx            # 主应用
├── src-tauri/             # Tauri 后端 (Rust)
│   ├── src/
│   │   ├── lib.rs         # 入口和命令注册
│   │   ├── main.rs        # 主程序
│   │   ├── browser.rs     # 浏览器管理
│   │   ├── config.rs      # 配置管理
│   │   ├── storage.rs    # SQLite 存储
│   │   ├── ai.rs         # AI 客户端
│   │   ├── voice.rs      # 语音服务
│   │   ├── auth.rs       # 认证模块
│   │   ├── reminder.rs   # 提醒模块
│   │   ├── personalization.rs  # 个性化学习
│   │   ├── file_ops.rs   # 文件操作
│   │   ├── downloader.rs # 下载管理
│   │   ├── notification.rs     # 通知
│   │   └── core/security/       # 安全模块
│   └── Cargo.toml        # Rust 依赖
├── ai-gateway/           # AI 网关 (.NET Core)
│   ├── src/
│   │   ├── AIGateway.API/      # API 项目
│   │   ├── AIGateway.Domain/   # 领域模型
│   │   └── AIGateway.Infrastructure/  # 基础设施
│   └── tests/            # 测试项目
├── .github/workflows/    # CI/CD 配置
├── specs/                # 项目规范文档
└── package.json          # NPM 配置
```

## 快速开始

### 1. 克隆项目

```bash
git clone <repository-url>
cd EWDesktopAgent
```

### 2. 安装前端依赖

```bash
npm install
```

### 3. 编译前端

```bash
# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build
```

### 4. Rust 开发（可选）

```bash
cd src-tauri

# 检查代码
cargo check

# 运行测试
cargo test

# 调试构建
cargo build

# 发布构建
cargo build --release
```

### 5. AI 网关开发

```bash
cd ai-gateway

# 恢复依赖
dotnet restore

# 运行开发服务器
dotnet run --project src/AIGateway.API

# 构建
dotnet build

# 发布
dotnet publish
```

## 构建说明

### 客户端构建

```bash
# 完整构建（前端 + Rust）
npm run tauri build

# 构建产物位置
# src-tauri/target/release/bundle/nsis/
```

### 网关 Docker 镜像

```bash
cd ai-gateway

# 构建镜像
docker build -t ew-ai-gateway:latest -f src/AIGateway.API/Dockerfile .

# 或使用 docker-compose
docker-compose up -d
```

### 发布到 GitHub Release

1. 创建 GitHub Personal Access Token
2. 在仓库设置中添加 `GITHUB_TOKEN` secret
3. 推送标签触发发布

```bash
# 创建版本标签
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

## 配置说明

### 客户端配置

配置文件位置: `%APPDATA%\EWDesktopAgent\config.json`

```json
{
  "business_systems": [
    { "id": "ris", "name": "RIS", "url": "http://localhost:8080/ris", "enabled": true },
    { "id": "pis", "name": "PIS", "url": "http://localhost:8080/pis", "enabled": true },
    { "id": "eis", "name": "EIS", "url": "http://localhost:8080/eis", "enabled": true }
  ],
  "user_preferences": {
    "language": "zh-CN",
    "theme": "light",
    "voice_enabled": true,
    "memory_limit_mb": 500
  }
}
```

### 网关配置

配置文件: `ai-gateway/src/AIGateway.API/appsettings.json`

```json
{
  "ConnectionStrings": {
    "PostgreSQL": "Host=localhost;Database=aigateway;Username=postgres;Password=your-password"
  },
  "FastGPT": {
    "BaseUrl": "https://api.fastgpt.in",
    "ApiKey": "your-fastgpt-api-key"
  }
}
```

## 开发注意事项

### Rust 部分

- 使用 `tracing` 进行日志记录
- 日志位置: `%LOCALAPPDATA%\EWDesktopAgent\logs\`
- 数据库位置: `%LOCALAPPDATA%\EWDesktopAgent\data.db`

### 前端部分

- 使用 Tauri 2.x API 与后端通信
- 组件位于 `src/components/`
- 样式位于 `src/components/*.css`

### 语音功能

语音功能需要额外的模型文件:
- Snowboy 唤醒词模型
- Silero VAD 模型
- Kokoro TTS 模型

请从对应官网下载模型文件到 `src-tauri/resources/` 目录。

## 常见问题

### WebView2 未安装

Windows 10/11 通常已内置 WebView2。如遇问题，下载 [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)。

### Rust 编译错误

确保安装了 C++ 构建工具:
```powershell
# 使用 rustup 安装
rustup default stable
rustup update
```

### 端口被占用

前端开发服务器默认端口 1420，修改 `vite.config.ts` 可更改。

## 相关链接

- [Tauri 文档](https://tauri.app/v1/guides/)
- [React 文档](https://react.dev/)
- [.NET 文档](https://docs.microsoft.com/en-us/dotnet/)
- [FastGPT API](https://fastgpt.cn/)
