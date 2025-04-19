use std::collections::HashMap;
use std::sync::Arc;
use crate::canvas::{Canvas, Point, Transform};
use crate::error::{Result, Error};
use crate::tools::{Tool, ToolType, ToolContext, ToolEvent};
use crate::tools::brush::BrushTool;
use crate::tools::shape::{ShapeTool, ShapeType};
use crate::tools::arrow::ArrowTool;
use crate::tools::eraser::EraserTool;
use crate::tools::text::TextTool;
use crate::tools::selection::SelectionTool;

pub struct ToolManager {
    tools: HashMap<ToolType, Box<dyn Tool>>,
    active_tool_type: ToolType,
}

impl ToolManager {
    pub fn new() -> Self {
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
        }
    }
    
    pub fn set_active_tool(&mut self, tool_type: ToolType) {
        if let Some(tool) = self.tools.get_mut(&self.active_tool_type) {
            tool.reset();
        }
        
        self.active_tool_type = tool_type;
        
        if !self.tools.contains_key(&tool_type) {
            let new_tool: Box<dyn Tool> = match tool_type {
                ToolType::Brush => Box::new(BrushTool::new()),
                ToolType::Shape => Box::new(ShapeTool::new(ShapeType::Rectangle)),
                ToolType::Arrow => Box::new(ArrowTool::new()),
                ToolType::Eraser => Box::new(EraserTool::new()),
                ToolType::Text => Box::new(TextTool::new()),
                ToolType::Selection => Box::new(SelectionTool::new()),
            };
            self.tools.insert(tool_type, new_tool);
        }
    }
    
    pub fn get_active_tool(&mut self) -> &mut dyn Tool {
        self.tools.get_mut(&self.active_tool_type)
            .expect("Active tool not found")
            .as_mut()
    }
    
    pub fn handle_event(&mut self, event: ToolEvent, context: ToolContext) -> Result<()> {
        let tool = self.get_active_tool();
        tool.handle_event(event.clone(), context)?;
        
        if let ToolEvent::Up { .. } = event {
            if let Some(element) = tool.create_element() {
                // self.canvas.add_element(element)?;
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
