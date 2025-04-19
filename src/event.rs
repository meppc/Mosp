
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventType(TypeId);

impl EventType {
    pub fn of<T: 'static>() -> Self {
        Self(TypeId::of::<T>())
    }
}

pub type EventHandler = Box<dyn Fn(&dyn Any) + Send + Sync>;

#[derive(Default)]
pub struct EventBus {
    handlers: Mutex<HashMap<EventType, Vec<EventHandler>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: Mutex::new(HashMap::new()),
        }
    }
    
    pub fn register<T: 'static>(&self, handler: impl Fn(&T) + Send + Sync + 'static) {
        let event_type = EventType::of::<T>();
        let mut handlers = self.handlers.lock().unwrap();
        
        let handler_wrapper: EventHandler = Box::new(move |event| {
            if let Some(typed_event) = event.downcast_ref::<T>() {
                handler(typed_event);
            }
        });
        
        handlers.entry(event_type).or_default().push(handler_wrapper);
    }
    
    pub fn dispatch<T: 'static>(&self, event: T) {
        let event_type = EventType::of::<T>();
        let handlers = self.handlers.lock().unwrap();
        
        if let Some(handlers) = handlers.get(&event_type) {
            for handler in handlers {
                handler(&event);
            }
        }
    }
}

pub static EVENT_BUS: Lazy<Arc<EventBus>> = Lazy::new(|| {
    Arc::new(EventBus::new())
});

use once_cell::sync::Lazy;
