use godot::prelude::*;

mod player;
mod map;
mod gameplay;
mod objectives;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}