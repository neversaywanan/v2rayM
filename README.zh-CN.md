# v2rayM

[English](./README.md) | [简体中文](./README.zh-CN.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](./LICENSE)
[![Desktop: Tauri 2](https://img.shields.io/badge/Desktop-Tauri%202-24C8D8)](https://tauri.app/)
[![Frontend: React 18](https://img.shields.io/badge/Frontend-React%2018-61DAFB)](https://react.dev/)
[![Backend: Rust](https://img.shields.io/badge/Backend-Rust-000000)](https://www.rust-lang.org/)

一个面向远端 Linux 服务器 V2Ray 管理场景的本地桌面控制台。

## 项目概述

v2rayM 是一个本地桌面应用，用于连接已经部署并运行 [V2Ray Core](https://github.com/v2fly/v2ray-core) 的远端 Linux 服务器。它提供了一套聚焦于运维场景的图形界面，用来查看当前服务器配置、导入订阅链接、应用节点、切换出站以及验证连通性，从而避免在 SSH 会话中反复手工编辑远端 JSON 配置。

更直接地说，这个项目服务于这样一类用户：你已经在服务器上运行了 V2Ray，但希望在本地电脑上通过一个更高效、更直观的 GUI 来完成日常管理。

## 为什么需要 v2rayM

远端维护 V2Ray 往往是一套割裂的操作链路：

- 先通过 SSH 登录服务器
- 手动定位并编辑当前使用中的 V2Ray 配置文件
- 同步订阅节点
- 切换当前激活的出站或路由行为
- 重启服务并确认流量是否真的正常工作

v2rayM 的目标不是替代 V2Ray 本身，而是把这条高频运维链路收敛到一个桌面应用里，让远端管理更高效、更稳定，也更容易重复执行。

## 产品预览

<table>
  <tr>
    <td align="center" width="50%">
      <img src="./screenshot/login.png" alt="SSH 登录界面" width="100%" />
      <br />
      <sub>通过专门设计的 SSH 登录界面连接远端 Linux 服务器。</sub>
    </td>
    <td align="center" width="50%">
      <img src="./screenshot/screenshot.png" alt="配置概览面板" width="100%" />
      <br />
      <sub>在一个界面内查看当前节点、核心配置与远端代理运行状态。</sub>
    </td>
  </tr>
</table>

## 它能帮你完成什么

- 在本地桌面应用中通过 SSH 连接远端 Linux 服务器
- 读取并摘要展示当前 V2Ray 配置
- 导入订阅链接并解析 `vmess`、`vless`、`trojan`、`ss`、`ssr` 节点
- 将一个或多个节点应用到远端服务器配置
- 通过 GUI 切换当前激活的出站节点，而不是手工改配置
- 修改入站监听端口
- 执行远端连通性与延迟相关检查
- 在本地保存常用连接信息、订阅地址和语言偏好
- 使用内置中英文界面文案

## 典型工作流

1. 在本地打开应用，并通过 SSH 连接远端 Linux 服务器。
2. 读取目标服务器上的当前 V2Ray 配置。
3. 导入订阅链接并解析可用节点。
4. 将节点写回服务器配置，或切换当前激活的出站节点。
5. 在需要时调整入站监听端口。
6. 运行连通性测试，确认代理链路处于健康状态。

## 项目边界

v2rayM 被明确定位为“远端管理客户端”。

- 它管理的是一个已经存在的 V2Ray 部署。
- 它不是 V2Ray 安装器。
- 它默认目标服务已经部署完成，并可通过 `systemctl restart v2ray` 进行管理。
- 默认读取的远端配置路径是 `/etc/v2ray/config.json`。

## 技术栈

- 前端：React 18 + TypeScript + Vite
- 桌面壳：Tauri v2
- 后端命令层：Rust + Tokio + Reqwest + SSH2
- 测试：Vitest 与 Rust 测试

## 运行要求

在本地运行前，请先确保：

- Node.js 20+
- npm 10+
- Rust stable 工具链
- 当前操作系统已满足 Tauri 依赖
- 有一台可通过 SSH 访问的 Linux 服务器
- 远端服务器已经安装并运行 V2Ray
- 目标服务器上的 V2Ray 配置文件可读

## 快速开始

### 1. 安装依赖

```bash
npm install
```

### 2. 启动开发模式

```bash
npm run tauri dev
```

### 3. 运行测试

```bash
npm test
cargo test --manifest-path src-tauri/Cargo.toml
```

### 4. 构建桌面应用

```bash
npm run tauri build
```

## 项目结构

```text
.
|-- screenshot/               # README 截图与产品预览素材
|-- src/                      # React 界面、状态、国际化与工具函数
|-- src-tauri/                # Rust 命令、SSH 客户端、解析器与配置拼装
|-- public/                   # 静态资源
|-- .github/                  # GitHub Issue 与 Pull Request 模板
|-- README.md                 # 英文 README
|-- README.zh-CN.md           # 中文 README
|-- CONTRIBUTING.md           # 英文贡献指南
|-- CONTRIBUTING.zh-CN.md     # 中文贡献指南
|-- CODE_OF_CONDUCT.md        # 英文行为准则
|-- CODE_OF_CONDUCT.zh-CN.md  # 中文行为准则
|-- SECURITY.md               # 英文安全说明
|-- SECURITY.zh-CN.md         # 中文安全说明
|-- CHANGELOG.md              # 英文变更记录
|-- CHANGELOG.zh-CN.md        # 中文变更记录
|-- LICENSE                   # MIT 协议原文（权威文本）
|-- LICENSE.zh-CN.md          # MIT 协议中文参考译文
```

## 文档索引

- 英文贡献指南：[CONTRIBUTING.md](./CONTRIBUTING.md)
- 中文贡献指南：[CONTRIBUTING.zh-CN.md](./CONTRIBUTING.zh-CN.md)
- 英文安全说明：[SECURITY.md](./SECURITY.md)
- 中文安全说明：[SECURITY.zh-CN.md](./SECURITY.zh-CN.md)
- 英文行为准则：[CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)
- 中文行为准则：[CODE_OF_CONDUCT.zh-CN.md](./CODE_OF_CONDUCT.zh-CN.md)

## 许可证

本项目采用 MIT License。请查看 [LICENSE](./LICENSE) 获取权威协议原文，查看 [LICENSE.zh-CN.md](./LICENSE.zh-CN.md) 获取中文参考译文。
