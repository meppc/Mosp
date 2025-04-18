# 图形渲染子系统

本文档详细介绍白板软件的图形渲染子系统设计与实现。该子系统是整个应用的核心，负责高性能图形绘制和视觉呈现。

## 设计目标

- 实现流畅的矢量图形绘制（60FPS+）
- 支持复杂图形与自定义造型
- 确保跨平台一致的渲染效果
- 优化内存使用和渲染性能
- 提供可扩展的渲染管线

## 渲染架构

图形渲染子系统采用分层架构设计：

```
┌───────────────────────────────────────────┐
│              Slint UI层                   │
│    (组件、布局、交互元素、控件系统)       │
├───────────────────────────────────────────┤
│              画布渲染层                   │
│     (路径处理、图形绘制、效果渲染)        │
├───────────────────────────────────────────┤
│              渲染抽象层                   │
│  (glow抽象、平台特定渲染API适配、后端选择)│
├───────────────┬─────────────┬─────────────┤
│    DirectX    │    Metal    │   OpenGL    │
│   (Windows)   │ (macOS/iOS) │   (跨平台)  │
└───────────────┴─────────────┴─────────────┘
```

### Slint 跨平台渲染架构

Slint 是一个基于 Rust 的跨平台 GUI 框架，为本方案的图形渲染子系统提供了统一的解决方案。Slint 采用分层渲染架构，通过硬件加速的 2D 图形引擎提供高性能的矢量图形绘制能力。在不同平台上，Slint 使用以下技术实现跨平台渲染:

- **Windows**: 使用 DirectComposition 进行硬件加速渲染
- **macOS**: 基于 Core Graphics 框架实现原生绘图加速
- **iOS/iPadOS**: 集成 Metal API 提供笔迹平滑渲染
- **Android**: 利用 OpenGL ES 实现硬件加速图形处理

Slint 的跨平台特性确保了白板软件在不同操作系统上拥有一致的渲染效果和性能表现。通过 Slint 提供的丰富组件库和灵活的布局系统，我们可以高效地构建出功能完备的白板 UI。

## 底层渲染技术

### 矢量图形处理

底层使用 `raqote` 2D 图形库进行基本图形绘制[8]。该库提供了高性能的路径构建、填充和描边功能，支持复杂的图形组合操作。

```rust
// 矢量路径构建示例
let mut path = PathBuilder::new();
path.move_to(100.0, 100.0);
path.quad_to(150.0, 50.0, 200.0, 100.0);
let gradient = Gradient::linear(
    Point::new(0.0, 0.0),
    Point::new(200.0, 200.0),
    vec![
        GradientStop { position: 0.0, color: Color::BLUE },
        GradientStop { position: 1.0, color: Color::GREEN },
    ]
);
canvas.fill(&path, &gradient, &DrawOptions::new());
```

### 硬件加速

通过 `glow` 抽象层集成 OpenGL/WebGPU 实现硬件加速渲染，支持 60FPS 动画效果[5]。这种抽象使我们能够利用各平台的图形硬件特性，同时维护统一的渲染API。

## Slint 与白板功能集成

### 自定义白板组件

基于 Slint 开发了专用的白板组件，支持以下功能：

```rust
slint::slint! {
    export component Whiteboard {
        in property <bool> active;
        in property <string> current-tool: "pen";
        in property <color> stroke-color: #3a86ff;
        in property <float> stroke-width: 2.0;
        
        callback stroke-started(Point);
        callback stroke-moved(Point);
        callback stroke-ended(Point);
        
        // 白板画布实现
        Rectangle {
            width: root.width;
            height: root.height;
            background: white;
            
            // 绘制层...
            // 交互层...
        }
    }
}
```

### 图层系统

渲染子系统实现了多层画布结构：

1. **背景层** - 处理网格、背景色和纹理
2. **内容层** - 绘制主要图形和对象
3. **交互层** - 实时绘制、选择框和操作控件
4. **UI覆盖层** - 工具栏和控制面板

这种分层设计允许我们对不同类型的内容应用不同的渲染策略，优化性能。

## 状态持久化

设计基于 `serde` 的序列化协议实现画布状态持久化，采用增量式存储策略优化大文件处理效率。画布上的每个对象都实现了序列化/反序列化接口：

```rust
#[derive(Serialize, Deserialize)]
struct CanvasObject {
    id: Uuid,
    object_type: ObjectType,
    geometry: Vec<PathSegment>,
    style: ObjectStyle,
    transform: Transform2D,
    metadata: HashMap<String, Value>,
}
```

## 性能优化

1. **四叉树空间分区** - 优化大型画布的渲染效率
2. **视口裁剪** - 仅渲染可见区域内的对象
3. **对象缓存** - 缓存静态对象的渲染结果
4. **渐进式渲染** - 在平移/缩放操作中应用渐进式质量提升
5. **SIMD加速** - 利用SIMD指令集优化矩阵运算

## 适配平台特性

### Windows特定优化

1. 利用Direct2D加速路径渲染
2. 支持Windows Ink API捕获手写笔输入

### macOS/iOS特定优化

1. 利用Metal性能着色器加速复杂绘图
2. 优化处理Apple Pencil高精度输入
3. 利用Core Animation提升UI流畅度

## 关键实现类

```rust
// 渲染管理器
pub struct RenderManager {
    canvas_size: Size,
    view_transform: Transform2D,
    layers: Vec<Layer>,
    renderer: Box<dyn Renderer>,
    object_cache: ObjectCache,
}

// 渲染器接口
pub trait Renderer {
    fn begin_frame(&mut self);
    fn render_path(&mut self, path: &Path, style: &DrawStyle);
    fn render_text(&mut self, text: &str, position: Point, font: &Font);
    fn apply_effect(&mut self, effect: &Effect, region: &Region);
    fn end_frame(&mut self);
}
```

## 未来扩展

1. **WebAssembly支持** - 通过WebGL后端实现web版本
2. **GPU计算加速** - 利用计算着色器优化大型场景
3. **自定义滤镜系统** - 支持用户定义图形效果
4. **SVG/PDF渲染优化** - 提升矢量文件导入性能

引用:
[5] https://github.com/emilk/egui
[8] https://github.com/jrmuizel/raqote