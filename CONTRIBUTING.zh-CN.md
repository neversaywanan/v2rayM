# 参与贡献 v2rayM

[English](./CONTRIBUTING.md) | [简体中文](./CONTRIBUTING.zh-CN.md)

感谢你考虑为 v2rayM 做出贡献。

这份文档说明了我们如何让变更更易于评审、更容易验证，也更符合项目方向。

## 基本原则

- 保持 PR 聚焦。一个问题对应一组改动。
- 优先提交小而清晰、便于评审的变更，而不是把重构和功能混在一起。
- 只要改动影响到用户行为，请在同一个 PR 中同步更新相关文档。
- 如果改动涉及 SSH、配置写回或订阅解析逻辑，请格外注意回归风险。
- 不要提交任何密钥、真实服务器凭据或生产环境订阅地址。

## 开发环境

### 前置要求

- Node.js 20+
- npm 10+
- Rust stable
- 当前操作系统所需的 Tauri 依赖

### 安装依赖

```bash
npm install
```

### 以开发模式启动应用

```bash
npm run tauri dev
```

### 运行前端测试

```bash
npm test
```

### 运行 Rust 测试

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

### 构建应用

```bash
npm run tauri build
```

## 分支命名

尽量使用清晰、可读的分支名。

示例：

- `feat/subscription-bulk-sync`
- `fix/ssh-timeout-handling`
- `docs/bilingual-docs`
- `refactor/config-composer`

## Commit 信息

建议使用 Conventional Commits。

示例：

- `feat: support bulk applying subscription nodes`
- `fix: preserve display names when switching outbound`
- `docs: add bilingual repository documentation`
- `test: add parser coverage for trojan links`

## Pull Request 检查清单

提交 PR 前，请尽量确认：

- 分支已经同步到目标分支的最新状态。
- 改动原因明确，范围清晰。
- 行为发生变化时，相关测试已经补充或更新。
- 相关本地测试已经执行。
- 涉及界面变化时，已附上截图。
- 新增用户文案时，`zh` 和 `en` 两套语言表都已更新。
- 如果安装、行为或项目结构发生变化，相关文档已同步更新。

## 编码建议

### 前端

- 保持 React 组件清晰可读，避免把无关的 UI 逻辑混在一起。
- 尽量复用 store 和工具函数，不要重复实现客户端逻辑。
- 引入非简单字符串处理或解析逻辑时，请补充工具函数测试。
- 新增面向用户的文案时，请同时更新 `zh` 和 `en` 翻译。

### Rust / Tauri

- 保持 command handler 职责单一，把可复用逻辑放入更聚焦的模块。
- 只要用户可以据此采取行动，就优先返回明确的错误，而不是静默失败。
- 在拼装或修改远端 JSON 配置时，尽量保留现有配置结构。
- 修改解析器、配置拼装器或错误行为时，请同步补充或更新测试。

## 测试要求

不是每次只改文档都需要完整构建，但只要涉及代码改动，就应该做相应验证。

推荐检查：

```bash
npm test
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

请根据改动范围运行合适的检查。如果有未执行的项目，请在 PR 描述中说明。

## Bug 反馈建议

提交缺陷反馈时，建议至少包含：

- 你的预期行为
- 实际发生了什么
- 操作系统与应用环境信息
- 目标服务器是否运行了 V2Ray 和 `systemd`
- 复现步骤
- 已脱敏的日志或截图

## 功能建议

欢迎提出功能建议，特别是以下方向：

- 提升远端配置编辑的安全性
- 提高常见订阅格式的兼容性
- 改善新用户引导和错误提示
- 提升打包、发布和跨平台体验

## 需要提前沟通的改动

如果是下面这些较大的改动，建议先开 discussion 或 draft PR：

- 配置拼装行为
- 回滚语义
- 解析器兼容性变更
- SSH 传输或认证流程
- 跨平台打包与签名

## 问题沟通

如果有不清楚的地方，请带着上下文和你的思路来提 issue。越早对齐方向，通常越能节省大家的时间。
