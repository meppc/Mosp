use std::any::Any;
use crate::canvas::{Canvas, Point, Transform, CanvasElement};
use crate::error::{Result, Error};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolType {
    Brush,
    Shape,
    Arrow,
    Eraser,
    Text,
    Selection,
}

pub struct ToolContext<'a> {
    pub canvas: &'a Canvas,
    pub position: Point,
    pub pressure: f32,
    pub transform: Transform,
}

#[derive(Debug)]
pub enum ToolEvent {
    Down { position: Point, pressure: f32 },
    Move { position: Point, pressure: f32 },
    Up { position: Point },
    PropertyChanged { name: String, value: Box<dyn Any + Send + Sync> },
}

impl Clone for ToolEvent {
    fn clone(&self) -> Self {
        match self {
            Self::Down { position, pressure } => Self::Down { 
                position: position.clone(), 
                pressure: *pressure 
            },
            Self::Move { position, pressure } => Self::Move { 
                position: position.clone(), 
                pressure: *pressure 
            },
            Self::Up { position } => Self::Up { 
                position: position.clone() 
            },
            Self::PropertyChanged { name, .. } => Self::PropertyChanged { 
                name: name.clone(), 
                value: Box::new(()) // We can't clone the Any value, so we replace it with unit
            },
        }
    }
}

pub trait Tool: Send + Sync {
    fn tool_type(&self) -> ToolType;
    
    fn handle_event(&mut self, event: ToolEvent, context: ToolContext) -> Result<()>;
    
    fn create_element(&self) -> Option<Box<dyn CanvasElement>>;
    
    fn reset(&mut self);
    
    fn config_to_json(&self) -> Result<String>;
    
    fn config_from_json(&mut self, json: &str) -> Result<()>;
}
