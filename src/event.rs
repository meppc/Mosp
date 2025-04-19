use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Failed to register event handler")]
    HandlerRegistrationFailed,
    #[error("Failed to dispatch event")]
    EventDispatchFailed,
}

/// 表示可以发送到事件总线的事件
pub trait Event: Any + Send + Sync {
    /// 返回事件的名称
    fn name(&self) -> &'static str;
    
    /// 将事件转换为特定类型
    fn as_any(&self) -> &dyn Any;
}

/// 处理特定类型事件的处理程序
pub trait EventHandler: Send + Sync {
    /// 处理接收到的事件
    fn handle_event(&self, event: &dyn Event) -> Result<(), EventError>;
    
    /// 返回此处理程序可以处理的事件类型
    fn event_types(&self) -> Vec<TypeId>;
}

/// 管理事件分发的事件总线
pub struct EventBus {
    handlers: Mutex<HashMap<TypeId, Vec<Arc<dyn EventHandler>>>>,
}

impl EventBus {
    /// 创建新的事件总线
    pub fn new() -> Self {
        Self {
            handlers: Mutex::new(HashMap::new()),
        }
    }
    
    /// 注册事件处理程序
    pub fn register_handler(&self, handler: Arc<dyn EventHandler>) -> Result<(), EventError> {
        let mut handlers = self.handlers.lock().map_err(|_| EventError::HandlerRegistrationFailed)?;
        
        for event_type in handler.event_types() {
            handlers
                .entry(event_type)
                .or_insert_with(Vec::new)
                .push(handler.clone());
        }
        
        Ok(())
    }
    
    /// 分发事件到所有注册的处理程序
    pub fn dispatch<E: Event>(&self, event: E) -> Result<(), EventError> {
        let handlers = self.handlers.lock().map_err(|_| EventError::EventDispatchFailed)?;
        
        let event_type = TypeId::of::<E>();
        if let Some(type_handlers) = handlers.get(&event_type) {
            debug!("Dispatching event: {}", event.name());
            
            for handler in type_handlers {
                if let Err(e) = handler.handle_event(&event) {
                    error!("Failed to handle event: {}", e);
                }
            }
        }
        
        Ok(())
    }
}

/// 获取全局事件总线实例
pub fn global_event_bus() -> &'static EventBus {
    static INSTANCE: once_cell::sync::Lazy<EventBus> = once_cell::sync::Lazy::new(EventBus::new);
    &INSTANCE
}

pub mod events {
    use super::Event;
    
    /// 应用程序启动事件
    #[derive(Debug)]
    pub struct AppStartupEvent;
    
    impl Event for AppStartupEvent {
        fn name(&self) -> &'static str {
            "AppStartupEvent"
        }
        
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }
    
    /// 应用程序关闭事件
    #[derive(Debug)]
    pub struct AppShutdownEvent;
    
    impl Event for AppShutdownEvent {
        fn name(&self) -> &'static str {
            "AppShutdownEvent"
        }
        
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }
    
    /// 窗口大小改变事件
    #[derive(Debug)]
    pub struct WindowResizeEvent {
        pub width: u32,
        pub height: u32,
    }
    
    impl Event for WindowResizeEvent {
        fn name(&self) -> &'static str {
            "WindowResizeEvent"
        }
        
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }
    
    /// 画布视图改变事件
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
        
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }
}

/// 为事件处理程序实现宏
#[macro_export]
macro_rules! impl_event_handler {
    ($handler:ty, [$($event_type:ty),*]) => {
        impl EventHandler for $handler {
            fn handle_event(&self, event: &dyn Event) -> Result<(), EventError> {
                $(
                    if let Some(e) = event.as_any().downcast_ref::<$event_type>() {
                        self.handle(e)?;
                    }
                )*
                Ok(())
            }
            
            fn event_types(&self) -> Vec<TypeId> {
                vec![$(TypeId::of::<$event_type>()),*]
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct TestEventHandler {
        counter: AtomicUsize,
    }

    impl TestEventHandler {
        fn new() -> Self {
            Self {
                counter: AtomicUsize::new(0),
            }
        }

        fn count(&self) -> usize {
            self.counter.load(Ordering::SeqCst)
        }
    }

    impl EventHandler for TestEventHandler {
        fn handle_event(&self, event: &dyn Event) -> Result<(), EventError> {
            if event.as_any().downcast_ref::<events::AppStartupEvent>().is_some() {
                self.counter.fetch_add(1, Ordering::SeqCst);
            }
            Ok(())
        }

        fn event_types(&self) -> Vec<TypeId> {
            vec![TypeId::of::<events::AppStartupEvent>()]
        }
    }

    #[test]
    fn test_event_registration() {
        let event_bus = EventBus::new();
        let handler = Arc::new(TestEventHandler::new());
        
        assert!(event_bus.register_handler(handler.clone()).is_ok());
    }

    #[test]
    fn test_event_dispatch() {
        let event_bus = EventBus::new();
        let handler = Arc::new(TestEventHandler::new());
        
        event_bus.register_handler(handler.clone()).unwrap();
        event_bus.dispatch(events::AppStartupEvent).unwrap();
        
        assert_eq!(handler.count(), 1);
    }

    #[test]
    fn test_multiple_handlers() {
        let event_bus = EventBus::new();
        let handler1 = Arc::new(TestEventHandler::new());
        let handler2 = Arc::new(TestEventHandler::new());
        
        event_bus.register_handler(handler1.clone()).unwrap();
        event_bus.register_handler(handler2.clone()).unwrap();
        
        event_bus.dispatch(events::AppStartupEvent).unwrap();
        
        assert_eq!(handler1.count(), 1);
        assert_eq!(handler2.count(), 1);
    }

    #[test]
    fn test_global_event_bus() {
        let handler = Arc::new(TestEventHandler::new());
        let event_bus = global_event_bus();
        
        event_bus.register_handler(handler.clone()).unwrap();
        event_bus.dispatch(events::AppStartupEvent).unwrap();
        
        assert_eq!(handler.count(), 1);
    }
}
