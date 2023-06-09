use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::event::*;


fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    event_loop.run(
        move |event, _, control_flow| match event {
            Event::WindowEvent {
                window_id,
                event
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape), ..
                    }, ..
                } => {
                    *control_flow = ControlFlow::Exit;
                },

                _ => {  }
            },

            _ => {  }
        }
    );
}
