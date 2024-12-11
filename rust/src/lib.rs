use godot::prelude::*;

mod player;
mod map;
mod gameplay;
mod objectives;
mod items;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}