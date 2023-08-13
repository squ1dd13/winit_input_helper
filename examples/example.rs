use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    keyboard::Key,
};
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        // Pass every event to the WindowInputHelper.
        // It will return true when the last event has been processed and it is time to run your application logic.
        if input.update(&event) {
            // query keypresses this update
            if input.key_pressed_os(Key::Character("A".into())) {
                println!("The 'A' key was pressed on the keyboard (OS repeating)");
            }

            if input.key_pressed(Key::Character("A".into())) {
                println!("The 'A' key was pressed on the keyboard");
            }

            if input.key_released(Key::Character("Q".into()))
                || input.close_requested()
                || input.destroyed()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed_scancode(KeyCode::KeyW) {
                println!("The 'W' key was pressed on the keyboard (scan code)");
            }

            if input.key_pressed_os_scancode(KeyCode::KeyE) {
                println!("The 'E' key was pressed on the keyboard (scan code, Os Repeating)");
            }

            if input.key_held_scancode(KeyCode::KeyR) {
                println!("The 'R' key is held (scan code)");
            }

            // query the change in mouse this update
            let mouse_diff = input.mouse_diff();
            if mouse_diff != (0.0, 0.0) {
                println!("The mouse diff is: {:?}", mouse_diff);
                println!("The mouse position is: {:?}", input.mouse());
            }

            let scroll_diff = input.scroll_diff();
            if scroll_diff != (0.0, 0.0) {
                println!("The scroll diff is: {:?}", scroll_diff);
            }

            // You are expected to control your own timing within this block.
            // Usually via rendering with vsync.
            // render();
        }
    });
}
