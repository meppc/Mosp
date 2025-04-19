use std::any::Any;
use crate::canvas::{Canvas, Point, CanvasElement};
use crate::error::{Result, Error};
use super::common::{Tool, ToolType, ToolContext, ToolEvent};

#[derive(Debug)]
pub struct EraserTool {
    size: f32,
    is_erasing: bool,
    last_position: Option<Point>,
}

impl EraserTool {
    pub fn new() -> Self {
        Self {
            size: 20.0,
            is_erasing: false,
            last_position: None,
        }
    }
    
    pub fn set_size(&mut self, size: f32) {
        self.size = size.max(5.0).min(100.0);
    }
    
    fn erase_at(&self, canvas: &Canvas, position: Point) -> Result<()> {
        
        
        Ok(())
    }
}

impl Tool for EraserTool {
    fn tool_type(&self) -> ToolType {
        ToolType::Eraser
    }
    
    fn handle_event(&mut self, event: ToolEvent, context: ToolContext) -> Result<()> {
        match event {
            ToolEvent::Down { position, .. } => {
                self.is_erasing = true;
                self.last_position = Some(position);
                self.erase_at(context.canvas, position)?;
                Ok(())
            },
            ToolEvent::Move { position, .. } => {
                if self.is_erasing {
                    if let Some(last_pos) = self.last_position {
                        let distance = last_pos.distance_to(&position);
                        let steps = (distance / (self.size as f64 / 4.0)).ceil() as usize;
                        
                        if steps > 1 {
                            for i in 1..steps {
                                let t = i as f64 / steps as f64;
                                let x = last_pos.x + (position.x - last_pos.x) * t;
                                let y = last_pos.y + (position.y - last_pos.y) * t;
                                let interpolated = Point::new(x, y);
                                
                                self.erase_at(context.canvas, interpolated)?;
                            }
                        }
                    }
                    
                    self.erase_at(context.canvas, position)?;
                    self.last_position = Some(position);
                }
                Ok(())
            },
            ToolEvent::Up { .. } => {
                self.is_erasing = false;
                self.last_position = None;
                Ok(())
            },
            ToolEvent::PropertyChanged { name, value } => {
                match name.as_str() {
                    "size" => {
                        if let Some(size) = value.downcast_ref::<f32>() {
                            self.size = *size;
                        }
                    },
                    _ => return Err(Error::generic(format!("Unknown property: {}", name))),
                }
                Ok(())
            }
        }
    }
    
    fn create_element(&self) -> Option<Box<dyn CanvasElement>> {
        None
    }
    
    fn reset(&mut self) {
        self.is_erasing = false;
        self.last_position = None;
    }
    
    fn config_to_json(&self) -> Result<String> {
        let config = serde_json::json!({
            "size": self.size,
        });
        
        Ok(serde_json::to_string(&config)?)
    }
    
    fn config_from_json(&mut self, json: &str) -> Result<()> {
        let config: serde_json::Value = serde_json::from_str(json)?;
        
        if let Some(size) = config.get("size") {
            if let Some(s) = size.as_f64() {
                self.size = s as f32;
            }
        }
        
        Ok(())
    }
}
