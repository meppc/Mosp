use std::any::Any;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::canvas::{CanvasElement, Point, Rect, RenderContext};
use crate::error::{Result, Error};
use super::common::{Tool, ToolType, ToolContext, ToolEvent};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShapeType {
    Rectangle,
    Circle,
    Line,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    id: String,
    shape_type: ShapeType,
    start: Point,
    end: Point,
    color: (u8, u8, u8, u8), // RGBA
    fill_color: Option<(u8, u8, u8, u8)>, // RGBA for fill, None for stroke-only
    stroke_width: f32,
}

impl CanvasElement for Shape {
    fn render(&self, context: &mut RenderContext) {
    }
    
    fn contains(&self, point: Point) -> bool {
        match self.shape_type {
            ShapeType::Rectangle => {
                let rect = self.to_rect();
                rect.contains(point)
            },
            ShapeType::Circle => {
                let center = Point::new(
                    (self.start.x + self.end.x) / 2.0,
                    (self.start.y + self.end.y) / 2.0
                );
                let rx = (self.end.x - self.start.x).abs() / 2.0;
                let ry = (self.end.y - self.start.y).abs() / 2.0;
                let radius = rx.max(ry);
                
                center.distance_to(&point) <= radius
            },
            ShapeType::Line => {
                let line_len_sq = self.start.distance_to(&self.end).powi(2);
                if line_len_sq < 0.0001 {
                    return self.start.distance_to(&point) <= (self.stroke_width / 2.0) as f64;
                }
                
                let t = ((point.x - self.start.x) * (self.end.x - self.start.x) + 
                         (point.y - self.start.y) * (self.end.y - self.start.y)) / line_len_sq;
                let t_clamped = t.max(0.0).min(1.0);
                
                let projected = Point::new(
                    self.start.x + t_clamped * (self.end.x - self.start.x),
                    self.start.y + t_clamped * (self.end.y - self.start.y)
                );
                
                point.distance_to(&projected) <= (self.stroke_width / 2.0) as f64
            }
        }
    }
    
    fn bounds(&self) -> Rect {
        let rect = self.to_rect();
        
        let padding = (self.stroke_width / 2.0) as f64;
        Rect::new(
            rect.x - padding,
            rect.y - padding,
            rect.width + padding * 2.0,
            rect.height + padding * 2.0
        )
    }
    
    fn element_type(&self) -> &'static str {
        match self.shape_type {
            ShapeType::Rectangle => "rectangle",
            ShapeType::Circle => "circle",
            ShapeType::Line => "line",
        }
    }
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn clone_element(&self) -> Box<dyn CanvasElement> {
        Box::new(self.clone())
    }
}

impl Shape {
    fn to_rect(&self) -> Rect {
        let x1 = self.start.x.min(self.end.x);
        let y1 = self.start.y.min(self.end.y);
        let x2 = self.start.x.max(self.end.x);
        let y2 = self.start.y.max(self.end.y);
        
        Rect::new(x1, y1, x2 - x1, y2 - y1)
    }
}

#[derive(Debug)]
pub struct ShapeTool {
    shape_type: ShapeType,
    current_shape: Option<Shape>,
    color: (u8, u8, u8, u8),
    fill_color: Option<(u8, u8, u8, u8)>,
    stroke_width: f32,
}

impl ShapeTool {
    pub fn new(shape_type: ShapeType) -> Self {
        Self {
            shape_type,
            current_shape: None,
            color: (0, 0, 0, 255), // Black with full opacity
            fill_color: None,       // No fill by default
            stroke_width: 2.0,
        }
    }
    
    pub fn set_shape_type(&mut self, shape_type: ShapeType) {
        self.shape_type = shape_type;
    }
    
    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.color = (r, g, b, a);
    }
    
    pub fn set_fill_color(&mut self, fill: Option<(u8, u8, u8, u8)>) {
        self.fill_color = fill;
    }
    
    pub fn set_stroke_width(&mut self, width: f32) {
        self.stroke_width = width.max(0.5).min(100.0);
    }
}

impl Tool for ShapeTool {
    fn tool_type(&self) -> ToolType {
        ToolType::Shape
    }
    
    fn handle_event(&mut self, event: ToolEvent, context: ToolContext) -> Result<()> {
        match event {
            ToolEvent::Down { position, .. } => {
                let id = Uuid::new_v4().to_string();
                self.current_shape = Some(Shape {
                    id,
                    shape_type: self.shape_type,
                    start: position,
                    end: position,
                    color: self.color,
                    fill_color: self.fill_color,
                    stroke_width: self.stroke_width,
                });
                Ok(())
            },
            ToolEvent::Move { position, .. } => {
                if let Some(shape) = &mut self.current_shape {
                    shape.end = position;
                }
                Ok(())
            },
            ToolEvent::Up { position } => {
                if let Some(shape) = &mut self.current_shape {
                    shape.end = position;
                }
                Ok(())
            },
            ToolEvent::PropertyChanged { name, value } => {
                match name.as_str() {
                    "shape_type" => {
                        if let Some(shape_type) = value.downcast_ref::<ShapeType>() {
                            self.shape_type = *shape_type;
                        }
                    },
                    "color" => {
                        if let Some(color) = value.downcast_ref::<(u8, u8, u8, u8)>() {
                            self.color = *color;
                        }
                    },
                    "fill_color" => {
                        if let Some(fill_color) = value.downcast_ref::<Option<(u8, u8, u8, u8)>>() {
                            self.fill_color = *fill_color;
                        }
                    },
                    "stroke_width" => {
                        if let Some(width) = value.downcast_ref::<f32>() {
                            self.stroke_width = *width;
                        }
                    },
                    _ => return Err(Error::generic(format!("Unknown property: {}", name))),
                }
                Ok(())
            }
        }
    }
    
    fn create_element(&self) -> Option<Box<dyn CanvasElement>> {
        self.current_shape.as_ref().map(|s| Box::new(s.clone()) as Box<dyn CanvasElement>)
    }
    
    fn reset(&mut self) {
        self.current_shape = None;
    }
    
    fn config_to_json(&self) -> Result<String> {
        let config = serde_json::json!({
            "shape_type": self.shape_type,
            "color": self.color,
            "fill_color": self.fill_color,
            "stroke_width": self.stroke_width,
        });
        
        Ok(serde_json::to_string(&config)?)
    }
    
    fn config_from_json(&mut self, json: &str) -> Result<()> {
        let config: serde_json::Value = serde_json::from_str(json)?;
        
        if let Some(shape_type) = config.get("shape_type") {
            if let Some(st) = shape_type.as_u64() {
                match st {
                    0 => self.shape_type = ShapeType::Rectangle,
                    1 => self.shape_type = ShapeType::Circle,
                    2 => self.shape_type = ShapeType::Line,
                    _ => {}
                }
            }
        }
        
        if let Some(color) = config.get("color") {
            if let Some(color_array) = color.as_array() {
                if color_array.len() == 4 {
                    self.color = (
                        color_array[0].as_u64().unwrap_or(0) as u8,
                        color_array[1].as_u64().unwrap_or(0) as u8,
                        color_array[2].as_u64().unwrap_or(0) as u8,
                        color_array[3].as_u64().unwrap_or(255) as u8,
                    );
                }
            }
        }
        
        if let Some(fill_color) = config.get("fill_color") {
            if fill_color.is_null() {
                self.fill_color = None;
            } else if let Some(color_array) = fill_color.as_array() {
                if color_array.len() == 4 {
                    self.fill_color = Some((
                        color_array[0].as_u64().unwrap_or(0) as u8,
                        color_array[1].as_u64().unwrap_or(0) as u8,
                        color_array[2].as_u64().unwrap_or(0) as u8,
                        color_array[3].as_u64().unwrap_or(255) as u8,
                    ));
                }
            }
        }
        
        if let Some(width) = config.get("stroke_width") {
            if let Some(w) = width.as_f64() {
                self.stroke_width = w as f32;
            }
        }
        
        Ok(())
    }
}
