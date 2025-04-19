use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl serde::Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Point", 2)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct PointHelper {
            x: f64,
            y: f64,
        }
        
        let helper = PointHelper::deserialize(deserializer)?;
        Ok(Point {
            x: helper.x,
            y: helper.y,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct RenderContext {
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub a: f64, pub b: f64,
    pub c: f64, pub d: f64,
    pub e: f64, pub f: f64,
}

pub trait CanvasElement: Send + Sync + Debug {
    fn render(&self, context: &mut RenderContext);
    
    fn contains(&self, point: Point) -> bool;
    
    fn bounds(&self) -> Rect;
    
    fn element_type(&self) -> &'static str;
    
    fn id(&self) -> &str;
    
    fn clone_element(&self) -> Box<dyn CanvasElement>;
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
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
        !(self.x > other.x + other.width ||
          self.x + self.width < other.x ||
          self.y > other.y + other.height ||
          self.y + self.height < other.y)
    }
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            a: 1.0, b: 0.0,
            c: 0.0, d: 1.0,
            e: 0.0, f: 0.0,
        }
    }
    
    pub fn translate(tx: f64, ty: f64) -> Self {
        Self {
            a: 1.0, b: 0.0,
            c: 0.0, d: 1.0,
            e: tx,  f: ty,
        }
    }
    
    pub fn scale(sx: f64, sy: f64) -> Self {
        Self {
            a: sx,  b: 0.0,
            c: 0.0, d: sy,
            e: 0.0, f: 0.0,
        }
    }
    
    pub fn apply(&self, point: &Point) -> Point {
        Point {
            x: self.a * point.x + self.c * point.y + self.e,
            y: self.b * point.x + self.d * point.y + self.f,
        }
    }
    
    pub fn inverse(&self) -> Option<Self> {
        let det = self.a * self.d - self.b * self.c;
        
        if det.abs() < 1e-6 {
            return None;
        }
        
        let inv_det = 1.0 / det;
        
        Some(Self {
            a: self.d * inv_det,
            b: -self.b * inv_det,
            c: -self.c * inv_det,
            d: self.a * inv_det,
            e: (self.c * self.f - self.d * self.e) * inv_det,
            f: (self.b * self.e - self.a * self.f) * inv_det,
        })
    }
    
    pub fn compose(&self, other: &Transform) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            e: self.e * other.a + self.f * other.c + other.e,
            f: self.e * other.b + self.f * other.d + other.f,
        }
    }
}

#[derive(Debug)]
pub struct Layer {
    id: String,
    name: String,
    visible: bool,
    locked: bool,
    opacity: f64,
    elements: Vec<Arc<Box<dyn CanvasElement>>>,
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
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn is_locked(&self) -> bool {
        self.locked
    }
    
    pub fn set_locked(&mut self, locked: bool) {
        self.locked = locked;
    }
    
    pub fn opacity(&self) -> f64 {
        self.opacity
    }
    
    pub fn set_opacity(&mut self, opacity: f64) {
        self.opacity = opacity.max(0.0).min(1.0);
    }
    
    pub fn add_element(&mut self, element: Arc<Box<dyn CanvasElement>>) {
        self.elements.push(element);
    }
    
    pub fn elements(&self) -> &[Arc<Box<dyn CanvasElement>>] {
        &self.elements
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

#[derive(Debug)]
pub struct Canvas {
    id: String,
    name: String,
    width: u32,
    height: u32,
    layers: RwLock<Vec<Arc<RwLock<Layer>>>>,
    layer_map: RwLock<HashMap<String, usize>>,
    active_layer_id: RwLock<Option<String>>,
    transform: RwLock<Transform>,
}

impl Canvas {
    pub fn new(id: String, name: String, width: u32, height: u32) -> Self {
        Self {
            id,
            name,
            width,
            height,
            layers: RwLock::new(Vec::new()),
            layer_map: RwLock::new(HashMap::new()),
            active_layer_id: RwLock::new(None),
            transform: RwLock::new(Transform::identity()),
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }
    
    pub fn add_layer(&self, layer: Layer) -> Result<()> {
        let layer_id = layer.id().to_string();
        
        let mut layers = self.layers.write().unwrap();
        let mut layer_map = self.layer_map.write().unwrap();
        
        if layer_map.contains_key(&layer_id) {
            return Err(Error::Canvas(format!("Layer with ID {} already exists", layer_id)));
        }
        
        let index = layers.len();
        layers.push(Arc::new(RwLock::new(layer)));
        layer_map.insert(layer_id.clone(), index);
        
        if layers.len() == 1 {
            let mut active_id = self.active_layer_id.write().unwrap();
            *active_id = Some(layer_id);
        }
        
        Ok(())
    }
    
    pub fn get_layer(&self, id: &str) -> Option<Arc<RwLock<Layer>>> {
        let layer_map = self.layer_map.read().unwrap();
        let layers = self.layers.read().unwrap();
        
        layer_map.get(id).map(|&index| layers[index].clone())
    }
    
    pub fn active_layer(&self) -> Option<Arc<RwLock<Layer>>> {
        let active_id = self.active_layer_id.read().unwrap();
        
        if let Some(id) = &*active_id {
            self.get_layer(id)
        } else {
            let layers = self.layers.read().unwrap();
            if !layers.is_empty() {
                Some(layers[0].clone())
            } else {
                None
            }
        }
    }
    
    pub fn set_active_layer(&self, id: &str) -> Result<()> {
        let mut active_id = self.active_layer_id.write().unwrap();
        
        if self.get_layer(id).is_some() {
            *active_id = Some(id.to_string());
            Ok(())
        } else {
            Err(Error::Canvas(format!("Layer with ID {} not found", id)))
        }
    }
    
    pub fn transform(&self) -> Transform {
        *self.transform.read().unwrap()
    }
    
    pub fn set_transform(&self, transform: Transform) {
        let mut t = self.transform.write().unwrap();
        *t = transform;
    }
    
    pub fn render(&self, context: &mut RenderContext) {
        let layers = self.layers.read().unwrap();
        
        for layer in layers.iter() {
            let layer = layer.read().unwrap();
            layer.render(context);
        }
    }
    
    pub fn add_element(&self, element: Box<dyn CanvasElement>) -> Result<()> {
        let active_layer = self.active_layer();
        
        if let Some(layer) = active_layer {
            let mut layer = layer.write().unwrap();
            layer.add_element(Arc::new(element));
            Ok(())
        } else {
            Err(Error::Canvas("No active layer available".to_string()))
        }
    }
}
