use std::any::Any;
use crate::canvas::{Canvas, Point, Rect, CanvasElement};
use crate::error::{Result, Error};
use super::common::{Tool, ToolType, ToolContext, ToolEvent};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    Rectangle,
    Lasso,
}

#[derive(Debug)]
pub struct SelectionTool {
    mode: SelectionMode,
    start_point: Option<Point>,
    current_point: Option<Point>,
    lasso_points: Vec<Point>,
    selected_elements: Vec<String>, // IDs of selected elements
}

impl SelectionTool {
    pub fn new() -> Self {
        Self {
            mode: SelectionMode::Rectangle,
            start_point: None,
            current_point: None,
            lasso_points: Vec::new(),
            selected_elements: Vec::new(),
        }
    }
    
    pub fn set_mode(&mut self, mode: SelectionMode) {
        self.mode = mode;
    }
    
    fn update_selection(&mut self, canvas: &Canvas) -> Result<()> {
        
        
        Ok(())
    }
}

impl Tool for SelectionTool {
    fn tool_type(&self) -> ToolType {
        ToolType::Selection
    }
    
    fn handle_event(&mut self, event: ToolEvent, context: ToolContext) -> Result<()> {
        match event {
            ToolEvent::Down { position, .. } => {
                self.start_point = Some(position);
                self.current_point = Some(position);
                
                if self.mode == SelectionMode::Lasso {
                    self.lasso_points.clear();
                    self.lasso_points.push(position);
                }
                
                self.selected_elements.clear();
                
                Ok(())
            },
            ToolEvent::Move { position, .. } => {
                self.current_point = Some(position);
                
                if self.mode == SelectionMode::Lasso {
                    self.lasso_points.push(position);
                }
                
                Ok(())
            },
            ToolEvent::Up { position } => {
                self.current_point = Some(position);
                
                if self.mode == SelectionMode::Lasso {
                    self.lasso_points.push(position);
                    if let Some(first) = self.lasso_points.first() {
                        self.lasso_points.push(*first);
                    }
                }
                
                self.update_selection(context.canvas)?;
                
                Ok(())
            },
            ToolEvent::PropertyChanged { name, value } => {
                match name.as_str() {
                    "mode" => {
                        if let Some(mode) = value.downcast_ref::<SelectionMode>() {
                            self.mode = *mode;
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
        self.start_point = None;
        self.current_point = None;
        self.lasso_points.clear();
        self.selected_elements.clear();
    }
    
    fn config_to_json(&self) -> Result<String> {
        let config = serde_json::json!({
            "mode": match self.mode {
                SelectionMode::Rectangle => "rectangle",
                SelectionMode::Lasso => "lasso",
            },
        });
        
        Ok(serde_json::to_string(&config)?)
    }
    
    fn config_from_json(&mut self, json: &str) -> Result<()> {
        let config: serde_json::Value = serde_json::from_str(json)?;
        
        if let Some(mode) = config.get("mode") {
            if let Some(mode_str) = mode.as_str() {
                self.mode = match mode_str {
                    "rectangle" => SelectionMode::Rectangle,
                    "lasso" => SelectionMode::Lasso,
                    _ => SelectionMode::Rectangle,
                };
            }
        }
        
        Ok(())
    }
}
