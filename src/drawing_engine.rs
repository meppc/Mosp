use slint::SharedPixelBuffer;
use std::sync::Mutex;

// 定义绘图工具类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    Pencil,
    Line,
    Rectangle,
    Ellipse,
    Eraser,
}

impl From<String> for Tool {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pencil" => Tool::Pencil,
            "line" => Tool::Line,
            "rectangle" => Tool::Rectangle,
            "ellipse" => Tool::Ellipse,
            "eraser" => Tool::Eraser,
            _ => Tool::Pencil,
        }
    }
}

// 定义笔画点
#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
    pressure: f32,
}

// 定义笔画
#[derive(Debug, Clone)]
struct Stroke {
    points: Vec<Point>,
    color: (u8, u8, u8, u8),
    size: f32,
    tool: Tool,
}

// 绘图引擎
pub struct DrawingEngine {
    strokes: Mutex<Vec<Stroke>>,
    current_stroke: Mutex<Option<Stroke>>,
    current_tool: Mutex<Tool>,
    current_color: Mutex<(u8, u8, u8, u8)>,
    current_size: Mutex<f32>,
}

impl DrawingEngine {
    pub fn new() -> Self {
        DrawingEngine {
            strokes: Mutex::new(Vec::new()),
            current_stroke: Mutex::new(None),
            current_tool: Mutex::new(Tool::Pencil),
            current_color: Mutex::new((0, 0, 0, 255)), // 默认黑色
            current_size: Mutex::new(2.0),             // 默认笔刷大小
        }
    }

    pub fn set_tool(&self, tool: Tool) {
        *self.current_tool.lock().unwrap() = tool;
    }

    pub fn set_color(&self, r: f32, g: f32, b: f32, a: f32) {
        *self.current_color.lock().unwrap() = (
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (a * 255.0) as u8,
        );
    }

    pub fn set_brush_size(&self, size: f32) {
        *self.current_size.lock().unwrap() = size;
    }

    pub fn begin_stroke(&self, x: f32, y: f32) {
        let mut current_stroke = self.current_stroke.lock().unwrap();
        let tool = *self.current_tool.lock().unwrap();
        let color = *self.current_color.lock().unwrap();
        let size = *self.current_size.lock().unwrap();

        *current_stroke = Some(Stroke {
            points: vec![Point {
                x,
                y,
                pressure: 1.0, // 默认压力值，可以在支持压感的平台上进行扩展
            }],
            color,
            size,
            tool,
        });
    }

    pub fn continue_stroke(&self, x: f32, y: f32) {
        let mut current_stroke = self.current_stroke.lock().unwrap();
        if let Some(stroke) = current_stroke.as_mut() {
            stroke.points.push(Point {
                x,
                y,
                pressure: 1.0, // 默认压力值
            });
        }
    }

    pub fn end_stroke(&self) {
        let mut current_stroke = self.current_stroke.lock().unwrap();
        if let Some(stroke) = current_stroke.take() {
            let mut strokes = self.strokes.lock().unwrap();
            strokes.push(stroke);
        }
    }

    pub fn render(&self, painter: &slint::Painter) {
        // 绘制所有完成的笔画
        let strokes = self.strokes.lock().unwrap();
        for stroke in strokes.iter() {
            self.draw_stroke(painter, stroke);
        }

        // 绘制当前正在进行的笔画
        let current_stroke = self.current_stroke.lock().unwrap();
        if let Some(stroke) = current_stroke.as_ref() {
            self.draw_stroke(painter, stroke);
        }
    }

    fn draw_stroke(&self, painter: &slint::Painter, stroke: &Stroke) {
        if stroke.points.is_empty() {
            return;
        }

        let (r, g, b, a) = stroke.color;
        let brush_color = slint::Color::from_argb_u8(a, r, g, b);
        
        match stroke.tool {
            Tool::Pencil => self.draw_pencil_stroke(painter, stroke, brush_color),
            Tool::Line => self.draw_line(painter, stroke, brush_color),
            Tool::Rectangle => self.draw_rectangle(painter, stroke, brush_color),
            Tool::Ellipse => self.draw_ellipse(painter, stroke, brush_color),
            Tool::Eraser => self.draw_eraser_stroke(painter, stroke),
        }
    }

    fn draw_pencil_stroke(&self, painter: &slint::Painter, stroke: &Stroke, color: slint::Color) {
        if stroke.points.len() < 2 {
            // 单点情况，绘制一个圆点
            let point = &stroke.points[0];
            painter.draw_circle(
                slint::Point::new(point.x as f32, point.y as f32),
                stroke.size / 2.0,
                color,
            );
            return;
        }

        // 连续绘制线段
        for i in 1..stroke.points.len() {
            let p1 = &stroke.points[i - 1];
            let p2 = &stroke.points[i];
            
            painter.draw_line(
                slint::Point::new(p1.x as f32, p1.y as f32),
                slint::Point::new(p2.x as f32, p2.y as f32),
                color,
                stroke.size,
            );
        }
    }

    fn draw_line(&self, painter: &slint::Painter, stroke: &Stroke, color: slint::Color) {
        if stroke.points.len() < 2 {
            return;
        }

        let first = &stroke.points[0];
        let last = &stroke.points[stroke.points.len() - 1];
        
        painter.draw_line(
            slint::Point::new(first.x as f32, first.y as f32),
            slint::Point::new(last.x as f32, last.y as f32),
            color,
            stroke.size,
        );
    }

    fn draw_rectangle(&self, painter: &slint::Painter, stroke: &Stroke, color: slint::Color) {
        if stroke.points.len() < 2 {
            return;
        }

        let first = &stroke.points[0];
        let last = &stroke.points[stroke.points.len() - 1];
        
        let x1 = first.x.min(last.x);
        let y1 = first.y.min(last.y);
        let x2 = first.x.max(last.x);
        let y2 = first.y.max(last.y);
        
        painter.draw_rectangle(
            slint::LogicalRect::new(
                slint::LogicalPoint::new(x1, y1),
                slint::LogicalSize::new(x2 - x1, y2 - y1),
            ),
            color,
        );
    }

    fn draw_ellipse(&self, painter: &slint::Painter, stroke: &Stroke, color: slint::Color) {
        if stroke.points.len() < 2 {
            return;
        }

        let first = &stroke.points[0];
        let last = &stroke.points[stroke.points.len() - 1];
        
        let center_x = (first.x + last.x) / 2.0;
        let center_y = (first.y + last.y) / 2.0;
        let radius_x = (last.x - first.x).abs() / 2.0;
        let radius_y = (last.y - first.y).abs() / 2.0;
        
        // Slint 目前没有直接绘制椭圆的 API，所以这里使用近似方法
        // 在实际应用中，可以使用更精确的算法或平台特定实现
        let steps = 36; // 分段数
        let mut prev_x = center_x + radius_x * 0.0_f32.cos();
        let mut prev_y = center_y + radius_y * 0.0_f32.sin();
        
        for i in 1..=steps {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (steps as f32);
            let x = center_x + radius_x * angle.cos();
            let y = center_y + radius_y * angle.sin();
            
            painter.draw_line(
                slint::Point::new(prev_x, prev_y),
                slint::Point::new(x, y),
                color,
                stroke.size,
            );
            
            prev_x = x;
            prev_y = y;
        }
    }

    fn draw_eraser_stroke(&self, painter: &slint::Painter, stroke: &Stroke) {
        // 橡皮擦实际上是用白色覆盖
        let white = slint::Color::from_rgb_u8(255, 255, 255);
        self.draw_pencil_stroke(painter, stroke, white);
    }
}