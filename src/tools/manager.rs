use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::canvas::{Canvas, Point, Transform};
use crate::error::{Result, Error};
use super::common::{Tool, ToolType, ToolContext, ToolEvent};
use super::{BrushTool, ShapeTool, ShapeType, ArrowTool, EraserTool, TextTool, SelectionTool};

pub struct ToolManager {
    tools: HashMap<ToolType, Box<dyn Tool>>,
    active_tool_type: ToolType,
    canvas: Arc<Canvas>,
}

impl ToolManager {
    pub fn new(canvas: Arc<Canvas>) -> Self {
        let mut tools = HashMap::new();
        
        tools.insert(ToolType::Brush, Box::new(BrushTool::new()) as Box<dyn Tool>);
        tools.insert(ToolType::Shape, Box::new(ShapeTool::new(ShapeType::Rectangle)) as Box<dyn Tool>);
        tools.insert(ToolType::Arrow, Box::new(ArrowTool::new()) as Box<dyn Tool>);
        tools.insert(ToolType::Eraser, Box::new(EraserTool::new()) as Box<dyn Tool>);
        tools.insert(ToolType::Text, Box::new(TextTool::new()) as Box<dyn Tool>);
        tools.insert(ToolType::Selection, Box::new(SelectionTool::new()) as Box<dyn Tool>);
        
        Self {
            tools,
            active_tool_type: ToolType::Brush, // Default tool
            canvas,
        }
    }
    
    pub fn set_active_tool(&mut self, tool_type: ToolType) {
        if let Some(tool) = self.tools.get_mut(&self.active_tool_type) {
            tool.reset();
        }
        
        self.active_tool_type = tool_type;
    }
    
    pub fn get_active_tool(&mut self) -> &mut dyn Tool {
        self.tools.get_mut(&self.active_tool_type)
            .expect("Active tool not found")
            .as_mut()
    }
    
    pub fn handle_event(&mut self, event: ToolEvent, position: Point, pressure: f32, transform: Transform) -> Result<()> {
        let context = ToolContext {
            canvas: &self.canvas,
            position,
            pressure,
            transform,
        };
        
        let tool = self.get_active_tool();
        tool.handle_event(event.clone(), context)?;
        
        if let ToolEvent::Up { .. } = event {
            if let Some(element) = tool.create_element() {
                self.canvas.add_element(element)?;
                tool.reset();
            }
        }
        
        Ok(())
    }
    
    pub fn get_tool_config(&mut self, tool_type: ToolType) -> Result<String> {
        if let Some(tool) = self.tools.get_mut(&tool_type) {
            tool.config_to_json()
        } else {
            Err(Error::generic(format!("Tool type {:?} not found", tool_type)))
        }
    }
    
    pub fn set_tool_config(&mut self, tool_type: ToolType, config: &str) -> Result<()> {
        if let Some(tool) = self.tools.get_mut(&tool_type) {
            tool.config_from_json(config)
        } else {
            Err(Error::generic(format!("Tool type {:?} not found", tool_type)))
        }
    }
}
