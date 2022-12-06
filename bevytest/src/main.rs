use bevy::prelude::*;

fn main() {
    App::new()
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    /*
     *  This is an example of a system for the app
     */

     println!("Hello world!");
}