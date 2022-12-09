/*
 *  This is the example code from the glfw-rs repo.
 *  I will comment this and alter it to figure it
 *  out. So far it is looking most promising out of
 *  the graphics libraries I have looked at for 
 *  making an engine.
 */

extern crate glfw;

use glfw::{Action, Context, Key};

// Make it more object oriented
fn main() {
    let mut app: Main = Main::new();
    app.init();
}

// Put the things in a struct so it can handle variables in Self
struct Main {
    some_x: u32,
    some_y: u32,
}
impl Main {
    fn new() -> Self {
        Main {
            some_x: 50,
            some_y: 50,
        }
    }

    fn init(&mut self) {
        // Get a glfw instance by init
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        // Fail on errors and unwrap to panic on fail
    
        // Create a glfw window, 300x300, windowed, and panic on fail
        let (mut window, events) = 
            glfw.create_window(300, 300, "Main window", glfw::WindowMode::Windowed)
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
                self.handle_window_event(&mut window, event);
            }
        }
    }
    
    // Function for handling the window event
    fn handle_window_event(&mut self, window: &mut glfw::Window, event: glfw::WindowEvent) {
        // Takes in a mut borrow to a glfw window and a glfw windowevent
        match event {
            // If the window event is the escape key, set the window to close
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
    
            /* -----------------------------------------------------------------
             *  These are some basic input testing cases for modifying values in
             *  self. Eventually this could be used with entities etc.
             */
            glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                self.some_x -= 10;
            }

            glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                self.some_x += 10;
            }

            glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                println!("self.somex: {:?}", self.some_x);
                println!("self.somey: {:?}", self.some_y);
            }
            // -----------------------------------------------------------------
    
            // Else, return nothing
            _ => {}
        }
    }
}