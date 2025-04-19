use std::any::Any;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::canvas::{CanvasElement, Point, Rect, RenderContext};
use crate::error::{Result, Error};
use super::common::{Tool, ToolType, ToolContext, ToolEvent};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    font_family: String,
    font_size: f32,
    bold: bool,
    italic: bool,
    underline: bool,
    color: (u8, u8, u8, u8),
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_family: "Arial".to_string(),
            font_size: 16.0,
            bold: false,
            italic: false,
            underline: false,
            color: (0, 0, 0, 255),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    id: String,
    position: Point,
    width: f32,
    text: String,
    style: TextStyle,
    alignment: TextAlignment,
}

impl CanvasElement for TextElement {
    fn render(&self, context: &mut RenderContext) {
    }
    
    fn contains(&self, point: Point) -> bool {
        let bounds = self.bounds();
        bounds.contains(point)
    }
    
    fn bounds(&self) -> Rect {
        
        let height = self.style.font_size * 1.2; // Approximate line height
        
        Rect::new(
            self.position.x,
            self.position.y,
            self.width as f64,
            height as f64
        )
    }
    
    fn element_type(&self) -> &'static str {
        "text"
    }
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn clone_element(&self) -> Box<dyn CanvasElement> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct TextTool {
    current_text: Option<TextElement>,
    style: TextStyle,
    alignment: TextAlignment,
    default_width: f32,
}

impl TextTool {
    pub fn new() -> Self {
        Self {
            current_text: None,
            style: TextStyle::default(),
            alignment: TextAlignment::Left,
            default_width: 200.0,
        }
    }
    
    pub fn set_font_family(&mut self, font_family: String) {
        self.style.font_family = font_family;
    }
    
    pub fn set_font_size(&mut self, size: f32) {
        self.style.font_size = size.max(8.0).min(72.0);
    }
    
    pub fn set_bold(&mut self, bold: bool) {
        self.style.bold = bold;
    }
    
    pub fn set_italic(&mut self, italic: bool) {
        self.style.italic = italic;
    }
    
    pub fn set_underline(&mut self, underline: bool) {
        self.style.underline = underline;
    }
    
    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.style.color = (r, g, b, a);
    }
    
    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
    }
    
    pub fn set_default_width(&mut self, width: f32) {
        self.default_width = width.max(50.0).min(1000.0);
    }
    
    pub fn set_text(&mut self, text: String) {
        if let Some(text_element) = &mut self.current_text {
            text_element.text = text;
        }
    }
}

impl Tool for TextTool {
    fn tool_type(&self) -> ToolType {
        ToolType::Text
    }
    
    fn handle_event(&mut self, event: ToolEvent, context: ToolContext) -> Result<()> {
        match event {
            ToolEvent::Down { position, .. } => {
                let id = Uuid::new_v4().to_string();
                self.current_text = Some(TextElement {
                    id,
                    position,
                    width: self.default_width,
                    text: "Text".to_string(), // Default text
                    style: self.style.clone(),
                    alignment: self.alignment,
                });
                Ok(())
            },
            ToolEvent::Move { .. } => {
                Ok(())
            },
            ToolEvent::Up { .. } => {
                Ok(())
            },
            ToolEvent::PropertyChanged { name, value } => {
                match name.as_str() {
                    "font_family" => {
                        if let Some(font) = value.downcast_ref::<String>() {
                            self.set_font_family(font.clone());
                        }
                    },
                    "font_size" => {
                        if let Some(size) = value.downcast_ref::<f32>() {
                            self.set_font_size(*size);
                        }
                    },
                    "bold" => {
                        if let Some(bold) = value.downcast_ref::<bool>() {
                            self.set_bold(*bold);
                        }
                    },
                    "italic" => {
                        if let Some(italic) = value.downcast_ref::<bool>() {
                            self.set_italic(*italic);
                        }
                    },
                    "underline" => {
                        if let Some(underline) = value.downcast_ref::<bool>() {
                            self.set_underline(*underline);
                        }
                    },
                    "color" => {
                        if let Some(color) = value.downcast_ref::<(u8, u8, u8, u8)>() {
                            self.set_color(color.0, color.1, color.2, color.3);
                        }
                    },
                    "alignment" => {
                        if let Some(alignment) = value.downcast_ref::<TextAlignment>() {
                            self.set_alignment(*alignment);
                        }
                    },
                    "width" => {
                        if let Some(width) = value.downcast_ref::<f32>() {
                            self.set_default_width(*width);
                        }
                    },
                    "text" => {
                        if let Some(text) = value.downcast_ref::<String>() {
                            self.set_text(text.clone());
                        }
                    },
                    _ => return Err(Error::generic(format!("Unknown property: {}", name))),
                }
                Ok(())
            }
        }
    }
    
    fn create_element(&self) -> Option<Box<dyn CanvasElement>> {
        self.current_text.as_ref().map(|t| Box::new(t.clone()) as Box<dyn CanvasElement>)
    }
    
    fn reset(&mut self) {
        self.current_text = None;
    }
    
    fn config_to_json(&self) -> Result<String> {
        let config = serde_json::json!({
            "font_family": self.style.font_family,
            "font_size": self.style.font_size,
            "bold": self.style.bold,
            "italic": self.style.italic,
            "underline": self.style.underline,
            "color": self.style.color,
            "alignment": self.alignment,
            "default_width": self.default_width,
        });
        
        Ok(serde_json::to_string(&config)?)
    }
    
    fn config_from_json(&mut self, json: &str) -> Result<()> {
        let config: serde_json::Value = serde_json::from_str(json)?;
        
        if let Some(font_family) = config.get("font_family") {
            if let Some(font) = font_family.as_str() {
                self.style.font_family = font.to_string();
            }
        }
        
        if let Some(font_size) = config.get("font_size") {
            if let Some(size) = font_size.as_f64() {
                self.style.font_size = size as f32;
            }
        }
        
        if let Some(bold) = config.get("bold") {
            if let Some(b) = bold.as_bool() {
                self.style.bold = b;
            }
        }
        
        if let Some(italic) = config.get("italic") {
            if let Some(i) = italic.as_bool() {
                self.style.italic = i;
            }
        }
        
        if let Some(underline) = config.get("underline") {
            if let Some(u) = underline.as_bool() {
                self.style.underline = u;
            }
        }
        
        if let Some(color) = config.get("color") {
            if let Some(color_array) = color.as_array() {
                if color_array.len() == 4 {
                    self.style.color = (
                        color_array[0].as_u64().unwrap_or(0) as u8,
                        color_array[1].as_u64().unwrap_or(0) as u8,
                        color_array[2].as_u64().unwrap_or(0) as u8,
                        color_array[3].as_u64().unwrap_or(255) as u8,
                    );
                }
            }
        }
        
        if let Some(alignment) = config.get("alignment") {
            if let Some(a) = alignment.as_u64() {
                match a {
                    0 => self.alignment = TextAlignment::Left,
                    1 => self.alignment = TextAlignment::Center,
                    2 => self.alignment = TextAlignment::Right,
                    _ => {}
                }
            }
        }
        
        if let Some(width) = config.get("default_width") {
            if let Some(w) = width.as_f64() {
                self.default_width = w as f32;
            }
        }
        
        Ok(())
    }
}
