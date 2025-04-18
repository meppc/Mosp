
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::event::{global_event_bus, events::CanvasViewChangeEvent};

pub trait CanvasElement: Send + Sync {
    fn render(&self, context: &mut RenderContext);
    
    fn contains(&self, point: Point) -> bool;
    
    fn bounds(&self) -> Rect;
    
    fn element_type(&self) -> &'static str;
}

pub struct RenderContext {
    pub transform: Transform,
    pub clip_rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }
    
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub tx: f64,
    pub ty: f64,
    pub scale: f64,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            tx: 0.0,
            ty: 0.0,
            scale: 1.0,
        }
    }
    
    pub fn new(tx: f64, ty: f64, scale: f64) -> Self {
        Self { tx, ty, scale }
    }
    
    pub fn apply(&self, point: Point) -> Point {
        Point {
            x: point.x * self.scale + self.tx,
            y: point.y * self.scale + self.ty,
        }
    }
    
    pub fn inverse(&self, point: Point) -> Point {
        Point {
            x: (point.x - self.tx) / self.scale,
            y: (point.y - self.ty) / self.scale,
        }
    }
    
    pub fn combine(&self, other: &Transform) -> Transform {
        Transform {
            tx: self.tx + other.tx * self.scale,
            ty: self.ty + other.ty * self.scale,
            scale: self.scale * other.scale,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    #[serde(skip)]
    elements: Vec<Arc<dyn CanvasElement>>,
}

impl Layer {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            visible: true,
            locked: false,
            opacity: 1.0,
            elements: Vec::new(),
        }
    }
    
    pub fn add_element(&mut self, element: Arc<dyn CanvasElement>) {
        self.elements.push(element);
    }
    
    pub fn remove_element(&mut self, index: usize) -> Option<Arc<dyn CanvasElement>> {
        if index < self.elements.len() {
            Some(self.elements.remove(index))
        } else {
            None
        }
    }
    
    pub fn render(&self, context: &mut RenderContext) {
        if !self.visible {
            return;
        }
        
        for element in &self.elements {
            element.render(context);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    pub center: Point,
    pub zoom: f64,
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            center: Point::new(0.0, 0.0),
            zoom: 1.0,
            width,
            height,
        }
    }
    
    pub fn get_transform(&self) -> Transform {
        let tx = self.width as f64 / 2.0 - self.center.x * self.zoom;
        let ty = self.height as f64 / 2.0 - self.center.y * self.zoom;
        
        Transform::new(tx, ty, self.zoom)
    }
    
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.center.x -= dx / self.zoom;
        self.center.y -= dy / self.zoom;
    }
    
    pub fn zoom(&mut self, factor: f64, focus_point: Option<Point>) {
        let old_zoom = self.zoom;
        self.zoom *= factor;
        
        self.zoom = self.zoom.max(0.1).min(10.0);
        
        if let Some(focus) = focus_point {
            let zoom_ratio = self.zoom / old_zoom;
            let dx = focus.x - self.center.x;
            let dy = focus.y - self.center.y;
            
            self.center.x += dx * (1.0 - 1.0 / zoom_ratio);
            self.center.y += dy * (1.0 - 1.0 / zoom_ratio);
        }
    }
    
    pub fn visible_area(&self) -> Rect {
        let half_width = self.width as f64 / (2.0 * self.zoom);
        let half_height = self.height as f64 / (2.0 * self.zoom);
        
        Rect::new(
            self.center.x - half_width,
            self.center.y - half_height,
            half_width * 2.0,
            half_height * 2.0,
        )
    }
}

pub struct Canvas {
    pub id: String,
    pub name: String,
    viewport: RwLock<Viewport>,
    layers: RwLock<Vec<Layer>>,
    layer_map: RwLock<HashMap<String, usize>>,
}

impl Canvas {
    pub fn new(id: String, name: String, width: u32, height: u32) -> Self {
        let viewport = Viewport::new(width, height);
        
        Self {
            id,
            name,
            viewport: RwLock::new(viewport),
            layers: RwLock::new(Vec::new()),
            layer_map: RwLock::new(HashMap::new()),
        }
    }
    
    pub fn add_layer(&self, layer: Layer) -> Result<()> {
        let mut layers = self.layers.write().unwrap();
        let mut layer_map = self.layer_map.write().unwrap();
        
        if layer_map.contains_key(&layer.id) {
            return Err(crate::error::Error::Generic(
                format!("Layer with ID {} already exists", layer.id)
            ));
        }
        
        let index = layers.len();
        layers.push(layer.clone());
        layer_map.insert(layer.id.clone(), index);
        
        Ok(())
    }
    
    pub fn get_layer(&self, id: &str) -> Option<Layer> {
        let layer_map = self.layer_map.read().unwrap();
        let layers = self.layers.read().unwrap();
        
        layer_map.get(id).map(|&index| layers[index].clone())
    }
    
    pub fn remove_layer(&self, id: &str) -> Result<()> {
        let mut layer_map = self.layer_map.write().unwrap();
        let mut layers = self.layers.write().unwrap();
        
        if let Some(&index) = layer_map.get(id) {
            layers.remove(index);
            layer_map.remove(id);
            
            *layer_map = layers
                .iter()
                .enumerate()
                .map(|(i, layer)| (layer.id.clone(), i))
                .collect();
            
            Ok(())
        } else {
            Err(crate::error::Error::Generic(
                format!("Layer with ID {} not found", id)
            ))
        }
    }
    
    pub fn pan(&self, dx: f64, dy: f64) {
        let mut viewport = self.viewport.write().unwrap();
        viewport.pan(dx, dy);
        
        global_event_bus().dispatch(CanvasViewChangeEvent {
            x: viewport.center.x,
            y: viewport.center.y,
            scale: viewport.zoom,
        });
    }
    
    pub fn zoom(&self, factor: f64, focus_point: Option<Point>) {
        let mut viewport = self.viewport.write().unwrap();
        viewport.zoom(factor, focus_point);
        
        global_event_bus().dispatch(CanvasViewChangeEvent {
            x: viewport.center.x,
            y: viewport.center.y,
            scale: viewport.zoom,
        });
    }
    
    pub fn render(&self) {
        let viewport = self.viewport.read().unwrap();
        let layers = self.layers.read().unwrap();
        
        let transform = viewport.get_transform();
        let clip_rect = Rect::new(0.0, 0.0, viewport.width as f64, viewport.height as f64);
        
        let mut context = RenderContext {
            transform,
            clip_rect,
        };
        
        for layer in layers.iter() {
            if layer.visible {
                layer.render(&mut context);
            }
        }
    }
    
    pub fn resize(&self, width: u32, height: u32) {
        let mut viewport = self.viewport.write().unwrap();
        viewport.width = width;
        viewport.height = height;
        
        global_event_bus().dispatch(CanvasViewChangeEvent {
            x: viewport.center.x,
            y: viewport.center.y,
            scale: viewport.zoom,
        });
    }
}
