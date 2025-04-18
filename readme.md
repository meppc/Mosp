# Mosp

Mosp 是一个跨平台的白板应用程序，使用 Rust 和 Slint 开发。

## 功能特点

- 无限画布
- 多种绘图工具
- 实时协作
- 跨平台支持
- 高性能渲染

## 开发环境要求

- Rust 1.75 或更高版本
- Cargo 包管理器
- CMake (用于构建 Slint)
- C++ 编译器

## 安装依赖

1. 安装 Rust：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装系统依赖：

macOS:
```bash
brew install cmake
```

Linux (Ubuntu/Debian):
```bash
sudo apt-get update
sudo apt-get install cmake build-essential
```

Windows:
- 安装 Visual Studio Build Tools
- 安装 CMake

## 构建和运行

1. 克隆仓库：
```bash
git clone https://github.com/yourusername/mosp.git
cd mosp
```

2. 构建项目：
```bash
cargo build
```

3. 运行项目：
```bash
cargo run
```

## 项目结构

```
mosp/
├── src/                # Rust 源代码
│   ├── main.rs        # 程序入口
│   ├── app.rs         # 应用程序核心
│   ├── config.rs      # 配置管理
│   ├── error.rs       # 错误处理
│   ├── storage.rs     # 数据存储
│   ├── window.rs      # 窗口管理
│   └── ui/            # UI 组件
├── ui/                # Slint UI 定义
│   └── main.slint     # 主窗口 UI
├── docs/              # 文档
├── tests/             # 测试
└── Cargo.toml         # 项目配置
```

## 开发规范

请参考 [开发规范文档](docs/code.md) 了解项目的编码规范和指南。

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

MIT License

# 基于 Rust 的跨平台白板软件技术实现方案（功能上参考 Miro）

## 核心模块技术选型

| 模块          | 技术方案                          | 性能指标           |
|---------------|-----------------------------------|--------------------|
| 图形渲染      | slint                             | 4K@60FPS 渲染延迟  |
| 输入处理      | winit + core-graphics             | 200μs 事件响应     |
| 网络同步      | tokio + y-sweet CRDT              | 50ms 操作同步延迟  |
| 数据存储      | sled + SQLite                     | 10k ops/sec 吞吐量 |
| 安全加密      | rustls + ring                     | AES-NI 指令集加速  |

## 核心功能架构设计

### 图形渲染子系统
采用分层渲染架构实现矢量图形处理，底层使用 `raqote` 2D 图形库进行基本图形绘制[8]。通过 `glow` 抽象层集成 OpenGL/WebGPU 实现硬件加速渲染，支持 60FPS 动画效果[5]。设计基于 `serde` 的序列化协议实现画布状态持久化，采用增量式存储策略优化大文件处理效率。

#### Slint 跨平台渲染架构
Slint 是一个基于 Rust 的跨平台 GUI 框架，为本方案的图形渲染子系统提供了统一的解决方案。Slint 采用分层渲染架构，通过硬件加速的 2D 图形引擎提供高性能的矢量图形绘制能力。在不同平台上，Slint 使用以下技术实现跨平台渲染:

- **Windows**: 使用 DirectComposition 进行硬件加速渲染
- **macOS**: 基于 Core Graphics 框架实现原生绘图加速
- **iOS/iPadOS**: 集成 Metal API 提供笔迹平滑渲染
- **Android**: 利用 OpenGL ES 实现硬件加速图形处理

Slint 的跨平台特性确保了白板软件在不同操作系统上拥有一致的渲染效果和性能表现。通过 Slint 提供的丰富组件库和灵活的布局系统，我们可以高效地构建出功能完备的白板 UI。

