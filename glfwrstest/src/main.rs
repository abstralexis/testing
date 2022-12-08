/*
 *  This is the example code from the glfw-rs repo.
 *  I will comment this and alter it to figure it
 *  out. So far it is looking most promising out of
 *  the graphics libraries I have looked at for 
 *  making an engine.
 */
extern crate glfw;                  

use glfw::{Action, Context, Key};

fn main() {
    // Get a glfw instance by init
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    // Fail on errors and unwrap to panic on fail

    // Create a glfw window, 300x300, windowed, and panic on fail
    let (mut window, events) = 
        glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window poll inputs
    window.set_key_polling(true);
    // Make it the current window
    window.make_current();

    // While the window is not supposed to close
    while !window.should_close() {
        // Poll events
        glfw.poll_events();
        // Flush the events from the window event queue to check
        for (_, event) in glfw::flush_messages(&events) {
            // Handle the window event for the window
            handle_window_event(&mut window, event);
        }
    }
}

// Function for handling the window event
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    // Takes in a mut borrow to a glfw window and a glfw windowevent
    match event {
        // If the window event is the escape key, set the window to close
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        // Else, return nothing
        _ => {}
    }
}