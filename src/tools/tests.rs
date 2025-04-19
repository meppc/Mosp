#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::{Canvas, Point, Transform, Layer};
    use crate::tools::common::{Tool, ToolType, ToolContext, ToolEvent};
    use crate::tools::{BrushTool, ShapeTool, ShapeType, ArrowTool, EraserTool, TextTool, SelectionTool};
    
    fn create_test_canvas() -> Canvas {
        let canvas = Canvas::new(
            "test".to_string(),
            "Test Canvas".to_string(),
            800,
            600
        );
        
        let layer = Layer::new("layer1".to_string(), "Layer 1".to_string());
        canvas.add_layer(layer).unwrap();
        canvas.set_active_layer("layer1").unwrap();
        
        canvas
    }
    
    #[test]
    fn test_brush_tool() {
        let canvas = create_test_canvas();
        let mut brush = BrushTool::new();
        
        assert_eq!(brush.tool_type(), ToolType::Brush);
        assert!(brush.create_element().is_none());
        
        let context = ToolContext {
            canvas: &canvas,
            position: Point::new(100.0, 100.0),
            pressure: 1.0,
            transform: Transform::identity(),
        };
        
        brush.handle_event(
            ToolEvent::Down { position: Point::new(100.0, 100.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        brush.handle_event(
            ToolEvent::Move { position: Point::new(150.0, 150.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        brush.handle_event(
            ToolEvent::Up { position: Point::new(200.0, 200.0) },
            context.clone()
        ).unwrap();
        
        let element = brush.create_element();
        assert!(element.is_some());
        
        brush.reset();
        assert!(brush.create_element().is_none());
    }
    
    #[test]
    fn test_shape_tool() {
        let canvas = create_test_canvas();
        let mut shape = ShapeTool::new(ShapeType::Rectangle);
        
        assert_eq!(shape.tool_type(), ToolType::Shape);
        assert!(shape.create_element().is_none());
        
        let context = ToolContext {
            canvas: &canvas,
            position: Point::new(100.0, 100.0),
            pressure: 1.0,
            transform: Transform::identity(),
        };
        
        shape.handle_event(
            ToolEvent::Down { position: Point::new(100.0, 100.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        shape.handle_event(
            ToolEvent::Move { position: Point::new(200.0, 150.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        shape.handle_event(
            ToolEvent::Up { position: Point::new(200.0, 150.0) },
            context.clone()
        ).unwrap();
        
        let element = shape.create_element();
        assert!(element.is_some());
        
        shape.reset();
        assert!(shape.create_element().is_none());
    }
    
    #[test]
    fn test_arrow_tool() {
        let canvas = create_test_canvas();
        let mut arrow = ArrowTool::new();
        
        assert_eq!(arrow.tool_type(), ToolType::Arrow);
        assert!(arrow.create_element().is_none());
        
        let context = ToolContext {
            canvas: &canvas,
            position: Point::new(100.0, 100.0),
            pressure: 1.0,
            transform: Transform::identity(),
        };
        
        arrow.handle_event(
            ToolEvent::Down { position: Point::new(100.0, 100.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        arrow.handle_event(
            ToolEvent::Move { position: Point::new(200.0, 200.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        arrow.handle_event(
            ToolEvent::Up { position: Point::new(200.0, 200.0) },
            context.clone()
        ).unwrap();
        
        let element = arrow.create_element();
        assert!(element.is_some());
        
        arrow.reset();
        assert!(arrow.create_element().is_none());
    }
    
    #[test]
    fn test_eraser_tool() {
        let canvas = create_test_canvas();
        let mut eraser = EraserTool::new();
        
        assert_eq!(eraser.tool_type(), ToolType::Eraser);
        assert!(eraser.create_element().is_none());
        
        let context = ToolContext {
            canvas: &canvas,
            position: Point::new(100.0, 100.0),
            pressure: 1.0,
            transform: Transform::identity(),
        };
        
        eraser.handle_event(
            ToolEvent::Down { position: Point::new(100.0, 100.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        eraser.handle_event(
            ToolEvent::Move { position: Point::new(150.0, 150.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        eraser.handle_event(
            ToolEvent::Up { position: Point::new(200.0, 200.0) },
            context.clone()
        ).unwrap();
        
        assert!(eraser.create_element().is_none());
        
        eraser.reset();
        assert!(eraser.create_element().is_none());
    }
    
    #[test]
    fn test_text_tool() {
        let canvas = create_test_canvas();
        let mut text = TextTool::new();
        
        assert_eq!(text.tool_type(), ToolType::Text);
        assert!(text.create_element().is_none());
        
        let context = ToolContext {
            canvas: &canvas,
            position: Point::new(100.0, 100.0),
            pressure: 1.0,
            transform: Transform::identity(),
        };
        
        text.handle_event(
            ToolEvent::Down { position: Point::new(100.0, 100.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        let element = text.create_element();
        assert!(element.is_some());
        
        text.reset();
        assert!(text.create_element().is_none());
    }
    
    #[test]
    fn test_selection_tool() {
        let canvas = create_test_canvas();
        let mut selection = SelectionTool::new();
        
        assert_eq!(selection.tool_type(), ToolType::Selection);
        assert!(selection.create_element().is_none());
        
        let context = ToolContext {
            canvas: &canvas,
            position: Point::new(100.0, 100.0),
            pressure: 1.0,
            transform: Transform::identity(),
        };
        
        selection.handle_event(
            ToolEvent::Down { position: Point::new(100.0, 100.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        selection.handle_event(
            ToolEvent::Move { position: Point::new(200.0, 200.0), pressure: 1.0 },
            context.clone()
        ).unwrap();
        
        selection.handle_event(
            ToolEvent::Up { position: Point::new(200.0, 200.0) },
            context.clone()
        ).unwrap();
        
        assert!(selection.create_element().is_none());
        
        selection.reset();
        assert!(selection.create_element().is_none());
    }
}
