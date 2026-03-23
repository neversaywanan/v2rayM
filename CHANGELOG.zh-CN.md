# 变更记录

[English](./CHANGELOG.md) | [简体中文](./CHANGELOG.zh-CN.md)

本文件用于记录项目中的重要变更。

格式上参考 Keep a Changelog。

## [v0.1.0] - 2026-03-23

### 新增

- 仓库级 README、贡献说明、安全说明和社区规范文档。
- 仓库级文档的简体中文版本。
- 中英双语的 GitHub Issue 与 Pull Request 模板。
- 英文 MIT 协议原文与中文参考译文。
- 基于 Tauri、React、TypeScript 与 Rust 的桌面应用基础结构。
- 面向已部署 V2Ray 的 Linux 服务器的 SSH 远端管理流程。
- 对 `vmess`、`vless`、`trojan`、`ss`、`ssr` 节点的订阅解析支持。
- 远端配置应用、出站切换、入站端口更新与连通性测试流程。

### 变更

- 刷新了 `.gitignore` 策略，忽略本地 docs、Tauri 构建产物和生成文件。
- 重构了 README，把“项目用途”和“解决的问题”放到最前面。
- 使用 `screenshot/` 目录中的图片替换了旧的单张截图，并重组为更规范的产品预览区域。
- 提升了 README 的整体措辞与项目定位表达，使其更接近成熟开源项目的呈现方式。
