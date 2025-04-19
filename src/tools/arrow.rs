use std::any::Any;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::canvas::{CanvasElement, Point, Rect, RenderContext};
use crate::error::{Result, Error};
use super::common::{Tool, ToolType, ToolContext, ToolEvent};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ArrowStyle {
    Simple,
    Filled,
    Double,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arrow {
    id: String,
    start: Point,
    end: Point,
    color: (u8, u8, u8, u8), // RGBA
    stroke_width: f32,
    arrow_style: ArrowStyle,
    head_size: f32,
}

impl CanvasElement for Arrow {
    fn render(&self, context: &mut RenderContext) {
    }
    
    fn contains(&self, point: Point) -> bool {
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
        
        if point.distance_to(&projected) <= (self.stroke_width / 2.0) as f64 {
            return true;
        }
        
        if self.end.distance_to(&point) <= self.head_size as f64 {
            return true;
        }
        
        false
    }
    
    fn bounds(&self) -> Rect {
        let x1 = self.start.x.min(self.end.x);
        let y1 = self.start.y.min(self.end.y);
        let x2 = self.start.x.max(self.end.x);
        let y2 = self.start.y.max(self.end.y);
        
        let padding = self.head_size.max(self.stroke_width) as f64;
        
        Rect::new(
            x1 - padding,
            y1 - padding,
            x2 - x1 + padding * 2.0,
            y2 - y1 + padding * 2.0
        )
    }
    
    fn element_type(&self) -> &'static str {
        "arrow"
    }
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn clone_element(&self) -> Box<dyn CanvasElement> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct ArrowTool {
    current_arrow: Option<Arrow>,
    color: (u8, u8, u8, u8),
    stroke_width: f32,
    arrow_style: ArrowStyle,
    head_size: f32,
}

impl ArrowTool {
    pub fn new() -> Self {
        Self {
            current_arrow: None,
            color: (0, 0, 0, 255), // Black with full opacity
            stroke_width: 2.0,
            arrow_style: ArrowStyle::Simple,
            head_size: 10.0,
        }
    }
    
    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.color = (r, g, b, a);
    }
    
    pub fn set_stroke_width(&mut self, width: f32) {
        self.stroke_width = width.max(0.5).min(100.0);
    }
    
    pub fn set_arrow_style(&mut self, style: ArrowStyle) {
        self.arrow_style = style;
    }
    
    pub fn set_head_size(&mut self, size: f32) {
        self.head_size = size.max(5.0).min(50.0);
    }
}

impl Tool for ArrowTool {
    fn tool_type(&self) -> ToolType {
        ToolType::Arrow
    }
    
    fn handle_event(&mut self, event: ToolEvent, context: ToolContext) -> Result<()> {
        match event {
            ToolEvent::Down { position, .. } => {
                let id = Uuid::new_v4().to_string();
                self.current_arrow = Some(Arrow {
                    id,
                    start: position,
                    end: position,
                    color: self.color,
                    stroke_width: self.stroke_width,
                    arrow_style: self.arrow_style,
                    head_size: self.head_size,
                });
                Ok(())
            },
            ToolEvent::Move { position, .. } => {
                if let Some(arrow) = &mut self.current_arrow {
                    arrow.end = position;
                }
                Ok(())
            },
            ToolEvent::Up { position } => {
                if let Some(arrow) = &mut self.current_arrow {
                    arrow.end = position;
                }
                Ok(())
            },
            ToolEvent::PropertyChanged { name, value } => {
                match name.as_str() {
                    "color" => {
                        if let Some(color) = value.downcast_ref::<(u8, u8, u8, u8)>() {
                            self.color = *color;
                        }
                    },
                    "stroke_width" => {
                        if let Some(width) = value.downcast_ref::<f32>() {
                            self.stroke_width = *width;
                        }
                    },
                    "arrow_style" => {
                        if let Some(style) = value.downcast_ref::<ArrowStyle>() {
                            self.arrow_style = *style;
                        }
                    },
                    "head_size" => {
                        if let Some(size) = value.downcast_ref::<f32>() {
                            self.head_size = *size;
                        }
                    },
                    _ => return Err(Error::generic(format!("Unknown property: {}", name))),
                }
                Ok(())
            }
        }
    }
    
    fn create_element(&self) -> Option<Box<dyn CanvasElement>> {
        self.current_arrow.as_ref().map(|a| Box::new(a.clone()) as Box<dyn CanvasElement>)
    }
    
    fn reset(&mut self) {
        self.current_arrow = None;
    }
    
    fn config_to_json(&self) -> Result<String> {
        let config = serde_json::json!({
            "color": self.color,
            "stroke_width": self.stroke_width,
            "arrow_style": self.arrow_style,
            "head_size": self.head_size,
        });
        
        Ok(serde_json::to_string(&config)?)
    }
    
    fn config_from_json(&mut self, json: &str) -> Result<()> {
        let config: serde_json::Value = serde_json::from_str(json)?;
        
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
        
        if let Some(width) = config.get("stroke_width") {
            if let Some(w) = width.as_f64() {
                self.stroke_width = w as f32;
            }
        }
        
        if let Some(style) = config.get("arrow_style") {
            if let Some(s) = style.as_u64() {
                match s {
                    0 => self.arrow_style = ArrowStyle::Simple,
                    1 => self.arrow_style = ArrowStyle::Filled,
                    2 => self.arrow_style = ArrowStyle::Double,
                    _ => {}
                }
            }
        }
        
        if let Some(size) = config.get("head_size") {
            if let Some(s) = size.as_f64() {
                self.head_size = s as f32;
            }
        }
        
        Ok(())
    }
}
