use std::any::Any;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::canvas::{CanvasElement, Point, Rect, RenderContext};
use crate::error::{Result, Error};
use super::common::{Tool, ToolType, ToolContext, ToolEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrushStroke {
    id: String,
    points: Vec<(Point, f32)>, // Points with pressure
    color: (u8, u8, u8, u8),   // RGBA
    width: f32,
}

impl CanvasElement for BrushStroke {
    fn render(&self, context: &mut RenderContext) {
    }
    
    fn contains(&self, point: Point) -> bool {
        for i in 1..self.points.len() {
            let p1 = self.points[i-1].0;
            let p2 = self.points[i].0;
            
            let line_len_sq = p1.distance_to(&p2).powi(2);
            if line_len_sq < 0.0001 {
                if point.distance_to(&p1) <= (self.width / 2.0) as f64 {
                    return true;
                }
                continue;
            }
            
            let t = ((point.x - p1.x) * (p2.x - p1.x) + 
                     (point.y - p1.y) * (p2.y - p1.y)) / line_len_sq;
            let t_clamped = t.max(0.0).min(1.0);
            
            let projected = Point::new(
                p1.x + t_clamped * (p2.x - p1.x),
                p1.y + t_clamped * (p2.y - p1.y)
            );
            
            if point.distance_to(&projected) <= (self.width / 2.0) as f64 {
                return true;
            }
        }
        
        false
    }
    
    fn bounds(&self) -> Rect {
        if self.points.is_empty() {
            return Rect::new(0.0, 0.0, 0.0, 0.0);
        }
        
        let mut min_x = self.points[0].0.x;
        let mut min_y = self.points[0].0.y;
        let mut max_x = min_x;
        let mut max_y = min_y;
        
        for (point, _) in &self.points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }
        
        let padding = (self.width / 2.0) as f64;
        Rect::new(
            min_x - padding,
            min_y - padding,
            max_x - min_x + padding * 2.0,
            max_y - min_y + padding * 2.0
        )
    }
    
    fn element_type(&self) -> &'static str {
        "brush_stroke"
    }
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn clone_element(&self) -> Box<dyn CanvasElement> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct BrushTool {
    current_stroke: Option<BrushStroke>,
    color: (u8, u8, u8, u8),
    width: f32,
    pressure_sensitivity: bool,
}

impl BrushTool {
    pub fn new() -> Self {
        Self {
            current_stroke: None,
            color: (0, 0, 0, 255), // Black with full opacity
            width: 2.0,
            pressure_sensitivity: true,
        }
    }
    
    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.color = (r, g, b, a);
    }
    
    pub fn set_width(&mut self, width: f32) {
        self.width = width.max(0.5).min(100.0);
    }
    
    pub fn set_pressure_sensitivity(&mut self, enabled: bool) {
        self.pressure_sensitivity = enabled;
    }
}

impl Tool for BrushTool {
    fn tool_type(&self) -> ToolType {
        ToolType::Brush
    }
    
    fn handle_event(&mut self, event: ToolEvent, context: ToolContext) -> Result<()> {
        match event {
            ToolEvent::Down { position, pressure } => {
                let effective_pressure = if self.pressure_sensitivity { pressure } else { 1.0 };
                let id = Uuid::new_v4().to_string();
                self.current_stroke = Some(BrushStroke {
                    id,
                    points: vec![(position, effective_pressure)],
                    color: self.color,
                    width: self.width,
                });
                Ok(())
            },
            ToolEvent::Move { position, pressure } => {
                if let Some(stroke) = &mut self.current_stroke {
                    let effective_pressure = if self.pressure_sensitivity { pressure } else { 1.0 };
                    stroke.points.push((position, effective_pressure));
                }
                Ok(())
            },
            ToolEvent::Up { position } => {
                if let Some(stroke) = &mut self.current_stroke {
                    if stroke.points.last().map(|p| p.0) != Some(position) {
                        stroke.points.push((position, 1.0));
                    }
                    
                    if stroke.points.len() == 1 {
                        let p = stroke.points[0].0;
                        stroke.points.push((Point::new(p.x + 0.1, p.y), 1.0));
                    }
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
                    "width" => {
                        if let Some(width) = value.downcast_ref::<f32>() {
                            self.width = *width;
                        }
                    },
                    "pressure_sensitivity" => {
                        if let Some(enabled) = value.downcast_ref::<bool>() {
                            self.pressure_sensitivity = *enabled;
                        }
                    },
                    _ => return Err(Error::generic(format!("Unknown property: {}", name))),
                }
                Ok(())
            }
        }
    }
    
    fn create_element(&self) -> Option<Box<dyn CanvasElement>> {
        self.current_stroke.as_ref().map(|s| Box::new(s.clone()) as Box<dyn CanvasElement>)
    }
    
    fn reset(&mut self) {
        self.current_stroke = None;
    }
    
    fn config_to_json(&self) -> Result<String> {
        let config = serde_json::json!({
            "color": self.color,
            "width": self.width,
            "pressure_sensitivity": self.pressure_sensitivity,
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
        
        if let Some(width) = config.get("width") {
            if let Some(w) = width.as_f64() {
                self.width = w as f32;
            }
        }
        
        if let Some(pressure) = config.get("pressure_sensitivity") {
            if let Some(p) = pressure.as_bool() {
                self.pressure_sensitivity = p;
            }
        }
        
        Ok(())
    }
}
