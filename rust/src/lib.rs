use godot::prelude::*;

mod player;
mod map;
mod gameplay;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}