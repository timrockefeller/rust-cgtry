use winit::{
    ControlFlow, Event, EventsLoop,dpi::LogicalSize, VirtualKeyCode, Window, WindowBuilder, WindowEvent,
};

pub struct WindowState {
    pub evloop: EventsLoop,
    pub window: Window,
}
impl WindowState {
    pub fn new<T: Into<String>>(title: T, size: (f64, f64)) -> Self {
        let evloop = EventsLoop::new();
        let window = match WindowBuilder::new()
            .with_title(title)
            .with_dimensions(LogicalSize::new(size.0, size.1))
            .build(&evloop)
        {
            Ok(x) => x,
            Err(e) => {
                println!("Window create failed! {:?}", e);
                panic!()
            }
        };
        Self { evloop, window }
    }
}

pub fn event_handler(event: Event) -> winit::ControlFlow {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    ControlFlow::Break
                } else {
                    ControlFlow::Continue
                }
            }
            WindowEvent::CloseRequested => ControlFlow::Break,
            _ => ControlFlow::Continue,
        },
        _ => ControlFlow::Continue,
    }
}
