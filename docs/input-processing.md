# 输入处理模块

本文档详细介绍白板软件的输入处理模块设计与实现。该模块负责捕获、处理和优化各类输入设备的交互，为用户提供流畅自然的绘图体验。

## 设计目标

- 实现低延迟输入响应（<200μs）
- 支持多种输入设备（鼠标、触控屏、数位笔）
- 提供精确的压感和倾斜角度处理
- 实现自然的手势识别系统
- 确保跨平台行为一致性

## 输入处理架构

```
┌────────────────────────────────────────────────┐
│              应用层事件处理                    │
│       (工具行为、命令分发、状态更新)           │
├────────────────────────────────────────────────┤
│              输入事件解释器                    │
│   (手势识别、笔迹平滑、预测算法、事件转换)     │
├────────────────────────────────────────────────┤
│              平台无关事件系统                  │
│   (winit抽象事件、统一事件队列、调度器)        │
├───────────────┬────────────────┬───────────────┤
│  Windows Input │  macOS/iOS Input │ Linux Input  │
│    (WinAPI)    │  (Core Graphics) │  (X11/Wayland)│
└───────────────┴────────────────┴───────────────┘
```

## 核心技术实现

### 跨平台输入抽象

集成`winit`窗口系统处理多平台输入事件[12]，为各平台输入系统提供统一的抽象层：

```rust
// 输入事件处理器
pub struct InputHandler {
    event_queue: Arc<Mutex<VecDeque<InputEvent>>>,
    gesture_recognizer: GestureRecognizer,
    stroke_predictor: StrokePredictor,
    // 平台特定处理器
    #[cfg(target_os = "windows")]
    windows_handler: WindowsInputHandler,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    apple_handler: AppleInputHandler,
}

// 跨平台输入事件
#[derive(Clone, Debug)]
pub enum InputEvent {
    Pointer(PointerEvent),
    Touch(TouchEvent),
    Pencil(PencilEvent),
    Keyboard(KeyboardEvent),
    Gesture(GestureEvent),
}
```

### 低延迟事件处理

通过自定义事件分发器实现200μs级延迟的笔迹预测算法。关键优化包括：

1. **事件批处理** - 合并连续事件减少处理开销
2. **线程隔离** - 输入捕获与处理在独立线程执行
3. **预测算法** - 基于历史轨迹预测笔尖位置
4. **动态采样率** - 根据设备能力调整事件采样频率

### 专业笔设备支持

针对Apple Pencil开发专用驱动模块，使用`apple-rs`框架捕获压感倾斜数据[14]，实现1024级压感支持：

```rust
// Apple Pencil 事件处理示例
#[cfg(target_os = "ios")]
impl UIViewProcessor {
    fn handle_pencil_event(event: &PencilData) {
        let pressure = event.force() * 1024.0;
        let tilt = event.altitudeAngle().to_degrees();
        renderer.update_brush(pressure, tilt);
    }
}
```

### 手势识别系统

实现双指捏合缩放手势识别[11]和多手势组合检测：

```rust
pub struct GestureRecognizer {
    active_gestures: Vec<Box<dyn GestureDetector>>,
    touch_points: HashMap<TouchId, TouchPoint>,
    gesture_config: GestureConfig,
}

impl GestureRecognizer {
    pub fn process_touch_event(&mut self, event: &TouchEvent) -> Option<GestureEvent> {
        // 更新触摸点状态
        self.update_touch_points(event);
        
        // 检测手势
        for detector in &mut self.active_gestures {
            if let Some(gesture) = detector.detect(&self.touch_points) {
                return Some(gesture);
            }
        }
        None
    }
}
```

支持的手势类型包括：

- 单指拖动 (平移画布)
- 双指捏合 (缩放画布)
- 双指旋转 (旋转对象)
- 长按手势 (上下文菜单)
- 多指轻扫 (历史导航)

## 笔迹平滑与优化

实现了多层级笔迹优化系统：

1. **实时平滑** - 应用贝塞尔样条插值减少抖动
2. **延迟渲染** - 绘制最终笔迹前应用额外平滑处理
3. **压力映射** - 自定义压力曲线满足不同绘图风格
4. **速度相关粗细** - 根据绘制速度调整笔迹宽度

```rust
pub struct StrokeProcessor {
    settings: StrokeSettings,
    points: VecDeque<StrokePoint>,
    // 速度和加速度缓存
    velocity: Point,
    prev_velocity: Point,
}

impl StrokeProcessor {
    pub fn process_point(&mut self, point: Point, pressure: f32, timestamp_us: u64) -> StrokePath {
        // 计算速度和加速度
        let velocity = self.calculate_velocity(point, timestamp_us);
        
        // 压力映射
        let adjusted_pressure = self.apply_pressure_curve(pressure);
        
        // 根据速度调整宽度
        let width = self.calculate_width(adjusted_pressure, velocity.length());
        
        // 生成平滑路径
        self.generate_smooth_path(point, width)
    }
}
```

## 平台特定优化

### Windows平台

1. 使用Windows Ink API获取更精确的笔输入
2. 优化WinTab驱动支持专业数位板
3. 实现Windows手势识别API集成

### macOS/iPadOS平台

1. 深度集成Apple Pencil 2的全部特性
2. 利用Force Touch触控板增强交互
3. 实现iPad侧边按钮辅助输入

### 触控优化策略

1. **触摸排斥** - 防止手掌误触
2. **工具优先级** - 笔输入优先于触摸输入
3. **触控延迟差异化** - 根据输入类型调整响应时间

## 测试与验证

1. **输入延迟测量** - 使用高速摄像机验证端到端延迟
2. **笔迹准确性测试** - 对比数字输入与物理绘制结果
3. **手势识别率** - 测量不同条件下手势检测准确率
4. **跨平台一致性** - 验证多平台输入行为差异

## 未来改进计划

1. 支持更多专业绘图设备 (Wacom, XP-Pen)
2. 实现基于机器学习的笔迹预测
3. 自定义手势编程系统
4. VR/AR输入模式支持

引用:
[11] https://github.com/jneem/gesture
[12] https://github.com/bevyengine/bevy/issues/14060
[14] https://github.com/briantkelley/apple-rs