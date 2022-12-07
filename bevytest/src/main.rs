use bevy::{
    prelude::*,
    window::*,
};

fn main() {
    /*
     *  This is the main function. It creates a Bevy App
     *  object and adds the default plugins and a custom
     *  plugin that we made. It then runs. This has a 
     *  blank window and spams "Hello world!" to the
     *  console.
     * 
     *  The App object has a scheduler that the plugins and
     *  systems are added to and, as much as they can, they
     *  run in parallel.
     */
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Example Title".to_string(),
                width: 500.,
                height: 300.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(HelloPlugin)
        .run();
}

fn hello_world() {
    /*
     *  This is an example of a system for the app
     */

     println!("Hello world!");
}

/*
 *  This is an example of a Plugin for Bevy.
 */
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    /*
     *  This makes our custom plugin act like a Plugin
     *  that works with Bevy.
     */
    fn build(&self, app: &mut App) {
        /*
         *  This adds a system for our app with our 
         *  hello_world function. This works by passing
         *  This plugin in the main function into the 
         *  main app, hence the App pointer gets satisfied
         *  and the function is added to the scheduler.
         *  However, this gets called every frame so there
         *  is "Hello world!" spam.
         */
        app.add_system(hello_world);
    }
}