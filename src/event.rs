
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::debug;

pub trait Event: Any + Send + Sync {
    fn name(&self) -> &'static str;
}

pub trait EventHandler: Send + Sync {
    fn handle_event(&self, event: &dyn Event);
    
    fn event_types(&self) -> Vec<TypeId>;
}

pub struct EventBus {
    handlers: Mutex<HashMap<TypeId, Vec<Arc<dyn EventHandler>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: Mutex::new(HashMap::new()),
        }
    }
    
    pub fn register_handler(&self, handler: Arc<dyn EventHandler>) {
        let mut handlers = self.handlers.lock().unwrap();
        
        for event_type in handler.event_types() {
            handlers
                .entry(event_type)
                .or_insert_with(Vec::new)
                .push(handler.clone());
        }
    }
    
    pub fn dispatch<E: Event>(&self, event: E) {
        let handlers = self.handlers.lock().unwrap();
        
        let event_type = TypeId::of::<E>();
        if let Some(type_handlers) = handlers.get(&event_type) {
            debug!("Dispatching event: {}", event.name());
            
            for handler in type_handlers {
                handler.handle_event(&event);
            }
        }
    }
}

pub fn global_event_bus() -> &'static EventBus {
    static INSTANCE: once_cell::sync::Lazy<EventBus> = once_cell::sync::Lazy::new(EventBus::new);
    &INSTANCE
}

pub mod events {
    use super::Event;
    use std::any::TypeId;
    
    #[derive(Debug)]
    pub struct AppStartupEvent;
    
    impl Event for AppStartupEvent {
        fn name(&self) -> &'static str {
            "AppStartupEvent"
        }
    }
    
    #[derive(Debug)]
    pub struct AppShutdownEvent;
    
    impl Event for AppShutdownEvent {
        fn name(&self) -> &'static str {
            "AppShutdownEvent"
        }
    }
    
    #[derive(Debug)]
    pub struct WindowResizeEvent {
        pub width: u32,
        pub height: u32,
    }
    
    impl Event for WindowResizeEvent {
        fn name(&self) -> &'static str {
            "WindowResizeEvent"
        }
    }
    
    #[derive(Debug)]
    pub struct CanvasViewChangeEvent {
        pub x: f64,
        pub y: f64,
        pub scale: f64,
    }
    
    impl Event for CanvasViewChangeEvent {
        fn name(&self) -> &'static str {
            "CanvasViewChangeEvent"
        }
    }
}

#[macro_export]
macro_rules! impl_event_handler {
    ($handler:ty, [$($event_type:ty),*]) => {
        impl EventHandler for $handler {
            fn handle_event(&self, event: &dyn Event) {
                $(
                    if let Some(e) = event.downcast_ref::<$event_type>() {
                        self.handle(e);
                    }
                )*
            }
            
            fn event_types(&self) -> Vec<TypeId> {
                vec![$(TypeId::of::<$event_type>()),*]
            }
        }
    };
}
